use serde::Serialize;
use sled::Tree;
use crate::{ group::Group, note::Note };

#[derive(Serialize)]
pub struct MenuItem {
    key: String,
    title: String,
}

#[derive(Serialize)]
pub struct Menu {
    key: String,
    title: String,
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
    pub fn new(key: String, title: String) -> Self {
        Self { key, title }
    }
}

impl Menu {
    pub fn new(root: Group, key: &str, groups: &Tree, notes: &Tree) -> Self {
        let items: Vec<MenuItem> = root
            .get_note_keys()
            .iter()
            .map(|key| {
                let note = Note::from(notes.get(key).unwrap().unwrap());
                MenuItem::new(key.to_string(), note.get_title().to_string())
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
            title: root.get_name().to_string(),
            items,
            submenus,
        }
    }
}
