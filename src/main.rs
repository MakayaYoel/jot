use std::io::{self, Write};

use clearscreen::ClearScreen;

mod commands;
mod models;
mod utils;

fn main() {
    loop {
        ClearScreen::default().clear().unwrap();
        io::stdout().flush().unwrap();

        let mut choice = String::new();

        println!("jot - a simple note taking app\n\nWhat would you like to do:\ncreate - create a note\nlist - see all notes\n");
        print!("CHOICE: ");

        io::stdout().flush().unwrap();

        io::stdin()
            .read_line(&mut choice)
            .unwrap();

        let choice = choice.trim();

        let valid_choices = vec!["create", "list"];

        if !valid_choices.iter().any(|c| c == &choice.to_lowercase()) {
            continue;
        }

        ClearScreen::default().clear().unwrap();

        if choice == "create" {
            commands::create_note();
        } else if choice == "list" {
            commands::list();
        }
    }
}
