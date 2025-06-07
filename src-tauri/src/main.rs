#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{
    Builder, Manager, WebviewUrl, WebviewWindowBuilder, Window, generate_context, generate_handler,
};

mod discord;

#[tauri::command]
fn toggle_fullscreen(window: Window) {
    let is_fullscreen = window.is_fullscreen().unwrap_or(false);
    window.set_fullscreen(!is_fullscreen).unwrap();
}

fn main() {
    discord::drpc_init();

    Builder::default()
        .plugin(tauri_plugin_deep_link::init())
        .plugin(tauri_plugin_single_instance::init(|app, _, _| {
            if let Some(window) = app.get_webview_window("main") {
                let _ = window.show();
                let _ = window.set_focus();
            }
        }))
        .invoke_handler(generate_handler![toggle_fullscreen])
        .setup(|app| {
            let script = include_str!("../../frontend-dist/script.js");

            WebviewWindowBuilder::new(
                app,
                "main",
                WebviewUrl::External("https://vectaria.io".parse().unwrap()),
            )
            .title("Solid Client")
            .initialization_script(script)
            // .additional_browser_args("--disable-gpu-vsync")
            .build()
            .unwrap();

            Ok(())
        })
        .run(generate_context!())
        .expect("Error while running Tauri app");
}
