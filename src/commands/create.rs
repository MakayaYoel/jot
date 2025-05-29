use std::io::{self, Write};
use crate::models::Note;
use std::fs::File;

pub fn create_note() {
    println!("Enter note content:");

    print!("> ");
    io::stdout().flush().unwrap();

    let mut content = String::new();

    io::stdin()
        .read_line(&mut content)
        .unwrap();

    let content = content.trim_end().to_string();

    // println!("\nEnter tags (comma-seperated, optional):");

    // print!("> ");
    // io::stdout().flush().unwrap();

    // let mut tags = String::new();

    // io::stdin()
    //     .read_line(&mut tags)
    //     .unwrap();

    // let tags: Vec<&str> = tags.replace(" ", "")
    //                           .split(",")
    //                           .collect();

    // TODO: make some kind of "next id" thing
    let note = Note::new(0, content);
    let serialized_note = serde_json::to_string(&note).unwrap();

    // testing for now
    let mut new_file = File::create("note.json").unwrap();
    new_file.write_all(serialized_note.as_bytes()).unwrap();
}