use rand::prelude::*;
use serde::{Deserialize, Serialize};
use std::{fs::read_to_string, path::PathBuf, sync::Mutex};
use tauri::{AppHandle, Emitter, Manager};
use tauri_plugin_dialog::{DialogExt, MessageDialogButtons, MessageDialogKind};
use tauri_plugin_fs::FsExt;
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
    pub wait_millis: Option<u64>,
}

/// To describe is the thread has opened
static IS_OPENED_THREAD: Mutex<bool> = Mutex::new(false);
static RANDOM_POOL: Mutex<Vec<i32>> = Mutex::new(Vec::new());

/* =====<BASIC DATA>===== */
/* =====<MAIN APP FUNCTIONS>===== */

#[tauri::command]
fn init_random_pool(app: AppHandle, min: i32, max: i32) {
    if min > max {
        // min >= max is not allowed
        app.dialog()
            .message(
                "最小值不可以大于最大值!\n请前往 设置 -> 基础设置项检查并修改最小值至最大值以下。",
            )
            .title("无法初始化程序 (critical)")
            .buttons(MessageDialogButtons::Ok)
            .kind(MessageDialogKind::Error)
            .blocking_show();
        return;
    }

    let mut random_pool = RANDOM_POOL.lock().unwrap();
    random_pool.clear();

    if !random_pool.is_empty() {
        app.dialog()
            .message("随机数池储存的值非空，这通常是配置文件未正确配置导致的。")
            .title("无法初始化程序 (critical)")
            .buttons(MessageDialogButtons::Ok)
            .kind(MessageDialogKind::Error)
            .blocking_show();
        return;
    }
}

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
async fn choose_number(app: AppHandle, min: i32, max: i32, wait_millis: u64) {
    let mut guard = IS_OPENED_THREAD.lock().unwrap();
    if *guard {
        return;
    }
    *guard = true;
    drop(guard);

    tauri::async_runtime::spawn(async move {
        loop {
            let running = *IS_OPENED_THREAD.lock().unwrap();
            let total = (max - min + 1) as usize;

            let (val, should_emit, should_break) = {
                let mut pool = RANDOM_POOL.lock().unwrap();
                if pool.len() >= total {
                    // Re-initialize the random pool
                    println!("重新初始化随机数池");
                    pool.clear();
                }
                let v = rand::rng().random_range(min..=max);
                println!("Running, current pool: {:?}, num: {}", pool, v);

                // Check is the value contains in the pool
                if !running {
                    println!("Not running, current pool: {:?}, num: {}", pool, v);

                    if pool.contains(&v) {
                        println!("The number {v} has chosen, re-choosing...");
                        (v, false, true)
                    } else {
                        pool.push(v);
                        println!("Not exist in pool, performing push {} and break...", v);
                        (v, true, true)
                    }
                } else {
                    (v, true, false)
                }
            };

            if should_emit {
                let _ = {
                    let pool = RANDOM_POOL.lock().unwrap();
                    app.emit(
                        "random_number",
                        EventPayload {
                            value: val,
                            remaining_length: total - pool.len(),
                        },
                    )
                };
                if should_break { break; }
            }

            sleep(Duration::from_millis(wait_millis)).await;
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
fn get_config_path(app: AppHandle) -> Result<PathBuf, String> {
    let app_data_dir = app.path().app_data_dir().map_err(|e| {
        app.dialog()
            .message("在查找应用数据目录时出现错误")
            .title("无法读取配置文件")
            .kind(MessageDialogKind::Error)
            .blocking_show();
        format!("Cannot get appdata dir: {}", e)
    })?;

    // Create the directory of appdata
    let _ = std::fs::create_dir_all(&app_data_dir);

    let config_path = app_data_dir.join("config.json");

    Ok(config_path)
}

#[tauri::command]
async fn get_config(app: AppHandle) -> Result<(), String> {
    let config_path = get_config_path(app.clone())?;

    // The default config file is at `config.json`
    let config_raw = read_to_string(&config_path)
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
    let config_path = get_config_path(app.clone())?;

    // Deserialize it to string
    let config_raw = serde_json::to_string(&config).map_err(|e| {
        let message = format!("Could not deserialize the config: {}", e);
        app.dialog()
            .message(message.clone())
            .title("写入配置文件失败 (critical)")
            .buttons(MessageDialogButtons::Ok)
            .kind(MessageDialogKind::Error)
            .blocking_show();
        message
    })?;

    // Pre-check
    let min_num = &config.essential.min_num;
    let max_num = &config.essential.max_num;

    if min_num > max_num {
        app.dialog()
            .message("最小值不能大于最大值！\n请重新设置最小值和最大值！")
            .title("无法写入配置文件")
            .buttons(MessageDialogButtons::Ok)
            .kind(MessageDialogKind::Error)
            .blocking_show();
        return Err(String::from("minnum larger than maxnum"));
    }

    // Write to file
    std::fs::write(&config_path, config_raw).map_err(|e| {
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
        .setup(|app| {
            let scope = app.fs_scope();
            let _ = scope.allow_directory("C:\\Users", true);

            Ok(())
        })
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
