// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command

mod command;

fn main() {
    tauri::Builder
        ::default()
        .invoke_handler(
            tauri::generate_handler![
                command::insert_content,
                command::remove_content,
                command::update_content,
                command::insert_note,
                command::remove_note,
                command::insert_group,
                command::remove_group,
                command::get_menu
            ]
        )
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
