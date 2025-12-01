use std::{
    collections::{HashMap, VecDeque},
    sync::{Arc, Mutex, RwLock},
};

use crate::ui_session_interface::InvokeUiSession;

// src/ui_backend.rs
pub trait UiBackend: Send + Sync {
    type Event: Send + 'static; // Events pushed to the UI backend
    type SessionHandler: InvokeUiSession;
    
    fn create_session_handler(&self, session_id: String) -> Arc<Self::SessionHandler>;
    fn push_event(&self, app_type: &str, event: Self::Event) -> bool;
    fn start_event_loop(&self) -> Result<(), Box<dyn std::error::Error>>;
    fn shutdown(&self);
}
