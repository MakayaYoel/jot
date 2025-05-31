use std::fs;

use crate::{models::Note, utils};

pub fn get_notes() -> Vec<Note> {
    let mut notes: Vec<Note> = Vec::new();

    for entry in fs::read_dir("assets").unwrap() {
        // error handling's for dweebs.
        let entry = entry.unwrap();
        let file_name = entry.file_name()
                                     .into_string()
                                     .unwrap();

        if !entry.file_type().unwrap().is_file() || utils::get_file_extension(&file_name) != "json" {
            continue;
        }

        let contents = fs::read_to_string(format!("assets/{file_name}")).unwrap();
        let Ok(note) = serde_json::from_str::<Note>(&contents) else {
            continue;
        };

        notes.push(note);
    }

    notes
}