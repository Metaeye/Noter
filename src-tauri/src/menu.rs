use serde::Serialize;
use sled::Tree;
use crate::{ group::Group, note::Note };

#[derive(Serialize)]
pub struct MenuItem {
    key: String,
    name: String,
}

#[derive(Serialize)]
pub struct Menu {
    key: String,
    name: String,
    items: Vec<MenuItem>,
    submenus: Vec<Menu>,
}

impl From<MenuItem> for String {
    fn from(menu_item: MenuItem) -> Self {
        serde_json::to_string(&menu_item).unwrap()
    }
}

impl From<Menu> for String {
    fn from(menu: Menu) -> Self {
        serde_json::to_string(&menu).unwrap()
    }
}

impl MenuItem {
    pub fn new(key: String, name: String) -> Self {
        Self { key, name }
    }
}

impl Menu {
    pub fn new(root: Group, key: &str, groups: &Tree, notes: &Tree) -> Self {
        let items: Vec<MenuItem> = root
            .get_note_keys()
            .iter()
            .map(|key| {
                let note = Note::from(notes.get(key).unwrap().unwrap());
                MenuItem::new(key.to_string(), note.get_name().to_string())
            })
            .collect();

        let submenus: Vec<Menu> = root
            .get_group_keys()
            .iter()
            .map(|key| {
                let group = Group::from(groups.get(key).unwrap().unwrap());
                Menu::new(group, key, groups, notes)
            })
            .collect();

        Self {
            key: key.to_string(),
            name: root.get_name().to_string(),
            items,
            submenus,
        }
    }
}
