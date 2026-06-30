use anyhow::{Context, Result};
use std::path::{Path, PathBuf};

use super::MemoryPruner;

#[derive(Debug, Clone)]
pub struct MemoryEntry {
    pub content: String,
    pub importance: u8,
    pub timestamp: u64,
}

pub struct MemoryStore {
    path: PathBuf,
    entries: Vec<MemoryEntry>,
    max_tokens: usize,
}

impl MemoryStore {
    pub fn new(path: PathBuf, max_tokens: usize) -> Self {
        Self {
            path,
            entries: Vec::new(),
            max_tokens,
        }
    }

    pub fn load(&mut self) -> Result<()> {
        if !self.path.exists() {
            return Ok(());
        }

        let content = std::fs::read_to_string(&self.path)
            .with_context(|| format!("failed to read memory: {}", self.path.display()))?;

        self.entries.clear();
        for block in content.split("\n---\n") {
            let trimmed = block.trim();
            if trimmed.is_empty() {
                continue;
            }
            self.entries.push(MemoryEntry {
                content: trimmed.to_string(),
                importance: 5,
                timestamp: 0,
            });
        }

        Ok(())
    }

    pub fn save(&self) -> Result<()> {
        if let Some(parent) = self.path.parent() {
            std::fs::create_dir_all(parent)?;
        }

        let content: String = self
            .entries
            .iter()
            .map(|e| e.content.as_str())
            .collect::<Vec<_>>()
            .join("\n---\n");

        std::fs::write(&self.path, content)
            .with_context(|| format!("failed to write memory: {}", self.path.display()))
    }

    pub fn add(&mut self, content: String, importance: u8) {
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();

        self.entries.push(MemoryEntry {
            content,
            importance,
            timestamp,
        });

        self.prune();
    }

    pub fn get_context(&self) -> String {
        if self.entries.is_empty() {
            return String::new();
        }

        let mut ctx = String::from("## Memory\n");
        for entry in &self.entries {
            ctx.push_str(&entry.content);
            ctx.push('\n');
        }
        ctx
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    fn prune(&mut self) {
        MemoryPruner::prune(&mut self.entries, self.max_tokens);
    }
}
