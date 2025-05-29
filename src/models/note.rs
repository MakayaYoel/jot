use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Note {
    id: u64,
    content: String
}

impl Note {
    pub fn new(id: u64, content: String) -> Self {
        Self {
            id,
            content
        }
    }

    pub fn id(&self) -> u64 {
        self.id
    }

    pub fn content(&self) -> &String {
        &self.content
    }
}