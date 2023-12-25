use std::collections::HashSet;
use sled::IVec;
use serde::{ Deserialize, Serialize };

#[derive(Serialize, Deserialize)]
pub struct Group {
    name: String,
    note_keys: HashSet<String>,
    group_keys: HashSet<String>,
}

impl Group {
    pub fn new(name: String) -> Self {
        Self {
            name,
            note_keys: HashSet::new(),
            group_keys: HashSet::new(),
        }
    }

    pub fn get_name(&self) -> &String {
        &self.name
    }

    pub fn get_note_keys(&self) -> &HashSet<String> {
        &self.note_keys
    }

    pub fn get_group_keys(&self) -> &HashSet<String> {
        &self.group_keys
    }

    pub fn insert_note_key(mut self, note_key: String) -> Self {
        self.note_keys.insert(note_key);
        self
    }

    pub fn remove_note_key(mut self, note_key: String) -> Self {
        self.note_keys.remove(&note_key);
        self
    }

    pub fn insert_group_key(mut self, group_key: String) -> Self {
        self.group_keys.insert(group_key);
        self
    }

    pub fn remove_group_key(mut self, group_key: String) -> Self {
        self.group_keys.remove(&group_key);
        self
    }
}

impl From<String> for Group {
    fn from(json: String) -> Self {
        serde_json::from_str(&json).unwrap()
    }
}

impl From<Group> for String {
    fn from(group: Group) -> Self {
        serde_json::to_string(&group).unwrap()
    }
}

impl From<Group> for IVec {
    fn from(group: Group) -> Self {
        String::from(group).into_bytes().into()
    }
}

impl From<IVec> for Group {
    fn from(ivec: IVec) -> Self {
        String::from_utf8(ivec.to_vec()).unwrap().into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_group_size() {
        let group_size = std::mem::size_of::<Group>();
        println!("Group size: {} bytes", group_size);
    }
}
