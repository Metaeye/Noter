use std::time::{ SystemTime, UNIX_EPOCH };
use serde_json::json;
use sled::Tree;

use crate::{ group::Group, note::Note, menu::{ Menu, MenuItem } };

pub struct Manager {
    db: sled::Db,
}

impl Default for Manager {
    fn default() -> Self {
        let exe_path = std::env::current_exe().expect("Failed to get current exe path");
        let install_path = exe_path.parent().expect("Failed to get install path");
        let db = sled
            ::open(format!("{}\\db", install_path.display()))
            .expect("Failed to open database");
        let _ = db.open_tree("notes").expect("Failed to open tree");
        let groups = db.open_tree("groups").expect("Failed to open tree");
        match groups.get("root") {
            Ok(Some(_)) => {}
            _ => {
                let _ = groups.insert("root", Group::new("root".to_string()));
            }
        }
        Self { db }
    }
}

impl Manager {
    pub fn new(db_path: &str) -> Self {
        let db = sled::open(db_path).expect("Failed to open database");
        let _ = db.open_tree("notes").expect("Failed to open tree");
        let groups = db.open_tree("groups").expect("Failed to open tree");
        match groups.get("root") {
            Ok(_) => (),
            _ => {
                let _ = groups.insert("root", Group::new("root".to_string()));
            }
        }
        Self { db }
    }

    pub fn clear_db(&self) {
        self.db.open_tree("notes").unwrap().clear().unwrap();
        let groups = self.db.open_tree("groups").unwrap();
        groups.clear().unwrap();
        let _ = groups.insert("root", Group::new(String::from("root")));
    }

    pub fn get_menu(&self) -> String {
        let root = Group::from(self.db.open_tree("groups").unwrap().get("root").unwrap().unwrap());
        let menu = Menu::new(
            root,
            "root",
            &self.db.open_tree("groups").unwrap(),
            &self.db.open_tree("notes").unwrap()
        );

        String::from(menu)
    }

    pub fn insert_note(&self, group_key: String, note_name: String) {
        let key = Self::time_stamp();
        let notes = self.db.open_tree("notes").expect("Failed to open tree");
        let groups = self.db.open_tree("groups").expect("Failed to open tree");
        let old_group = Group::from(groups.remove(group_key.clone()).unwrap().unwrap());
        let new_group = old_group.insert_note_key(key.clone());
        groups.insert(group_key, new_group).unwrap();
        notes.insert(key, Note::new(note_name)).unwrap();
    }

    pub fn remove_note(&self, group_key: String, key: String) {
        let notes = self.db.open_tree("notes").expect("Failed to open tree");
        let groups = self.db.open_tree("groups").expect("Failed to open tree");
        let old_group = Group::from(groups.remove(group_key.clone()).unwrap().unwrap());
        let new_group = old_group.remove_note_key(key.clone());
        groups.insert(group_key, new_group).unwrap();
        notes.remove(key).unwrap();
    }

    pub fn insert_content(&self, note_key: String, json_content: String) {
        let notes = self.db.open_tree("notes").expect("Failed to open tree");
        let old_note = Note::from(notes.remove(note_key.clone()).unwrap().unwrap());
        let new_note = old_note.insert_content(json_content);
        notes.insert(note_key, new_note).unwrap();
    }

    pub fn update_content(&self, note_key: String, index: usize, json_content: String) {
        let notes = self.db.open_tree("notes").expect("Failed to open tree");
        let old_note = Note::from(notes.remove(note_key.clone()).unwrap().unwrap());
        let new_note = old_note.update_content(index, json_content);
        notes.insert(note_key, new_note).unwrap();
    }

    pub fn remove_content(&self, note_key: String, index: usize) {
        let notes = self.db.open_tree("notes").expect("Failed to open tree");
        let old_note = Note::from(notes.remove(note_key.clone()).unwrap().unwrap());
        let new_note = old_note.remove_content(index);
        notes.insert(note_key, new_note).unwrap();
    }

    pub fn get_note_contents(&self, note_key: String) -> String {
        let notes = self.db.open_tree("notes").expect("Failed to open tree");
        let note = Note::from(notes.get(note_key).unwrap().unwrap());
        let contents = note
            .get_contents()
            .iter()
            .map(|content| json!(content).to_string())
            .collect::<Vec<String>>();

        format!("[{}]", contents.join(","))
    }

    pub fn insert_group(&self, parent_group_key: String, group_name: String) {
        let key = Self::time_stamp();
        let groups = self.db.open_tree("groups").expect("Failed to open tree");
        let old_group = Group::from(groups.remove(parent_group_key.clone()).unwrap().unwrap());
        let new_group = old_group.insert_group_key(key.clone());
        groups.insert(parent_group_key, new_group).unwrap();
        groups.insert(key, Group::new(group_name)).unwrap();
    }

    pub fn remove_group(&self, parent_group_key: String, key: String) {
        let groups = self.db.open_tree("groups").expect("Failed to open tree");
        let old_group = Group::from(groups.remove(parent_group_key.clone()).unwrap().unwrap());
        let new_group = old_group.remove_group_key(key.clone());
        groups.insert(parent_group_key, new_group).unwrap();
        groups.remove(key).unwrap();
    }

    pub fn get_group_items(&self, group_key: String) -> String {
        let groups = self.db.open_tree("groups").expect("Failed to open tree");
        let notes = self.db.open_tree("notes").expect("Failed to open tree");
        let group = Group::from(groups.get(group_key).unwrap().unwrap());
        let note_items = group
            .get_note_keys()
            .iter()
            .map(|key| {
                let note = Note::from(notes.get(key).unwrap().unwrap());
                String::from(MenuItem::new(String::from(key), format!("Note: {}", note.get_name())))
            })
            .collect::<Vec<String>>();
        let group_items = group
            .get_group_keys()
            .iter()
            .map(|key| {
                let group = Group::from(groups.get(key).unwrap().unwrap());
                String::from(
                    MenuItem::new(String::from(key), format!("Group: {}", group.get_name()))
                )
            })
            .collect::<Vec<String>>();

        format!("[{}]", [note_items, group_items].concat().join(","))
    }

    pub fn get_groups(&self) -> String {
        fn traverse_tree(group_key: &str, path: &str, groups: &Tree) -> Vec<String> {
            let group = Group::from(groups.get(group_key).unwrap().unwrap());
            let current_path = if path.is_empty() {
                String::from("root")
            } else {
                format!("{}/{}", path, group.get_name())
            };
            let menu_levels = group
                .get_group_keys()
                .iter()
                .flat_map(|key| traverse_tree(key, &current_path, groups))
                .collect::<Vec<_>>();

            [
                vec![json!({"key": group_key, "path": current_path}).to_string()],
                menu_levels,
            ].concat()
        }
        let groups = self.db.open_tree("groups").expect("Failed to open tree");

        format!("[{}]", traverse_tree("root", "", &groups).join(","))
    }

    pub fn is_note(&self, key: String) -> bool {
        let notes = self.db.open_tree("notes").expect("Failed to open tree");
        notes.contains_key(key).unwrap()
    }

    fn time_stamp() -> String {
        let now = SystemTime::now();
        let since_the_epoch = now.duration_since(UNIX_EPOCH).expect("Time went backwards");
        since_the_epoch.as_secs().to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn print_notes(manager: &Manager) {
        manager.db
            .open_tree("notes")
            .unwrap()
            .iter()
            .map(|x| {
                let (k, v) = x.unwrap();
                (String::from_utf8(k.to_vec()).unwrap(), String::from(Note::from(v)))
            })
            .for_each(|(k, v)| println!("{}: {}", k, v));
    }

    fn print_groups(manager: &Manager) {
        manager.db
            .open_tree("groups")
            .unwrap()
            .iter()
            .map(|x| {
                let (k, v) = x.unwrap();
                (String::from_utf8(k.to_vec()).unwrap(), String::from(Group::from(v)))
            })
            .for_each(|(k, v)| println!("{}: {}", k, v));
    }

    fn last_note_key(manager: &Manager) -> String {
        manager.db
            .open_tree("notes")
            .unwrap()
            .iter()
            .map(|x| {
                let (k, _) = x.unwrap();
                String::from_utf8(k.to_vec()).unwrap()
            })
            .last()
            .unwrap()
    }

    fn last_group_key(manager: &Manager) -> String {
        manager.db
            .open_tree("groups")
            .unwrap()
            .iter()
            .map(|x| {
                let (k, _) = x.unwrap();
                String::from_utf8(k.to_vec()).unwrap()
            })
            .filter(|k| k != "root")
            .last()
            .unwrap()
    }

    #[test]
    fn test_manager_insert_note() {
        let manager = Manager::new("../db/test_db");
        manager.clear_db();
        let json = String::from(r#"{"title":"note1","contents":[["活动","描述"]]}"#);
        manager.insert_note(String::from("root"), json);
        print_notes(&manager);
        print_groups(&manager);
    }

    #[test]
    fn test_manager_remove_note() {
        let manager = Manager::new("../db/test_db");
        manager.clear_db();
        let json = String::from(r#"{"title":"note1","contents":[["活动","描述"]]}"#);
        manager.insert_note(String::from("root"), json);
        manager.remove_note(String::from("root"), last_note_key(&manager));
        print_notes(&manager);
        print_groups(&manager);
    }

    #[test]
    fn test_manager_insert_note_content() {
        let manager = Manager::new("../db/test_db");
        manager.clear_db();
        manager.insert_note(String::from("root"), "note1".to_string());
        let json = String::from(r#"["新活动","新描述"]"#);
        manager.insert_content(last_note_key(&manager), json);
        print_notes(&manager);
    }

    #[test]
    fn test_manager_update_note_content() {
        let manager = Manager::new("../db/test_db");
        manager.clear_db();
        manager.insert_note(String::from("root"), "note1".to_string());
        let key = last_note_key(&manager);
        let json = String::from(r#"["活动","描述"]"#);
        manager.insert_content(key.clone(), json);
        print_notes(&manager);

        let json = String::from(r#"["新活动","新描述"]"#);
        manager.update_content(key, 1, json);
        print_notes(&manager);
    }

    #[test]
    fn test_manager_remove_note_content() {
        let manager = Manager::new("../db/test_db");
        manager.clear_db();
        manager.insert_note(String::from("root"), "note1".to_string());
        let key = last_note_key(&manager);
        let json = String::from(r#"["活动","描述"]"#);
        manager.insert_content(key.clone(), json);
        print_notes(&manager);

        manager.remove_content(key, 0);
        print_notes(&manager);
    }

    #[test]
    fn test_manager_insert_group() {
        let manager = Manager::new("../db/test_db");
        manager.clear_db();
        manager.insert_group(String::from("root"), "group1".to_string());
        print_groups(&manager);
    }

    #[test]
    fn test_manager_remove_group() {
        let manager = Manager::new("../db/test_db");
        manager.clear_db();
        manager.insert_group(String::from("root"), "group1".to_string());
        print_groups(&manager);
        manager.remove_group(String::from("root"), last_group_key(&manager));
        print_groups(&manager);
    }

    #[test]
    fn test_manager_menu() {
        let manager = Manager::new("../db/test_db");
        manager.clear_db();
        manager.insert_group(String::from("root"), "group1".to_string());
        let key = last_group_key(&manager);
        manager.insert_note(key.clone(), String::from("note1"));
        println!("{}", manager.get_menu());
    }

    #[test]
    fn test_get_group_items() {
        let manager = Manager::new("../db/test_db");
        manager.clear_db();
        manager.insert_group(String::from("root"), "group1".to_string());
        manager.insert_note(String::from("root"), String::from("note1"));
        println!("{}", manager.get_group_items(String::from("root")));
    }

    #[test]
    fn test_get_groups() {
        let manager = Manager::new("../db/test_db");
        manager.clear_db();
        manager.insert_group(String::from("root"), "group1".to_string());
        let key = last_group_key(&manager);
        manager.insert_group(key.clone(), "group2".to_string());
        println!("{}", manager.get_groups());
    }

    #[test]
    fn test_get_note_contents() {
        let manager = Manager::new("../db/test_db");
        manager.clear_db();
        manager.insert_note(String::from("root"), String::from("note1"));
        let key = last_note_key(&manager);
        println!("{}", manager.get_note_contents(key));
    }

    #[test]
    fn test_path() {
        let exe_path = std::env::current_exe().expect("Failed to get current exe path");
        let install_path = exe_path.parent().expect("Failed to get install path");
        println!("{:?}", format!("{}\\db", install_path.display()));
    }
}
