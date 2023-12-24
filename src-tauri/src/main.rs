// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command

use noter::menu_manager::MenuManager;

// #[tauri::command]
// async fn initialize_menu() -> String {
//     let mut menu_manager = MenuManager::new();
//     // menu_manager.test_init_root();
//     menu_manager.serialize()
// }

fn main() {
    tauri::Builder
        ::default()
        .invoke_handler(tauri::generate_handler![])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
