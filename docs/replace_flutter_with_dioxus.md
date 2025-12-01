# DeepWiki Q&A with Code Context for Repository: rustdesk/rustdesk
## Q1
what is the architecture of the gui and how does it compare to rust dioxus
### Answer
## GUI Architecture Overview

RustDesk uses a **hybrid Flutter-Rust architecture** where Flutter handles the UI layer while Rust manages the core application logic, connected through an FFI bridge. This differs significantly from Dioxus, which is a pure Rust GUI framework. rustdesk:39-42 rustdesk:53-58 

## Detailed Architecture

### UI Layer Components

**Modern UI (Flutter)**:
- Located in `flutter/` directory for both desktop and mobile platforms rustdesk:39-40 
- Uses Dart for UI implementation with cross-platform support
- Communicates with Rust backend via FFI

**Legacy UI (Sciter - Deprecated)**:
- Previously in `src/ui/` directory rustdesk:54-55 
- No longer actively maintained

### FFI Bridge Architecture

The Flutter-Rust integration uses `flutter_rust_bridge` for communication:

```rust
// Key FFI components
- FlutterHandler (flutter.rs:217-240)
- EventToUI Stream (flutter_ffi.rs:89-93)
- session_* functions (flutter_ffi.rs:172-894)
``` rustdesk:89-93 rustdesk:172-894 

### Session Management

The `Session<T: InvokeUiSession>` generic maintains session state and provides the interface between UI and connection logic rustdesk:51-64 .

## Comparison with Dioxus

| Aspect | RustDesk (Flutter) | Dioxus |
|--------|-------------------|--------|
| **Language** | Dart UI + Rust backend | Pure Rust |
| **Architecture** | FFI bridge between languages | Single language stack |
| **Platform Support** | Desktop + Mobile (via Flutter) | Desktop + Web (experimental mobile) |
| **Performance** | FFI overhead for UI-backend communication | Direct Rust execution |
| **Ecosystem** | Mature Flutter ecosystem | Growing Rust ecosystem |
| **Development** | Requires Dart/Flutter knowledge | Rust-only development |

### Key Differences

1. **Language Stack**: RustDesk uses two languages (Dart for UI, Rust for backend) while Dioxus uses Rust throughout rustdesk:39-42 

2. **Communication**: RustDesk requires FFI bridge for UI-backend communication, adding complexity compared to Dioxus's direct approach

3. **Maturity**: Flutter is more mature for cross-platform development, especially mobile, while Dioxus is newer but offers better Rust integration

4. **Bundle Size**: RustDesk's approach may result in larger bundles due to Flutter engine inclusion

## Notes

- RustDesk's architecture allows leveraging Flutter's mature cross-platform UI capabilities while maintaining Rust's performance for core logic
- The FFI bridge pattern is common for applications needing both rich UI and systems-level performance
- Dioxus would eliminate the FFI complexity but currently has less mature mobile support compared to Flutter
- The choice between these approaches depends on whether cross-platform mobile support or pure Rust development is prioritized

Wiki pages you might want to explore:
- [Overview (rustdesk/rustdesk)](/wiki/rustdesk/rustdesk#1)
### Citations
**File:** README.md (L39-42)
```markdown
## Dependencies

Desktop versions use Flutter or Sciter (deprecated) for GUI, this tutorial is for Sciter only, since it is easier and more friendly to start. Check out our [CI](https://github.com/rustdesk/rustdesk/blob/master/.github/workflows/flutter-build.yml) for building Flutter version.

```
**File:** CLAUDE.md (L39-40)
```markdown
- **`flutter/`** - Flutter UI code for desktop and mobile
- **`libs/`** - Core libraries
```
**File:** CLAUDE.md (L53-58)
```markdown
### UI Architecture
- **Legacy UI**: Sciter-based (deprecated) - files in `src/ui/`
- **Modern UI**: Flutter-based - files in `flutter/`
  - Desktop: `flutter/lib/desktop/`
  - Mobile: `flutter/lib/mobile/`
  - Shared: `flutter/lib/common/` and `flutter/lib/models/`
```
## Q2
plan to replace flutter with a trait and implement in dioxus
### Answer
# Plan to Replace Flutter with Trait-Based Architecture for Dioxus Implementation

## Overview

You want to replace the current Flutter UI layer with a trait-based abstraction that can be implemented by Dioxus. The current architecture already has some abstraction with `Session<T: InvokeUiSession>` which can be leveraged for this migration.

## Current Architecture Analysis

### Flutter Integration Points

The current Flutter integration is centered around these key components:

1. **FlutterHandler** (`src/flutter.rs:217-240`) - Main UI handler implementing `InvokeUiSession`
2. **FFI Bridge** (`src/flutter_ffi.rs`) - Communication layer using `flutter_rust_bridge`
3. **Event Stream** (`src/flutter_ffi.rs:89-93`) - Rust-to-Dart event channel
4. **Session Functions** (`src/flutter_ffi.rs:172-894`) - UI operations exposed to Flutter

### Existing Abstraction

The `InvokeUiSession` trait in `ui_session_interface.rs` already provides the interface abstraction:

```rust
pub trait InvokeUiSession: Send + Sync {
    fn on_rgba(&self, display: usize, rgba: &mut scrap::ImageRgb);
    fn on_rgba_texture(&self, display: usize, texture: &mut scrap::ImageTexture);
    fn msgbox(&self, msgtype: &str, title: &str, text: &str, link: &str, retry: bool);
    fn cancel_msgbox(&self, tag: &str);
    fn new_message(&self, msg: String);
    // ... other UI methods
}
```

## Migration Plan

### Phase 1: Create UI Backend Trait

Create a new trait `UiBackend` that abstracts all UI framework-specific operations:

```rust
// src/ui_backend.rs
pub trait UiBackend: Send + Sync {
    type Event;
    type SessionHandler: InvokeUiSession;
    
    fn create_session_handler(&self, session_id: String) -> Arc<Self::SessionHandler>;
    fn push_event(&self, app_type: &str, event: Self::Event) -> bool;
    fn start_event_loop(&self) -> Result<(), Box<dyn std::error::Error>>;
    fn shutdown(&self);
}
```

### Phase 2: Implement Flutter Backend

Refactor existing Flutter code to implement the new trait:

```rust
// src/flutter_backend.rs
pub struct FlutterBackend {
    event_streams: Arc<RwLock<HashMap<String, StreamSink<String>>>>,
}

impl UiBackend for FlutterBackend {
    type Event = String;
    type SessionHandler = FlutterHandler;
    
    fn create_session_handler(&self, session_id: String) -> Arc<FlutterHandler> {
        // Migrate from existing session_add logic
    }
    
    fn push_event(&self, app_type: &str, event: String) -> bool {
        // Use existing push_global_event logic
    }
    
    fn start_event_loop(&self) -> Result<(), Box<dyn std::error::Error>> {
        // Initialize Flutter engine
    }
}
```

### Phase 3: Implement Dioxus Backend

Create the Dioxus implementation:

```rust
// src/dioxus_backend.rs
pub struct DioxusBackend {
    event_channel: Arc<Mutex<VecDeque<DioxusEvent>>>,
    sessions: Arc<RwLock<HashMap<String, Arc<DioxusHandler>>>>,
}

#[derive(Clone, Debug)]
pub enum DioxusEvent {
    MsgBox { msgtype: String, title: String, text: String },
    NewMessage { msg: String },
    SwitchDisplay { display: SwitchDisplay },
    // ... other events
}

impl UiBackend for DioxusBackend {
    type Event = DioxusEvent;
    type SessionHandler = DioxusHandler;
    
    fn create_session_handler(&self, session_id: String) -> Arc<DioxusHandler> {
        let handler = Arc::new(DioxusHandler::new(session_id.clone()));
        self.sessions.write().unwrap().insert(session_id, handler.clone());
        handler
    }
    
    fn push_event(&self, app_type: &str, event: DioxusEvent) -> bool {
        self.event_channel.lock().unwrap().push_back(event);
        true
    }
    
    fn start_event_loop(&self) -> Result<(), Box<dyn std::error::Error>> {
        // Launch Dioxus app
        dioxus_desktop::launch_cfg(App, Config::default());
        Ok(())
    }
}

pub struct DioxusHandler {
    session_id: String,
    event_channel: Arc<Mutex<VecDeque<DioxusEvent>>>,
}

impl InvokeUiSession for DioxusHandler {
    fn on_rgba(&self, display: usize, rgba: &mut scrap::ImageRgb) {
        self.event_channel.lock().unwrap().push_back(
            DioxusEvent::VideoFrame { display, data: rgba.data.clone() }
        );
    }
    
    fn msgbox(&self, msgtype: &str, title: &str, text: &str, link: &str, retry: bool) {
        self.event_channel.lock().unwrap().push_back(
            DioxusEvent::MsgBox { 
                msgtype: msgtype.to_string(), 
                title: title.to_string(), 
                text: text.to_string() 
            }
        );
    }
    // ... implement other methods
}
```

### Phase 4: Dioxus UI Components

Create the Dioxus UI components to replace Flutter widgets:

```rust
// components/main.rs
#[derive(Props, Clone, PartialEq)]
pub struct RemoteDesktopProps {
    session_id: String,
}

#[component]
pub fn RemoteDesktop(props: RemoteDesktopProps) -> Element {
    let mut video_frames = use_signal(Vec::<VideoFrame>::new);
    let mut event_channel = use_context::<Signal<DioxusEventChannel>>();
    
    // Listen for events from Rust backend
    use_coroutine(|_| {
        let mut channel = event_channel.write();
        async move {
            while let Some(event) = channel.recv().await {
                match event {
                    DioxusEvent::VideoFrame { display, data } => {
                        video_frames.write().push(VideoFrame { display, data });
                    }
                    DioxusEvent::MsgBox { title, text, .. } => {
                        // Show message box
                    }
                    // ... handle other events
                }
            }
        }
    });
    
    rsx! {
        div { class: "remote-desktop",
            div { class: "video-container",
                for frame in video_frames.read().iter() {
                    img { src: "data:image/png;base64,{frame.data}" }
                }
            }
            div { class: "controls",
                button { onclick: move |_| send_key_event("ctrl_alt_del"), 
                    "Ctrl+Alt+Del" 
                }
                // ... other controls
            }
        }
    }
}
```

### Phase 5: Backend Selection

Add configuration to select UI backend:

```rust
// src/main.rs
#[derive(Debug, Clone)]
pub enum UiBackendType {
    Flutter,
    Dioxus,
}

pub fn create_ui_backend(backend_type: UiBackendType) -> Box<dyn UiBackend> {
    match backend_type {
        UiBackendType::Flutter => Box::new(FlutterBackend::new()),
        UiBackendType::Dioxus => Box::new(DioxusBackend::new()),
    }
}

// In core_main()
let ui_backend = create_ui_backend(config.ui_backend_type);
ui_backend.start_event_loop()?;
```

## Migration Strategy

### Step-by-Step Approach

1. **Week 1-2**: Create `UiBackend` trait and migrate Flutter implementation
2. **Week 3-4**: Implement Dioxus backend with basic functionality
3. **Week 5-6**: Port core UI components (connection dialog, remote desktop)
4. **Week 7-8**: Implement advanced features (file transfer, settings)
5. **Week 9-10**: Testing and optimization
6. **Week 11-12**: Documentation and deployment

### Testing Strategy

- Keep both implementations running in parallel during migration
- Use feature flags to switch between backends: `--features ui-dioxus`
- Implement comprehensive integration tests for both backends

## Benefits of This Approach

1. **Clean Separation**: UI framework logic completely separated from core business logic
2. **Parallel Development**: Both Flutter and Dioxus can coexist during transition
3. **Future-Proof**: Easy to add other UI frameworks (Tauri, egui, etc.)
4. **Testing**: Can test UI logic independently of framework
5. **Performance**: Eliminates FFI overhead when using Dioxus

## Notes

- The existing `InvokeUiSession` trait provides a solid foundation for this migration
- Flutter's mature ecosystem can still be leveraged during the transition period
- Dioxus implementation will have better performance due to eliminating FFI bridge
- Consider mobile platform requirements - Dioxus mobile support is still experimental compared to Flutter's mature mobile support
- The event-driven architecture in `src/flutter_ffi.rs` can be adapted for Dioxus's signal system

This plan maintains backward compatibility while enabling a clean migration to Dioxus through trait-based abstraction.

Wiki pages you might want to explore:
- [Overview (rustdesk/rustdesk)](/wiki/rustdesk/rustdesk#1)
