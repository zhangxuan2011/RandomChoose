use rand::random_range;
use serde::Serialize;
use std::{thread::sleep, time::Duration};
use tauri::Emitter;

#[derive(Serialize, Clone, Copy)]
struct EventPayload {
    value: i32,
}

/// To describe is the thread has opened
static mut IS_OPENED_THREAD: bool = false;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
async fn choose_number(app: tauri::AppHandle, min: i32, max: i32) {
    if unsafe { !IS_OPENED_THREAD } {
        tauri::async_runtime::spawn(async move {
            loop {
                let random_number = random_range(min..=max);
                app.emit(
                    "random_number",
                    EventPayload {
                        value: random_number,
                    },
                )
                .unwrap();
                sleep(Duration::from_millis(5));
            }
        });
        unsafe { IS_OPENED_THREAD = true }
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![choose_number])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
