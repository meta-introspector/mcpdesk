// rustdesk/src/main_app.rs
use dioxus::prelude::*;
use hbb_common::log;
use solfunmeme_ui::MemeManagement;
use solfunmeme_loader::{MemeLoader, Result}; // Added MemeLoader trait import
use solfunmeme_core::StaticMemeSource; // Added StaticMemeSource import
use std::rc::Rc; // Added for Rc<dyn MemeLoader>

// Simple implementation of MemeLoader that provides StaticMemeSource
struct StaticMemeLoader;

impl MemeLoader for StaticMemeLoader {
    fn load_source(&self, source_id: &str) -> Result<Box<dyn solfunmeme_loader::MemeSource>> {
        if source_id == "static" {
            Ok(Box::new(StaticMemeSource))
        } else {
            Err(Box::new(solfunmeme_loader::MemeLoaderError::Other(
                format!("Unknown meme source: {}", source_id),
            )))
        }
    }
}

pub fn App() -> Element {
    log::info!("Dioxus App: Initializing main application component.");

    // Initialize our StaticMemeLoader
    let meme_loader: Rc<dyn MemeLoader> = Rc::new(StaticMemeLoader);

    // Get the MemeSource from the loader
    let meme_source = use_memo(move || {
        meme_loader.load_source("static").unwrap_or_else(|e| {
            log::error!("Failed to load static meme source: {:?}", e);
            Box::new(StaticMemeSource) // Fallback or handle error appropriately
        })
    });

    rsx! {
        div {
            h1 { "mcpdesk Dioxus UI" }
            p { "Welcome to the new mcpdesk user interface powered by Dioxus!" }
            p { "Status: Under Construction" }
            MemeManagement { meme_source: meme_source.clone() } // Pass meme_source to MemeManagement
        }
    }
}