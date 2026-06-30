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
use llm::{ChatMessage, LlmClient, ModelSelector, StreamEvent};
use orchestrator::Orchestrator;
use skills::loader::SkillRegistry;
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

    let base_dir = std::env::current_dir().unwrap_or(exe_dir);
    let core_content =
        config::load_core_md(&base_dir).unwrap_or_else(|_| String::new());

    let skills_dir = base_dir.join("skills");
    let skill_registry =
        SkillRegistry::load_from_dir(&skills_dir).unwrap_or_else(|_| {
            SkillRegistry::load_from_dir(&std::path::PathBuf::from("skills"))
                .unwrap_or_else(|_| SkillRegistry::empty())
        });

    let client = LlmClient::new(&env_config)?;
    let model = &env_config.default_model;
    let model_selector = ModelSelector::from_config(&env_config);

    let orchestrator =
        Orchestrator::new(client.clone(), model_selector, core_content.clone(), skill_registry);

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

                match orchestrator.handle(&history).await {
                    Ok(response) => {
                        OutputRenderer::print_token(&response);
                        OutputRenderer::print_done();
                        history.push(ChatMessage::assistant(&response));
                    }
                    Err(_) => {
                        // Fallback to direct streaming on orchestrator error
                        let mut rx = match client.chat_stream(model, &history, 4096).await
                        {
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
        }
    }

    Ok(())
}
