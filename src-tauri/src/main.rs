// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::Manager;
use tauri_plugin_updater::UpdaterExt;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_updater::Builder::new().build())
        .setup(move |app| {
            println!("version: {}", env!("CARGO_PKG_VERSION"));

            let app_handle = app.app_handle().clone();
            tauri::async_runtime::spawn(async move {
                let builder = app_handle.updater_builder();
                let updater = builder.build().unwrap();

                let update = match updater.check().await {
                    Ok(Some(update)) => update,
                    Ok(None) => {
                        println!("no update.");
                        return;
                    }
                    Err(err) => {
                        println!("err: {:?}", err);
                        return;
                    }
                };

                println!("update available");
                let Ok(package) = update.download(|_, _| {}, || {}).await else {
                    return;
                };

                println!("downloaded");
                match update.install(package) {
                    Ok(_) => {
                        println!("restart");
                        app_handle.restart();
                    }
                    Err(err) => {
                        println!("err: {:?}", err);
                    }
                }
            });
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
