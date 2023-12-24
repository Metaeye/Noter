use serde::{ Deserialize, Serialize };

#[derive(Serialize, Deserialize, Debug)]
pub struct Note {
    key: String,
    parent: String,
    title: String,
    contents: Vec<(String, String)>,
}

impl From<String> for Note {
    fn from(json: String) -> Self {
        serde_json::from_str(&json).unwrap()
    }
}

impl From<&Note> for String {
    fn from(note: &Note) -> Self {
        serde_json::to_string(&note).unwrap()
    }
}

impl From<Note> for String {
    fn from(note: Note) -> Self {
        serde_json::to_string(&note).unwrap()
    }
}

impl Note {
    pub fn new(key: String, parent: String, title: String, content: Vec<(String, String)>) -> Self {
        Self { key, parent, title, contents: content }
    }

    pub fn get_key(&self) -> &str {
        &self.key
    }

    pub fn push_content(&mut self, json: String) {
        self.contents.push(serde_json::from_str(&json).unwrap());
    }

    pub fn remove_content(&mut self, index: usize) {
        self.contents.remove(index);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_note_from_string() {
        let json =
            r#"{
            "key": "key",
            "parent": "parent",
            "title": "title",
            "contents": [
                ["key", "value"]
            ]
        }"#;
        let note: Note = Note::from(json.to_string());
        assert_eq!(note.key, "key");
        assert_eq!(note.parent, "parent");
        assert_eq!(note.title, "title");
        assert_eq!(note.contents, vec![("key".to_string(), "value".to_string())]);
    }

    #[test]
    fn test_note_to_string() {
        let note = Note::new(
            "key".to_string(),
            "parent".to_string(),
            "title".to_string(),
            vec![("key".to_string(), "value".to_string())]
        );
        let json: String = note.into();
        assert_eq!(
            json,
            r#"{"key":"key","parent":"parent","title":"title","contents":[["key","value"]]}"#
        );
    }

    #[test]
    fn test_string_to_contents() {
        let json = r#"["activity","description"]"#;
        let content: (String, String) = serde_json::from_str(json).unwrap();
        assert_eq!(content, ("activity".to_string(), "description".to_string()));
    }

    #[test]
    fn test_note_push_content() {
        let mut note = Note::new(
            "key".to_string(),
            "parent".to_string(),
            "title".to_string(),
            vec![("key".to_string(), "value".to_string())]
        );
        note.push_content(r#"["activity","description"]"#.to_string());
        assert_eq!(
            note.contents,
            vec![
                ("key".to_string(), "value".to_string()),
                ("activity".to_string(), "description".to_string())
            ]
        );
    }
}
