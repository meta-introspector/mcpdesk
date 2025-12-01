use std::{
    collections::{HashMap, VecDeque},
    sync::{Arc, Mutex, RwLock},
};
use crate::ui_backend::{UiBackend, InvokeUiSession};
use dioxus_desktop::{WindowBuilder, launch_cfg, Config as DioxusConfig};
use dioxus::prelude::*;
use hbb_common::log;

// Define DioxusEvent similar to the plan
#[derive(Clone, Debug)]
pub enum DioxusEvent {
    MsgBox { msgtype: String, title: String, text: String },
    NewMessage { msg: String },
    // VideoFrame { display: usize, data: Vec<u8> }, // Placeholder for video frames
    SwitchDisplay { display: crate::message_proto::SwitchDisplay }, // Use actual SwitchDisplay type
    // ... other events that need to be pushed to Dioxus UI
}

pub struct DioxusBackend {
    event_channel: Arc<Mutex<VecDeque<DioxusEvent>>>,
    sessions: Arc<RwLock<HashMap<String, Arc<DioxusHandler>>>>,
}

impl DioxusBackend {
    pub fn new() -> Self {
        Self {
            event_channel: Arc::new(Mutex::new(VecDeque::new())),
            sessions: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}

impl UiBackend for DioxusBackend {
    type Event = DioxusEvent;
    type SessionHandler = DioxusHandler;
    
    fn create_session_handler(&self, session_id: String) -> Arc<Self::SessionHandler> {
        let handler = Arc::new(DioxusHandler::new(session_id.clone(), self.event_channel.clone()));
        self.sessions.write().unwrap().insert(session_id, handler.clone());
        handler
    }
    
    fn push_event(&self, _app_type: &str, event: Self::Event) -> bool {
        self.event_channel.lock().unwrap().push_back(event);
        true
    }
    
    fn start_event_loop(&self) -> Result<(), Box<dyn std::error::Error>> {
        // This is a basic placeholder. Real Dioxus app launch involves rendering components.
        // For now, we'll just launch an empty window or a basic component.
        // The actual rendering logic will come in Phase 4.
        
        // This assumes a root component `App` will be defined later
        // and that `dioxus_desktop::launch_cfg` is the correct way to start.
        launch_cfg(component! {
            rsx! {
                // Root component for the Dioxus app
                // Will contain logic to render the main UI
                div {
                    "mcpdesk Dioxus UI - Under Construction"
                }
            }
        }, DioxusConfig::new().with_window(WindowBuilder::new().with_title("mcpdesk Dioxus"))
        );
        Ok(())
    }
    
    fn shutdown(&self) {
        // Dioxus desktop apps usually shutdown when the main window closes.
        // More explicit shutdown logic can be added here if needed.
        log::info!("Dioxus backend shutdown requested.");
    }
}

// DioxusHandler will implement InvokeUiSession, but its definition will be in a separate file.
pub struct DioxusHandler {
    session_id: String,
    event_channel: Arc<Mutex<VecDeque<DioxusEvent>>>,
}

impl DioxusHandler {
    pub fn new(session_id: String, event_channel: Arc<Mutex<VecDeque<DioxusEvent>>>) -> Self {
        Self {
            session_id,
            event_channel,
        }
    }
}
