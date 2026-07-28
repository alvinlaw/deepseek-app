// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::menu::{Menu, MenuItem, PredefinedMenuItem};
use tauri::Manager;

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            let handle = app.handle();
            let reload = MenuItem::with_id(handle, "reload", "Reload", true, Some("CmdOrCtrl+R"))?;
            let separator = PredefinedMenuItem::separator(handle)?;
            let quit = PredefinedMenuItem::quit(handle, Some("Quit"))?;
            let menu = Menu::new(handle)?;
            menu.append(&reload)?;
            menu.append(&separator)?;
            menu.append(&quit)?;
            app.set_menu(menu)?;
            Ok(())
        })
        .on_menu_event(|app, event| {
            if event.id().as_ref() == "reload" {
                if let Some(window) = app.get_webview_window("main") {
                    let _ = window.reload();
                }
            }
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
