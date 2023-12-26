use sled::IVec;
use serde::{ Deserialize, Serialize };

#[derive(Serialize, Deserialize)]
pub struct Note {
    name: String,
    contents: Vec<(String, String)>,
}

impl Note {
    pub fn new(name: String) -> Self {
        Self {
            name,
            contents: vec![(String::from("Activity"), String::from("Description"))],
        }
    }

    pub fn get_name(&self) -> &String {
        &self.name
    }

    pub fn get_contents(&self) -> &Vec<(String, String)> {
        &self.contents
    }

    pub fn insert_content(mut self, json_content: String) -> Self {
        self.contents.push(Note::json_to_content(&json_content));
        self
    }

    pub fn update_content(mut self, index: usize, json_content: String) -> Self {
        self.contents[index] = Note::json_to_content(&json_content);
        self
    }

    pub fn remove_content(mut self, index: usize) -> Self {
        self.contents.remove(index);
        self
    }

    fn json_to_content(json_content: &str) -> (String, String) {
        serde_json::from_str(json_content).unwrap()
    }
}

impl From<String> for Note {
    fn from(json: String) -> Self {
        serde_json::from_str(&json).unwrap()
    }
}

impl From<Note> for String {
    fn from(note: Note) -> Self {
        serde_json::to_string(&note).unwrap()
    }
}

impl From<Note> for IVec {
    fn from(note: Note) -> Self {
        String::from(note).into_bytes().into()
    }
}

impl From<IVec> for Note {
    fn from(ivec: IVec) -> Self {
        String::from_utf8(ivec.to_vec()).unwrap().into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_note_size() {
        let note_size = std::mem::size_of::<Note>();
        println!("Note size: {} bytes", note_size);
    }

    #[test]
    fn test_note() {
        let db = sled::open("my_db").expect("Failed to open database");
        let notes = db.open_tree("test_notes").expect("Failed to open tree");
        let note = Note::new(String::from("note1"));
        let note = note.insert_content(String::from(r#"["content1", "content2"]"#));
        notes.insert("note1", IVec::from(note)).unwrap();

        println!("{}", String::from(Note::from(notes.get("note1").unwrap().unwrap())));
    }
}
