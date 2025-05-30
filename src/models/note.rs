use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Note<'a> {
    title: &'a str,
    content: &'a str,
    tags: Vec<&'a str>
}

impl<'a> Note<'a> {
    pub fn new(title: &'a str, content: &'a str, tags: Vec<&'a str>) -> Self {
        Self {
            title,
            content,
            tags,
        }
    }

    pub fn title(&self) -> &str {
        self.title
    }
}