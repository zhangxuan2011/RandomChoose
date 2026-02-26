use rand::random_range;
use serde::Serialize;
use std::sync::Mutex;
use tokio::time::{sleep, Duration};
use tauri::Emitter;

#[derive(Serialize, Clone, Copy)]
struct EventPayload {
    value: i32,
}

/// To describe is the thread has opened
static IS_OPENED_THREAD: Mutex<bool> = Mutex::new(false);

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
async fn choose_number(app: tauri::AppHandle, min: i32, max: i32) {
    let mut guard = IS_OPENED_THREAD.lock().unwrap();
    if *guard {
        return;
    }
    *guard = true;
    drop(guard);

    tauri::async_runtime::spawn(async move {
        while *IS_OPENED_THREAD.lock().unwrap() {
            let random_number = random_range(min..=max);
            app.emit(
                "random_number",
                EventPayload {
                    value: random_number,
                },
            )
            .unwrap();

            sleep(Duration::from_millis(5)).await;
        }
    });
}

#[tauri::command]
fn stop_choose() {
    let mut guard = IS_OPENED_THREAD.lock().unwrap();
    *guard = false;
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![choose_number, stop_choose])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
