// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command

mod cmd;

fn main() {
    tauri::Builder
        ::default()
        .invoke_handler(
            tauri::generate_handler![
                cmd::insert_content,
                cmd::remove_content,
                cmd::update_content,
                cmd::insert_note,
                cmd::remove_note,
                cmd::get_note_contents,
                cmd::insert_group,
                cmd::remove_group,
                cmd::get_menu,
                cmd::get_group_items,
                cmd::get_groups
            ]
        )
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
