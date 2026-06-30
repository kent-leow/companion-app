pub mod context;

pub use context::SessionContext;

use std::collections::HashMap;
use std::path::{Path, PathBuf};
use uuid::Uuid;

pub struct SessionManager {
    sessions_dir: PathBuf,
    active: HashMap<String, SessionContext>,
}

impl SessionManager {
    pub fn new(sessions_dir: PathBuf) -> Self {
        Self {
            sessions_dir,
            active: HashMap::new(),
        }
    }

    pub fn create_session(&mut self, id: Option<String>) -> &SessionContext {
        let session_id = id.unwrap_or_else(|| Uuid::new_v4().to_string());
        let session_dir = self.sessions_dir.join(&session_id);
        let ctx = SessionContext::new(session_id.clone(), session_dir);
        self.active.insert(session_id.clone(), ctx);
        self.active.get(&session_id).unwrap()
    }

    pub fn get_session(&self, id: &str) -> Option<&SessionContext> {
        self.active.get(id)
    }

    pub fn get_session_mut(&mut self, id: &str) -> Option<&mut SessionContext> {
        self.active.get_mut(id)
    }

    pub fn list_sessions(&self) -> Vec<&str> {
        self.active.keys().map(|s| s.as_str()).collect()
    }
}
