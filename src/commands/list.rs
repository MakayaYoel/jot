use std::fs;

use crate::{storage, utils};

pub fn list() {
    let notes = storage::get_notes();

    println!("Available Notes ({}):", notes.len());
    for n in notes {
        println!("- {}", n.title());
    }

    utils::ask_enter();
}