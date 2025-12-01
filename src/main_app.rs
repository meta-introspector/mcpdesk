// rustdesk/src/main_app.rs
use dioxus::prelude::*;
use hbb_common::log;

// Define the root Dioxus component for the application.
// This will replace the placeholder in dioxus_backend.rs
pub fn App() -> Element {
    log::info!("Dioxus App: Initializing main application component.");
    rsx! {
        div {
            h1 { "mcpdesk Dioxus UI" }
            p { "Welcome to the new mcpdesk user interface powered by Dioxus!" }
            p { "Status: Under Construction" }
        }
    }
}
