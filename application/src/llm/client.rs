use anyhow::{Context, Result};
use futures_util::StreamExt;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use tokio::sync::mpsc;

use crate::config::EnvConfig;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum Role {
    System,
    User,
    Assistant,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatMessage {
    pub role: Role,
    pub content: serde_json::Value,
}

impl ChatMessage {
    pub fn system(content: &str) -> Self {
        Self {
            role: Role::System,
            content: serde_json::Value::String(content.to_string()),
        }
    }

    pub fn user(content: &str) -> Self {
        Self {
            role: Role::User,
            content: serde_json::Value::String(content.to_string()),
        }
    }

    pub fn assistant(content: &str) -> Self {
        Self {
            role: Role::Assistant,
            content: serde_json::Value::String(content.to_string()),
        }
    }
}

#[derive(Debug, Clone)]
pub enum StreamEvent {
    Token(String),
    Done(String),
    Error(String),
}

#[derive(Serialize)]
struct ChatRequest {
    model: String,
    messages: Vec<ChatMessage>,
    stream: bool,
    max_tokens: u32,
}

#[derive(Deserialize)]
struct ChatChunk {
    choices: Option<Vec<ChunkChoice>>,
}

#[derive(Deserialize)]
struct ChunkChoice {
    delta: Option<Delta>,
    finish_reason: Option<String>,
}

#[derive(Deserialize)]
struct Delta {
    content: Option<String>,
}

#[derive(Clone)]
pub struct LlmClient {
    http: Client,
    base_url: String,
    api_key: String,
}

impl LlmClient {
    pub fn new(config: &EnvConfig) -> Result<Self> {
        let mut builder = Client::builder();

        let cert_path = std::env::var("SSL_CERT_FILE").ok().or_else(|| {
            let home = std::env::var("HOME").ok()?;
            let path = format!("{}/.config/.cloudflare/combined-ca.pem", home);
            if std::path::Path::new(&path).exists() { Some(path) } else { None }
        });

        if let Some(path) = &cert_path {
            if let Ok(pem) = std::fs::read(path) {
                if let Ok(certs) = reqwest::Certificate::from_pem_bundle(&pem) {
                    for cert in certs {
                        builder = builder.add_root_certificate(cert);
                    }
                }
            }
        }

        builder = builder
            .danger_accept_invalid_certs(true)
            .user_agent("companion/0.1.0");

        let http = builder
            .build()
            .context("failed to create HTTP client")?;

        Ok(Self {
            http,
            base_url: config.base_url.clone(),
            api_key: config.auth_token.clone(),
        })
    }

    pub async fn chat_stream(
        &self,
        model: &str,
        messages: &[ChatMessage],
        max_tokens: u32,
    ) -> Result<mpsc::Receiver<StreamEvent>> {
        let url = format!("{}/v1/chat/completions", self.base_url);
        #[cfg(debug_assertions)]
        eprintln!("[debug] POST {} model={}", url, model);

        let request = ChatRequest {
            model: model.to_string(),
            messages: messages.to_vec(),
            stream: true,
            max_tokens,
        };

        let response = self
            .http
            .post(&url)
            .header("x-api-key", &self.api_key)
            .header("Content-Type", "application/json")
            .json(&request)
            .send()
            .await
            .context("failed to send request to LLM gateway")?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            anyhow::bail!("LLM API error {}: {}", status, body);
        }

        let (tx, rx) = mpsc::channel(128);

        tokio::spawn(async move {
            let mut full_response = String::new();
            let mut stream = response.bytes_stream();

            let mut buffer = String::new();

            while let Some(chunk) = stream.next().await {
                let chunk = match chunk {
                    Ok(c) => c,
                    Err(e) => {
                        let _ = tx.send(StreamEvent::Error(e.to_string())).await;
                        return;
                    }
                };

                buffer.push_str(&String::from_utf8_lossy(&chunk));

                while let Some(line_end) = buffer.find('\n') {
                    let line = buffer[..line_end].trim().to_string();
                    buffer = buffer[line_end + 1..].to_string();

                    if line.is_empty() || line == "data: [DONE]" {
                        continue;
                    }

                    if let Some(data) = line.strip_prefix("data: ") {
                        if let Ok(chunk) = serde_json::from_str::<ChatChunk>(data) {
                            if let Some(choices) = chunk.choices {
                                for choice in choices {
                                    if let Some(delta) = choice.delta {
                                        if let Some(content) = delta.content {
                                            full_response.push_str(&content);
                                            let _ =
                                                tx.send(StreamEvent::Token(content)).await;
                                        }
                                    }
                                    if choice.finish_reason.is_some() {
                                        let _ = tx
                                            .send(StreamEvent::Done(full_response.clone()))
                                            .await;
                                        return;
                                    }
                                }
                            }
                        }
                    }
                }
            }

            if !full_response.is_empty() {
                let _ = tx.send(StreamEvent::Done(full_response)).await;
            }
        });

        Ok(rx)
    }

    pub async fn chat(
        &self,
        model: &str,
        messages: &[ChatMessage],
        max_tokens: u32,
    ) -> Result<String> {
        let mut rx = self.chat_stream(model, messages, max_tokens).await?;
        let mut result = String::new();

        while let Some(event) = rx.recv().await {
            match event {
                StreamEvent::Done(text) => return Ok(text),
                StreamEvent::Token(t) => result.push_str(&t),
                StreamEvent::Error(e) => anyhow::bail!("stream error: {}", e),
            }
        }

        Ok(result)
    }
}
