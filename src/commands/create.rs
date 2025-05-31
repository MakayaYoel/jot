use crate::models::Note;
use crate::utils;

use std::{fs::{self, File}, io::Write};

pub fn create_note() {
    let title = utils::ask("Enter note title");
    let content = utils::ask("Enter note content");
    let tags = utils::ask("Enter tags (comma-seperated, optional)").replace(" ", "");

    let tags: Vec<String> = tags.split(",").map(|val| val.to_string()).collect();

    // TODO: make some kind of "next id" thing
    let note = Note::new(title, content, tags);
    let serialized_note = serde_json::to_string(&note).unwrap();

    if !fs::exists("assets").unwrap() {
        fs::create_dir("assets").unwrap();
    }

    let file_path = format!("assets/{}.json", note.title().replace(" ", "_"));

    if fs::exists(&file_path).unwrap() {
        println!("\nThat note already exists!");
        utils::ask_enter();
        return;
    }

    let mut new_file = File::create(file_path).unwrap();
    new_file.write(serialized_note.as_bytes()).unwrap();
}