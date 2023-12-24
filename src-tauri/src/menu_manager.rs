use std::collections::HashMap;
use serde::{ Deserialize, Serialize };
use crate::group::Group;

#[derive(Serialize, Deserialize, Debug)]
pub struct MenuManager {
    root: Group,
}

impl Default for MenuManager {
    fn default() -> Self {
        Self {
            root: Group::new(
                "root".to_string(),
                "root".to_string(),
                HashMap::new(),
                HashMap::new()
            ),
        }
    }
}

impl MenuManager {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn push_content(&mut self, parent: &str, json: String) {
        self.root.get_mut_note(parent).unwrap().push_content(json);
    }

    pub fn remove_content(&mut self, parent: &str, index: usize) {
        self.root.get_mut_note(parent).unwrap().remove_content(index);
    }

    pub fn insert_note(&mut self, parent: &str, key: String, json: String) {
        self.root.get_mut_group(parent).unwrap().insert_note(key, json);
    }

    pub fn remove_note(&mut self, parent: &str, key: &str) {
        self.root.get_mut_group(parent).unwrap().remove_note(key);
    }

    pub fn insert_group(&mut self, parent: &str, key: String, json: String) {
        self.root.get_mut_group(parent).unwrap().insert_group(key, json);
    }

    pub fn remove_group(&mut self, parent: &str, key: &str) {
        self.root.get_mut_group(parent).unwrap().remove_group(key);
    }

    pub fn serialize(&self) -> String {
        String::from(&self.root)
    }

    pub fn deserialize(&mut self, json: String) {
        self.root = Group::from(json);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::{ SystemTime, UNIX_EPOCH };
    use crate::note::Note;

    fn time_stamp() -> String {
        SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos().to_string()
    }

    #[test]
    fn test_menu_manager_insert_group() {
        let mut menu_manager = MenuManager::new();
        menu_manager.insert_group(
            "root",
            "key".to_string(),
            r#"{"key": "key", "title": "title", "notes": {}, "groups": {}}"#.to_string()
        );
        assert_eq!(
            menu_manager.serialize(),
            r#"{"key":"root","title":"root","notes":{},"groups":{"key":{"key":"key","title":"title","notes":{},"groups":{}}}}"#.to_string()
        );
    }

    #[test]
    fn test_init_root() {
        let contents = vec![
            ("activity1".to_string(), "description1".to_string()),
            ("activity2".to_string(), "description2".to_string())
        ];

        let mut notes = HashMap::new();
        let key = time_stamp();
        let parent = time_stamp();
        notes.insert(
            key.clone(),
            Note::new(key, parent.clone(), "add".to_string(), contents.clone())
        );
        let group_add = Group::new(parent, "add".to_string(), notes, HashMap::new());

        let mut notes = HashMap::new();
        let key = time_stamp();
        let parent = time_stamp();
        notes.insert(
            key.clone(),
            Note::new(key, parent.clone(), "algebra".to_string(), contents.clone())
        );
        let mut groups = HashMap::new();
        groups.insert(group_add.get_key().to_string(), group_add);
        let group_algebra = Group::new(parent, "algebra".to_string(), notes, groups);

        let mut notes = HashMap::new();
        let key = time_stamp();
        let parent = time_stamp();
        notes.insert(
            key.clone(),
            Note::new(key, parent.clone(), "math".to_string(), contents.clone())
        );
        let mut groups = HashMap::new();
        groups.insert(group_algebra.get_key().to_string(), group_algebra);
        let group_math = Group::new(parent, "math".to_string(), notes, groups);

        let note1 = Note::new(
            time_stamp(),
            "root".to_string(),
            "gift".to_string(),
            contents.clone()
        );

        let mut root = Group::new(
            "root".to_string(),
            "root".to_string(),
            HashMap::new(),
            HashMap::new()
        );
        root.insert_note(note1.get_key().to_string(), String::from(note1));
        root.insert_group(group_math.get_key().to_string(), String::from(group_math));

        // println!("{}", String::from(&root));
    }
}
