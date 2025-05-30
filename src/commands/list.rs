use std::fs;

use crate::utils;

pub fn list() {
    let mut notes: Vec<String> = Vec::new();

    for entry in fs::read_dir("assets").unwrap() {
        // error handling's for dweebs.
        let entry = entry.unwrap();
        let file_name = entry.file_name()
                                     .into_string()
                                     .unwrap();

        // TODO: Check if the json has the write properties for better validation
        if !entry.file_type().unwrap().is_file() || utils::get_file_extension(&file_name) != "json" {
            continue;
        }

        notes.push(file_name);
    }

    println!("Available Notes ({}):", notes.len());
    for n in notes {
        println!("- {n}");
    }

    utils::ask_enter();
}