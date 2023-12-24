use std::collections::HashMap;
use std::time::{ SystemTime, UNIX_EPOCH };
use serde::{ Deserialize, Serialize };

#[derive(Serialize, Deserialize, Debug)]
pub struct Note {
    key: String,
    parent: String,
    title: String,
    content: Vec<(String, String)>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Group {
    key: String,
    title: String,
    notes: HashMap<String, Note>,
    groups: HashMap<String, Group>,
}

pub struct MenuManager {
    root: Group,
}

impl Note {
    pub fn new(key: String, parent: String, title: String, content: Vec<(String, String)>) -> Self {
        Self { key, parent, title, content }
    }
}

impl Group {
    fn new(
        key: String,
        title: String,
        notes: HashMap<String, Note>,
        groups: HashMap<String, Group>
    ) -> Self {
        Self { key, title, notes, groups }
    }

    fn add_group(&mut self, key: String, group: Group) {
        self.groups.insert(key, group);
    }

    fn add_note(&mut self, key: String, note: Note) {
        self.notes.insert(key, note);
    }
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

    fn time_stamp() -> String {
        SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos().to_string()
    }

    pub fn test_init_root(&mut self) {
        let content = vec![
            ("activity1".to_string(), "description1".to_string()),
            ("activity2".to_string(), "description2".to_string())
        ];

        let mut notes = HashMap::new();
        let key = MenuManager::time_stamp();
        let parent = MenuManager::time_stamp();
        notes.insert(
            key.clone(),
            Note::new(key, parent.clone(), "add".to_string(), content.clone())
        );
        let group_add = Group::new(parent, "add".to_string(), notes, HashMap::new());

        let mut notes = HashMap::new();
        let key = MenuManager::time_stamp();
        let parent = MenuManager::time_stamp();
        notes.insert(
            key.clone(),
            Note::new(key, parent.clone(), "algebra".to_string(), content.clone())
        );
        let mut groups = HashMap::new();
        groups.insert(group_add.key.clone(), group_add);
        let group_algebra = Group::new(parent, "algebra".to_string(), notes, groups);

        let mut notes = HashMap::new();
        let key = MenuManager::time_stamp();
        let parent = MenuManager::time_stamp();
        notes.insert(
            key.clone(),
            Note::new(key, parent.clone(), "math".to_string(), content.clone())
        );
        let mut groups = HashMap::new();
        groups.insert(group_algebra.key.clone(), group_algebra);
        let group_math = Group::new(parent, "math".to_string(), notes, groups);

        let note1 = Note::new(
            MenuManager::time_stamp(),
            "root".to_string(),
            "gift".to_string(),
            content.clone()
        );
        
        self.root.add_note(note1.key.clone(), note1);
        self.root.add_group(group_math.key.clone(), group_math);
    }

    pub fn serialize(&self) -> String {
        serde_json::to_string(&self.root).unwrap()
    }

    pub fn deserialize(&mut self, json: String) {
        self.root = serde_json::from_str(&json).unwrap();
    }
}
