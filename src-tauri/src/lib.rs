use rand::seq::SliceRandom;
use serde::Serialize;
use std::sync::Mutex;
use tokio::time::{sleep, Duration};
use tauri::Emitter;

#[derive(Serialize, Clone, Copy)]
struct EventPayload {
    value: i32,
    remaining_length: usize,
}

/// To describe is the thread has opened
static IS_OPENED_THREAD: Mutex<bool> = Mutex::new(false);
static RANDOM_POOL: Mutex<Vec<i32>> = Mutex::new(Vec::new());

#[tauri::command]
fn init_random_pool(min: i32, max: i32) {
    let mut random_pool = RANDOM_POOL.lock().unwrap();
    *random_pool = (min..=max).collect::<Vec<_>>();
}

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
async fn choose_number(app: tauri::AppHandle) {
    let mut guard = IS_OPENED_THREAD.lock().unwrap();
    if *guard {
        return;
    }
    *guard = true;
    drop(guard);

    tauri::async_runtime::spawn(async move {
        loop {
            // Check is stopped
            let running = *IS_OPENED_THREAD.lock().unwrap();
            if !running {
                let mut pool = RANDOM_POOL.lock().unwrap();
                println!("Not running, current pool: {:?}", pool);
                if !pool.is_empty() {
                    let last = pool.pop().unwrap();
                    println!("停止，移除: {}，剩余: {}", last, pool.len());
                }
                break;
            }

            let val = {
                let mut pool = RANDOM_POOL.lock().unwrap();
                println!("Running, current pool: {:?}", pool);
                if pool.is_empty() {
                    return;
                }
                pool.shuffle(&mut rand::rng());
                let v = pool[pool.len() - 1];
                v   // Lock released here
            };

            let _ = {
                let pool = RANDOM_POOL.lock().unwrap();
                app.emit("random_number", EventPayload { value: val, remaining_length: pool.len()})
            };

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
        .invoke_handler(tauri::generate_handler![choose_number, stop_choose, init_random_pool])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
