use rand::seq::SliceRandom;
use serde::{Deserialize, Serialize};
use std::{fs::read_to_string, sync::Mutex};
use tauri::{AppHandle, Emitter};
use tauri_plugin_dialog::{DialogExt, MessageDialogButtons, MessageDialogKind};
use tokio::time::{sleep, Duration};

/* =====<BASIC DATA>===== */
#[derive(Serialize, Debug, Clone, Copy)]
struct EventPayload {
    value: i32,
    remaining_length: usize,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct ConfigData {
    pub essential: ConfigDataEssential,
    pub optional: ConfigDataOptional,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct ConfigDataEssential {
    pub min_num: i32,
    pub max_num: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct ConfigDataOptional {
    pub wait_millis: Option<u32>,
}

/// To describe is the thread has opened
static IS_OPENED_THREAD: Mutex<bool> = Mutex::new(false);
static RANDOM_POOL: Mutex<Vec<i32>> = Mutex::new(Vec::new());

/* =====<BASIC DATA>===== */
/* =====<MAIN APP FUNCTIONS>===== */

#[tauri::command]
fn init_random_pool(app: AppHandle, min: i32, max: i32) {
    if min >= max {
        // min >= max is not allowed
        app.dialog()
            .message("最小值不可以大于最大值!\n请前往 设置 -> 基础设置项检查并修改最小值至最大值以下。")
            .title("无法初始化程序 (critical)")
            .buttons(MessageDialogButtons::Ok)
            .kind(MessageDialogKind::Error)
            .blocking_show();
        return;
    }

    let mut random_pool = RANDOM_POOL.lock().unwrap();
    *random_pool = (min..=max).collect::<Vec<_>>();

    if random_pool.is_empty() {
        app.dialog()
            .message("随机数池储存的值为空，这通常是配置文件未正确配置导致的。")
            .title("无法初始化程序 (critical)")
            .buttons(MessageDialogButtons::Ok)
            .kind(MessageDialogKind::Error)
            .blocking_show();
        return;
    }
}

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
async fn choose_number(app: AppHandle, min: i32, max: i32) {
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
                if pool.is_empty() {
                    // Re-initialize the random pool
                    println!("重新初始化随机数池");
                    *pool = (min..=max).collect::<Vec<_>>();
                }
                println!("Running, current pool: {:?}", pool);
                pool.shuffle(&mut rand::rng());
                let v = pool[pool.len() - 1];
                v // Lock released here
            };

            let _ = {
                let pool = RANDOM_POOL.lock().unwrap();
                app.emit(
                    "random_number",
                    EventPayload {
                        value: val,
                        remaining_length: pool.len(),
                    },
                )
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

/* =====<MAIN APP FUNCTIONS>===== */
/* =====<SETTINGS APP FUNCTIONS>===== */
#[tauri::command]
async fn get_config(app: AppHandle) -> Result<(), String> {
    // The default config file is at `config.json`
    let config_raw = read_to_string("config.json")
        .map_err(|e| format!("Failed to read the config data: {}", e))?;

    // Parse the configuration
    let config: ConfigData = serde_json::from_str(&config_raw)
        .map_err(|e| format!("Failed to serialize the config: {}", e))?;

    // Emit to frontend
    let _ = app.emit("config", config);
    Ok(())
}

#[tauri::command]
async fn write_config(app: AppHandle, config: ConfigData) -> Result<(), String> {
    // Deserialize it to string
    let config_raw = serde_json::to_string(&config)
        .map_err(|e| {
            let message = format!("Could not deserialize the config: {}", e);
            app.dialog()
                .message(message.clone())
                .title("写入配置文件失败 (critical)")
                .buttons(MessageDialogButtons::Ok)
                .kind(MessageDialogKind::Error)
                .blocking_show();
            message
        })?;

    // Write to file
    std::fs::write("config.json", config_raw)
        .map_err(|e| {
            let message = format!("Failed to write configuration file to config file: {}", e);
            app.dialog()
                .message(message.clone())
                .title("写入配置文件失败 (critical)")
                .buttons(MessageDialogButtons::Ok)
                .kind(MessageDialogKind::Error)
                .blocking_show();
            message
        })?;

    app.dialog()
        .message("成功写入了配置文件！")
        .title("提示")
        .kind(MessageDialogKind::Info)
        .blocking_show();

    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            choose_number,
            stop_choose,
            init_random_pool,
            get_config,
            write_config
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
