use std::sync::Mutex;
use lazy_static::lazy_static;
use noter::manager::Manager;

lazy_static! {
    static ref MANAGER: Mutex<Manager> = Mutex::new(Manager::default());
}

#[tauri::command]
pub async fn get_menu() -> String {
    let manager = MANAGER.lock().unwrap();
    manager.get_menu()
}

#[tauri::command]
pub async fn insert_content(note_key: String, json: String) {
    let manager = MANAGER.lock().unwrap();
    manager.insert_note_content(note_key, json);
}

#[tauri::command]
pub async fn remove_content(note_key: String, index: usize) {
    let manager = MANAGER.lock().unwrap();
    manager.remove_note_content(note_key, index);
}

#[tauri::command]
pub async fn update_content(note_key: String, index: usize, json: String) {
    let manager = MANAGER.lock().unwrap();
    manager.update_note_content(note_key, index, json);
}

#[tauri::command]
pub async fn insert_note(group_key: String, note_name: String) {
    let manager = MANAGER.lock().unwrap();
    manager.insert_note(group_key, note_name);
}

#[tauri::command]
pub async fn remove_note(group_key: String, key: String) {
    let manager = MANAGER.lock().unwrap();
    manager.remove_note(group_key, key);
}

#[tauri::command]
pub async fn insert_group(parent_group_key: String, group_name: String) {
    let manager = MANAGER.lock().unwrap();
    manager.insert_group(parent_group_key, group_name);
}

#[tauri::command]
pub async fn remove_group(parent_group_key: String, key: String) {
    let manager = MANAGER.lock().unwrap();
    manager.remove_group(parent_group_key, key);
}
