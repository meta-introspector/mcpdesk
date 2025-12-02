use std::{
    collections::{HashMap, VecDeque},
    sync::{Arc, Mutex, RwLock},
};
use crate::ui_backend::{UiBackend, self};
use hbb_common::log;

// We temporarily remove `dioxus::prelude::*;` and `crate::main_app::App;`
// as we are bypassing Dioxus launching for this test.
// We also need `tao` and `wry` imports, now correctly from `dioxus_desktop`.
use dioxus_desktop::tao::{
    event::{Event, StartCause, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};
use dioxus_desktop::wry::{WebContext, WebViewBuilder};

// Define DioxusEvent similar to the plan
#[derive(Clone, Debug)]
pub enum DioxusEvent {
    MsgBox { msgtype: String, title: String, text: String },
    NewMessage { msg: String },
    // VideoFrame { display: usize, data: Vec<u8> }, // Placeholder for video frames
    SwitchDisplay { display: hbb_common::message_proto::SwitchDisplay }, // Use actual SwitchDisplay type
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
        log::info!("dioxus_backend: Launching vanilla wry-tao test window.");

        // Code from working wry-tao example
        let event_loop = EventLoop::new();
        let window = WindowBuilder::new()
            .with_title("MCPDesk Wry-Tao Test")
            .build(&event_loop)
            .expect("Failed to build window");
        
        let mut web_context = WebContext::new(None);
        let _webview = WebViewBuilder::new_with_web_context(&mut web_context)
            .with_url("https://www.google.com")
            .build(&window)
            .expect("Failed to build webview");

        event_loop.run(move |event, _, control_flow| {
            *control_flow = ControlFlow::Wait;

            match event {
                Event::NewEvents(StartCause::Init) => {
                    log::info!("MCPDesk Wry-Tao application started. WebView initialized.");
                }
                Event::WindowEvent {
                    event: WindowEvent::CloseRequested,
                    ..
                } => {
                    log::info!("MCPDesk Wry-Tao window close requested. Exiting.");
                    *control_flow = ControlFlow::Exit
                },
                _ => (),
            }
        });
        Ok(()) // event_loop.run() is diverging, this Ok(()) is technically unreachable.
               // However, to make the compiler happy with the Result return type, it's there.
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
