use std::{io::{self, Write}, path::Path};

pub fn ask(prompt: &str) -> String {
    println!("{prompt}:");

    print!("> ");
    io::stdout().flush().unwrap();

    let mut ans = String::new();

    io::stdin()
        .read_line(&mut ans)
        .unwrap();

    ans.trim_end().to_string()
}

pub fn ask_enter() {
    let mut exit_input = String::new();

    println!(" ");
    print!("Press [Enter] to continue...");
    io::stdout().flush().unwrap();

    io::stdin()
        .read_line(&mut exit_input)
        .unwrap();
}

pub fn get_file_extension(file_name: &str) -> &str {
    let ext = Path::new(file_name)
                            .extension()
                            .unwrap();
    
    ext.to_str().unwrap()
} 