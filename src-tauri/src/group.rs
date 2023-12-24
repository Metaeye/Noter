use std::collections::HashMap;
use serde::{ Deserialize, Serialize };
use crate::note::Note;

#[derive(Serialize, Deserialize, Debug)]
pub struct Group {
    key: String,
    title: String,
    notes: HashMap<String, Note>,
    groups: HashMap<String, Group>,
}

impl From<String> for Group {
    fn from(json: String) -> Self {
        serde_json::from_str(&json).unwrap()
    }
}

impl From<&Group> for String {
    fn from(group: &Group) -> Self {
        serde_json::to_string(&group).unwrap()
    }
}

impl From<Group> for String {
    fn from(group: Group) -> Self {
        serde_json::to_string(&group).unwrap()
    }
}

impl Group {
    pub fn new(
        key: String,
        title: String,
        notes: HashMap<String, Note>,
        groups: HashMap<String, Group>
    ) -> Self {
        Self { key, title, notes, groups }
    }

    pub fn get_key(&self) -> &str {
        &self.key
    }

    pub fn insert_note(&mut self, key: String, json: String) {
        self.notes.insert(key, Note::from(json));
    }

    pub fn remove_note(&mut self, key: &str) {
        self.notes.remove(key);
    }

    pub fn insert_group(&mut self, key: String, json: String) {
        self.groups.insert(key, Group::from(json));
    }

    pub fn remove_group(&mut self, key: &str) {
        self.groups.remove(key);
    }

    pub fn get_mut_group(&mut self, key: &str) -> Option<&mut Group> {
        if self.key == key {
            return Some(self);
        }
        self.groups.iter_mut().find_map(|(_, group)| group.get_mut_group(key))
    }

    pub fn get_mut_note(&mut self, key: &str) -> Option<&mut Note> {
        let cur_result = self.notes.iter_mut().find(|(_, note)| note.get_key() == key);
        match cur_result {
            Some((_, note)) => Some(note),
            None => self.groups.iter_mut().find_map(|(_, group)| group.get_mut_note(key)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_group_from_string() {
        let json =
            r#"{
            "key": "key",
            "title": "title",
            "notes": {
                "key": {
                    "key": "key",
                    "parent": "parent",
                    "title": "title",
                    "contents": [
                        ["key", "value"]
                    ]
                }
            },
            "groups": {
                "key": {
                    "key": "key",
                    "title": "title",
                    "notes": {},
                    "groups": {}
                }
            }
        }"#;
        let group: Group = Group::from(json.to_string());
        assert_eq!(group.key, "key");
        assert_eq!(group.title, "title");
        assert_eq!(group.notes.len(), 1);
        assert_eq!(group.groups.len(), 1);
    }

    #[test]
    fn test_group_to_string() {
        let group = Group::new(
            "key".to_string(),
            "title".to_string(),
            HashMap::new(),
            HashMap::new()
        );
        let json = String::from(&group);
        assert_eq!(json, r#"{"key":"key","title":"title","notes":{},"groups":{}}"#);
    }

    #[test]
    fn test_group_insert_note() {
        let mut group = Group::new(
            "key".to_string(),
            "title".to_string(),
            HashMap::new(),
            HashMap::new()
        );
        let json =
            r#"{
            "key": "key",
            "parent": "parent",
            "title": "title",
            "contents": [
                ["key", "value"]
            ]
        }"#;
        group.insert_note("key".to_string(), json.to_string());
        assert_eq!(group.notes.len(), 1);
    }

    #[test]
    fn test_group_remove_note() {
        let mut group = Group::new(
            "key".to_string(),
            "title".to_string(),
            HashMap::new(),
            HashMap::new()
        );
        let json =
            r#"{
            "key": "key",
            "parent": "parent",
            "title": "title",
            "contents": [
                ["key", "value"]
            ]
        }"#;
        group.insert_note("key".to_string(), json.to_string());
        group.remove_note("key");
        assert_eq!(group.notes.len(), 0);
    }

    #[test]
    fn test_group_insert_group() {
        let mut group = Group::new(
            "key".to_string(),
            "title".to_string(),
            HashMap::new(),
            HashMap::new()
        );
        let json = r#"{"key":"key","title":"title","notes":{},"groups":{}}"#;
        group.insert_group("key".to_string(), json.to_string());
        assert_eq!(group.groups.len(), 1);
    }

    #[test]
    fn test_group_remove_group() {
        let mut group = Group::new(
            "key".to_string(),
            "title".to_string(),
            HashMap::new(),
            HashMap::new()
        );
        let json = r#"{"key":"key","title":"title","notes":{},"groups":{}}"#;
        group.insert_group("key".to_string(), json.to_string());
        group.remove_group("key");
        assert_eq!(group.groups.len(), 0);
    }

    #[test]
    fn test_group_get_mut_group() {
        let mut group = Group::new(
            "key".to_string(),
            "title".to_string(),
            HashMap::new(),
            HashMap::new()
        );
        let json = r#"{"key":"key","title":"title","notes":{},"groups":{}}"#;
        group.insert_group("key".to_string(), json.to_string());
        let result = group.get_mut_group("key");
        assert_eq!(result.unwrap().key, "key");
    }

    #[test]
    fn test_group_get_mut_note() {
        let mut group = Group::new(
            "key".to_string(),
            "title".to_string(),
            HashMap::new(),
            HashMap::new()
        );
        let json =
            r#"{
            "key": "key",
            "parent": "parent",
            "title": "title",
            "contents": [
                ["key", "value"]
            ]
        }"#;
        group.insert_note("key".to_string(), json.to_string());
        let result = group.get_mut_note("key");
        assert_eq!(result.unwrap().get_key(), "key");
    }
}
