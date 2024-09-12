use std::io::{self, Write};

pub fn prompt_for_project_folder() -> String {
    print!("Enter project folder name: ");
    io::stdout().flush().unwrap();
    let mut folder = String::new();
    io::stdin().read_line(&mut folder).expect("Failed to read folder name");
    folder.trim().to_string()
}

pub fn prompt_for_github_token() -> String {
    print!("Enter your GitHub token: ");
    io::stdout().flush().unwrap();
    let mut token = String::new();
    io::stdin().read_line(&mut token).expect("Failed to read GitHub token");
    token.trim().to_string()
}