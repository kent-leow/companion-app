use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyModifiers};
use std::time::Duration;

#[derive(Debug, Clone)]
pub struct ParsedInput {
    pub text: String,
    pub tags: Vec<TagRef>,
    pub has_image: bool,
}

#[derive(Debug, Clone)]
pub struct TagRef {
    pub path: String,
    pub is_dir: bool,
}

pub enum InputEvent {
    Message(ParsedInput),
    Exit,
    Empty,
}

pub struct InputHandler;

impl InputHandler {
    pub fn read_line(prompt: &str) -> InputEvent {
        use std::io::{self, Write};

        print!("{}", prompt);
        let _ = io::stdout().flush();

        let mut line = String::new();
        if io::stdin().read_line(&mut line).is_err() {
            return InputEvent::Exit;
        }

        let trimmed = line.trim();
        if trimmed.is_empty() {
            return InputEvent::Empty;
        }

        if trimmed == "/exit" || trimmed == "/quit" {
            return InputEvent::Exit;
        }

        let parsed = Self::parse(trimmed);
        InputEvent::Message(parsed)
    }

    pub fn parse(input: &str) -> ParsedInput {
        let mut tags = Vec::new();
        let mut has_image = false;

        for word in input.split_whitespace() {
            if let Some(path) = word.strip_prefix('@') {
                if path.is_empty() {
                    continue;
                }
                let is_dir = path.ends_with('/');
                let is_image = path.ends_with(".png")
                    || path.ends_with(".jpg")
                    || path.ends_with(".jpeg")
                    || path.ends_with(".gif")
                    || path.ends_with(".webp");

                if is_image {
                    has_image = true;
                }

                tags.push(TagRef {
                    path: path.to_string(),
                    is_dir,
                });
            }
        }

        ParsedInput {
            text: input.to_string(),
            tags,
            has_image,
        }
    }

    pub fn poll_ctrl_c() -> bool {
        if event::poll(Duration::from_millis(0)).unwrap_or(false) {
            if let Ok(Event::Key(KeyEvent {
                code: KeyCode::Char('c'),
                modifiers: KeyModifiers::CONTROL,
                ..
            })) = event::read()
            {
                return true;
            }
        }
        false
    }
}
