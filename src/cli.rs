use std::io::{self, Write};

pub fn prompt_for_project_folder() -> String {
    print!("Enter project folder name: ");
    io::stdout().flush().unwrap();
    let mut folder = String::new();
    io::stdin().read_line(&mut folder).expect("Failed to read folder name");
    folder.trim().to_string()
}

pub fn prompt_for_api_key() -> String {
    print!("Enter your HUGGING FACE API key: ");
    io::stdout().flush().unwrap();
    let mut api_key = String::new();
    io::stdin().read_line(&mut api_key).expect("Failed to read API key");
    api_key.trim().to_string()
}

pub fn prompt_for_model() -> String {
    print!("Choose a Hugging Face model (e.g., 'gpt2', 'facebook/bart-large-cnn', 'EleutherAI/gpt-neo-2.7B'): ");
    io::stdout().flush().unwrap();
    let mut model = String::new();
    io::stdin().read_line(&mut model).expect("Failed to read model name");
    model.trim().to_string()
}
