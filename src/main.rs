use std::io::{self, Write};

use clearscreen::ClearScreen;

mod commands;
mod models;

fn main() {
    loop {
        ClearScreen::default().clear().unwrap();

        let mut choice = String::new();

        println!("jot - a simple note taking app\n\nWhat would you like to do:\ncreate - create a note\n");
        print!("CHOICE: ");

        io::stdout().flush().unwrap();

        io::stdin()
            .read_line(&mut choice)
            .unwrap();

        let choice = choice.trim();

        let valid_choices = vec!["create"];

        if !valid_choices.iter().any(|c| c == &choice.to_lowercase()) {
            continue;
        }

        ClearScreen::default().clear().unwrap();

        if choice == "create" {
            commands::create_note();
        }
    }
}
