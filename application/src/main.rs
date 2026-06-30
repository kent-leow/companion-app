#![allow(dead_code)]

mod agent;
mod config;
mod llm;
mod memory;
mod orchestrator;
mod session;
mod skills;
mod tui;

use clap::Parser;
use llm::{ChatMessage, LlmClient, StreamEvent};
use tui::{InputEvent, InputHandler, OutputRenderer};

#[derive(Parser)]
#[command(name = "companion", version, about = "AI companion agent CLI")]
struct Cli {
    #[arg(long, help = "Session ID for multi-session support")]
    session_id: Option<String>,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let _cli = Cli::parse();
    let env_config = config::EnvConfig::load()?;

    let exe_dir = std::env::current_exe()
        .ok()
        .and_then(|p| p.parent().map(|p| p.to_path_buf()))
        .unwrap_or_else(|| std::env::current_dir().unwrap());

    let core_content = config::load_core_md(&std::env::current_dir().unwrap_or(exe_dir))
        .unwrap_or_else(|_| String::new());

    let client = LlmClient::new(&env_config)?;
    let model = &env_config.default_model;

    tui::show_splash();

    let mut history: Vec<ChatMessage> = Vec::new();
    if !core_content.is_empty() {
        history.push(ChatMessage::system(&core_content));
    }

    loop {
        match InputHandler::read_line("\n> ") {
            InputEvent::Exit => {
                OutputRenderer::print_system("bye.");
                break;
            }
            InputEvent::Empty => continue,
            InputEvent::Message(parsed) => {
                history.push(ChatMessage::user(&parsed.text));

                let mut rx = match client.chat_stream(model, &history, 4096).await {
                    Ok(rx) => rx,
                    Err(e) => {
                        OutputRenderer::print_error(&e.to_string());
                        history.pop();
                        continue;
                    }
                };

                let mut full_response = String::new();
                while let Some(event) = rx.recv().await {
                    match event {
                        StreamEvent::Token(t) => {
                            OutputRenderer::print_token(&t);
                            full_response.push_str(&t);
                        }
                        StreamEvent::Done(_) => {
                            OutputRenderer::print_done();
                            break;
                        }
                        StreamEvent::Error(e) => {
                            OutputRenderer::print_error(&e);
                            break;
                        }
                    }
                }

                if !full_response.is_empty() {
                    history.push(ChatMessage::assistant(&full_response));
                }
            }
        }
    }

    Ok(())
}
