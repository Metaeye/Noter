use std::sync::Mutex;
use lazy_static::lazy_static;
use noter::manager::Manager;

lazy_static! {
    static ref MANAGER: Mutex<Manager> = Mutex::new(Manager::default());
}

#[tauri::command(rename_all = "snake_case")]
pub fn get_menu() -> String {
    println!("Calling get_menu");
    let manager = MANAGER.lock().unwrap();
    manager.get_menu()
}

#[tauri::command(rename_all = "snake_case")]
pub fn get_groups() -> String {
    println!("Calling get_groups");
    let manager = MANAGER.lock().unwrap();
    manager.get_groups()
}

#[tauri::command(rename_all = "snake_case")]
pub fn get_group_items(group_key: String) -> String {
    println!("Calling get_group_items");
    let manager = MANAGER.lock().unwrap();
    manager.get_group_items(group_key)
}

#[tauri::command(rename_all = "snake_case")]
pub fn insert_content(note_key: String, json: String) {
    println!("Calling insert_content");
    let manager = MANAGER.lock().unwrap();
    manager.insert_note_content(note_key, json);
}

#[tauri::command(rename_all = "snake_case")]
pub fn remove_content(note_key: String, index: usize) {
    println!("Calling remove_content");
    let manager = MANAGER.lock().unwrap();
    manager.remove_note_content(note_key, index);
}

#[tauri::command(rename_all = "snake_case")]
pub fn update_content(note_key: String, index: usize, json: String) {
    println!("Calling update_content");
    let manager = MANAGER.lock().unwrap();
    manager.update_note_content(note_key, index, json);
}

#[tauri::command(rename_all = "snake_case")]
pub fn insert_note(group_key: String, note_name: String) {
    println!("Calling insert_note");
    let manager = MANAGER.lock().unwrap();
    manager.insert_note(group_key, note_name);
}

#[tauri::command(rename_all = "snake_case")]
pub fn remove_note(group_key: String, key: String) {
    println!("Calling remove_note");
    let manager = MANAGER.lock().unwrap();
    manager.remove_note(group_key, key);
}

#[tauri::command(rename_all = "snake_case")]
pub fn insert_group(parent_group_key: String, group_name: String) {
    println!("Calling insert_group");
    let manager = MANAGER.lock().unwrap();
    manager.insert_group(parent_group_key, group_name);
}

#[tauri::command(rename_all = "snake_case")]
pub fn remove_group(parent_group_key: String, key: String) {
    println!("Calling remove_group");
    let manager = MANAGER.lock().unwrap();
    manager.remove_group(parent_group_key, key);
}
