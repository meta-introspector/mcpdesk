use std::{
    collections::VecDeque,
    sync::{Arc, Mutex},
};
use crate::ui_backend::ui_session_interface::InvokeUiSession;
use crate::dioxus_backend::DioxusEvent;
use hbb_common::{
    log,
    message_proto::{CursorData, CursorPosition, DisplayInfo, PeerInfo, SwitchDisplay, WindowsSession, FileEntry, ReadEmptyDirsResponse, TerminalResponse, ConnType},
};
use scrap::ImageRgb; // Assuming scrap::ImageRgb is available

#[derive(Clone, Default)]
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



// Basic implementation of InvokeUiSession for DioxusHandler
impl InvokeUiSession for DioxusHandler {

    fn set_cursor_data(&self, cd: CursorData) {
        log::debug!("DioxusHandler: set_cursor_data: {:?}", cd);
    }
    fn set_cursor_id(&self, id: String) {
        log::debug!("DioxusHandler: set_cursor_id: {}", id);
    }
    fn set_cursor_position(&self, cp: CursorPosition) {
        log::debug!("DioxusHandler: set_cursor_position: {:?}", cp);
    }
    fn set_display(&self, x: i32, y: i32, w: i32, h: i32, cursor_embedded: bool, scale: f64) {
        log::debug!("DioxusHandler: set_display: x={}, y={}, w={}, h={}, cursor_embedded={}, scale={}", x, y, w, h, cursor_embedded, scale);
    }
    fn switch_display(&self, display: &SwitchDisplay) {
        log::debug!("DioxusHandler: switch_display: {:?}", display);
        self.event_channel.lock().unwrap().push_back(DioxusEvent::SwitchDisplay { display: display.clone() });
    }
    fn set_peer_info(&self, peer_info: &PeerInfo) {
        log::debug!("DioxusHandler: set_peer_info: {:?}", peer_info);
    }
    fn set_displays(&self, displays: &Vec<DisplayInfo>) {
        log::debug!("DioxusHandler: set_displays: {:?}", displays);
    }
    fn set_platform_additions(&self, data: &str) {
        log::debug!("DioxusHandler: set_platform_additions: {}", data);
    }
    fn on_connected(&self, conn_type: ConnType) {
        log::debug!("DioxusHandler: on_connected: {:?}", conn_type);
    }
    fn update_privacy_mode(&self) {
        log::debug!("DioxusHandler: update_privacy_mode");
    }
    fn set_permission(&self, name: &str, value: bool) {
        log::debug!("DioxusHandler: set_permission: {}, {}", name, value);
    }
    fn close_success(&self) {
        log::debug!("DioxusHandler: close_success");
    }
    fn update_quality_status(&self, qs: crate::client::QualityStatus) {
        log::debug!("DioxusHandler: update_quality_status: {:?}", qs);
    }
    fn set_connection_type(&self, is_secured: bool, direct: bool, stream_type: &str) {
        log::debug!("DioxusHandler: set_connection_type: is_secured={}, direct={}, stream_type={}", is_secured, direct, stream_type);
    }
    fn set_fingerprint(&self, fingerprint: String) {
        log::debug!("DioxusHandler: set_fingerprint: {}", fingerprint);
    }
    fn job_error(&self, id: i32, err: String, file_num: i32) {
        log::debug!("DioxusHandler: job_error: id={}, err={}, file_num={}", id, err, file_num);
    }
    fn job_done(&self, id: i32, file_num: i32) {
        log::debug!("DioxusHandler: job_done: id={}, file_num={}", id, file_num);
    }
    fn clear_all_jobs(&self) {
        log::debug!("DioxusHandler: clear_all_jobs");
    }
    fn new_message(&self, msg: String) {
        log::debug!("DioxusHandler: new_message: {}", msg);
        self.event_channel.lock().unwrap().push_back(DioxusEvent::NewMessage { msg });
    }
    fn update_transfer_list(&self) {
        log::debug!("DioxusHandler: update_transfer_list");
    }
    fn load_last_job(&self, cnt: i32, job_json: &str, auto_start: bool) {
        log::debug!("DioxusHandler: load_last_job: cnt={}, job_json={}, auto_start={}", cnt, job_json, auto_start);
    }
    fn update_folder_files(
        &self,
        id: i32,
        entries: &Vec<FileEntry>,
        path: String,
        is_local: bool,
        only_count: bool,
    ) {
        log::debug!("DioxusHandler: update_folder_files: id={}, entries={:?}, path={}, is_local={}, only_count={}", id, entries, path, is_local, only_count);
    }
    fn confirm_delete_files(&self, id: i32, i: i32, name: String) {
        log::debug!("DioxusHandler: confirm_delete_files: id={}, i={}, name={}", id, i, name);
    }
    fn override_file_confirm(
        &self,
        id: i32,
        file_num: i32,
        to: String,
        is_upload: bool,
        is_identical: bool,
    ) {
        log::debug!("DioxusHandler: override_file_confirm: id={}, file_num={}, to={}, is_upload={}, is_identical={}", id, file_num, to, is_upload, is_identical);
    }
    fn update_block_input_state(&self, on: bool) {
        log::debug!("DioxusHandler: update_block_input_state: {}", on);
    }
    fn job_progress(&self, id: i32, file_num: i32, speed: f64, finished_size: f64) {
        log::debug!("DioxusHandler: job_progress: id={}, file_num={}, speed={}, finished_size={}", id, file_num, speed, finished_size);
    }
    fn adapt_size(&self) {
        log::debug!("DioxusHandler: adapt_size");
    }
    fn on_rgba(&self, display: usize, rgba: &mut ImageRgb) {
        log::debug!("DioxusHandler: on_rgba: display={}", display);
        // Placeholder: send video frame data to Dioxus UI
        // self.event_channel.lock().unwrap().push_back(DioxusEvent::VideoFrame { display, data: rgba.data.clone() });
    }
    fn msgbox(&self, msgtype: &str, title: &str, text: &str, link: &str, retry: bool) {
        log::debug!("DioxusHandler: msgbox: msgtype={}, title={}, text={}, link={}, retry={}", msgtype, title, text, link, retry);
        self.event_channel.lock().unwrap().push_back(DioxusEvent::MsgBox { msgtype: msgtype.to_string(), title: title.to_string(), text: text.to_string() });
    }
    #[cfg(any(target_os = "android", target_os = "ios"))]
    fn clipboard(&self, content: String) {
        log::debug!("DioxusHandler: clipboard: {}", content);
    }
    fn cancel_msgbox(&self, tag: &str) {
        log::debug!("DioxusHandler: cancel_msgbox: {}", tag);
    }
    fn switch_back(&self, id: &str) {
        log::debug!("DioxusHandler: switch_back: {}", id);
    }
    fn portable_service_running(&self, running: bool) {
        log::debug!("DioxusHandler: portable_service_running: {}", running);
    }
    fn on_voice_call_started(&self) {
        log::debug!("DioxusHandler: on_voice_call_started");
    }
    fn on_voice_call_closed(&self, reason: &str) {
        log::debug!("DioxusHandler: on_voice_call_closed: {}", reason);
    }
    fn on_voice_call_waiting(&self) {
        log::debug!("DioxusHandler: on_voice_call_waiting");
    }
    fn on_voice_call_incoming(&self) {
        log::debug!("DioxusHandler: on_voice_call_incoming");
    }
    fn get_rgba(&self, display: usize) -> *const u8 {
        log::debug!("DioxusHandler: get_rgba: display={}", display);
        std::ptr::null() // Placeholder
    }
    fn next_rgba(&self, display: usize) {
        log::debug!("DioxusHandler: next_rgba: display={}", display);
    }
    #[cfg(all(feature = "vram", feature = "flutter"))]
    fn on_texture(&self, display: usize, texture: *mut c_void) {
        log::debug!("DioxusHandler: on_texture: display={:?}, texture={:?}", display, texture);
    }
    fn set_multiple_windows_session(&self, sessions: Vec<WindowsSession>) {
        log::debug!("DioxusHandler: set_multiple_windows_session: {:?}", sessions);
    }
    fn set_current_display(&self, disp_idx: i32) {
        log::debug!("DioxusHandler: set_current_display: {}", disp_idx);
    }
    fn is_multi_ui_session(&self) -> bool {
        log::debug!("DioxusHandler: is_multi_ui_session");
        false // Placeholder for Dioxus
    }
    fn update_record_status(&self, start: bool) {
        log::debug!("DioxusHandler: update_record_status: {}", start);
    }
    fn update_empty_dirs(&self, _res: ReadEmptyDirsResponse) {
        log::debug!("DioxusHandler: update_empty_dirs: {:?}", _res);
    }
    fn printer_request(&self, id: i32, path: String) {
        log::debug!("DioxusHandler: printer_request: id={}, path={}", id, path);
    }
    fn handle_screenshot_resp(&self, sid: String, msg: String) {
        log::debug!("DioxusHandler: handle_screenshot_resp: sid={}, msg={}", sid, msg);
    }
    fn handle_terminal_response(&self, response: TerminalResponse) {
        log::debug!("DioxusHandler: handle_terminal_response: {:?}", response);
    }
}


