use std::io::{self, Write};
use std::env;

pub fn prompt_for_project_folder() -> String {
    print!("Enter project folder name: ");
    io::stdout().flush().unwrap();
    let mut folder = String::new();
    io::stdin().read_line(&mut folder).expect("Failed to read folder name");
    folder.trim().to_string()
}

pub fn get_github_token() -> String {
    match env::var("GITHUB_TOKEN") {
        Ok(token) => token,
        Err(_) => {
            println!("GITHUB_TOKEN environment variable not found. Please enter your GitHub token:");
            let mut token = String::new();
            io::stdin().read_line(&mut token).expect("Failed to read GitHub token");
            token.trim().to_string()
        }
    }
}