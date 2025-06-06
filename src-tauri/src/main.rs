#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{
    Builder, WebviewUrl, WebviewWindowBuilder, Window, generate_context, generate_handler,
};

mod discord;

#[tauri::command]
fn toggle_fullscreen(window: Window) {
    let is_fullscreen = window.is_fullscreen().unwrap_or(false);
    window.set_fullscreen(!is_fullscreen).unwrap();
}

#[tauri::command]
fn open_url(url: String) {
    webbrowser::open(&url).ok();
}

fn main() {
    discord::drpc_init();

    Builder::default()
        .plugin(tauri_plugin_deep_link::init())
        .invoke_handler(generate_handler![toggle_fullscreen, open_url,])
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
