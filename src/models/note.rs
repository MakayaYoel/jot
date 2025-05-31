use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Note {
    title: String,
    content: String,
    tags: Vec<String>
}

impl Note {
    pub fn new(title: String, content: String, tags: Vec<String>) -> Self {
        Self {
            title,
            content,
            tags,
        }
    }

    pub fn title(&self) -> &String {
        &self.title
    }
}