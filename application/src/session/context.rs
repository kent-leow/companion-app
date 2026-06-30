use std::path::PathBuf;

use crate::llm::ChatMessage;

pub struct SessionContext {
    pub id: String,
    pub dir: PathBuf,
    pub history: Vec<ChatMessage>,
}

impl SessionContext {
    pub fn new(id: String, dir: PathBuf) -> Self {
        Self {
            id,
            dir,
            history: Vec::new(),
        }
    }

    pub fn add_message(&mut self, msg: ChatMessage) {
        self.history.push(msg);
    }

    pub fn memory_path(&self) -> PathBuf {
        self.dir.join("memory.md")
    }

    pub fn history_path(&self) -> PathBuf {
        self.dir.join("history.json")
    }
}
