mod generator;
mod cli;

use std::fs;
use std::path::Path;
use colored::*;

fn print_banner() {
    println!("{}", r#"
 _____               _  _____            __ _   
|  __ \             | |/ ____|          / _| |  
| |__) |___  __ _  _| | |     _ __ __ _| |_| |_ 
|  _  // _ \/ _` |/ _` | |    | '__/ _` |  _| __|
| | \ \  __/ (_| | (_| | |____| | | (_| | | | |_ 
|_|  \_\___|\__,_|\__,_|\_____|_|  \__,_|_|  \__|
                                                 
"#.bright_yellow());
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    print_banner();
    println!("{}", "Generates a README for your project using Hugging Face API".bright_blue());
    println!();

    let project_dir = cli::prompt_for_project_folder();
    let api_key = cli::prompt_for_api_key();
    
    println!("Choose a Hugging Face model (e.g., 'gpt2', 'facebook/bart-large-cnn', 'EleutherAI/gpt-neo-2.7B'):");
    let model = cli::prompt_for_model();
    
    println!("Generating README... This may take a moment.");
    
    match generator::generate_readme(&project_dir, &api_key, &model) {
        Ok(readme_content) => {
            println!("\nGenerated README content:\n");
            println!("{}", readme_content);
            
            println!("\nDo you want to save this README? (y/n)");
            let mut input = String::new();
            std::io::stdin().read_line(&mut input)?;
            
            if input.trim().to_lowercase() == "y" {
                fs::write(Path::new(&project_dir).join("README.md"), readme_content)?;
                println!("\nREADME.md has been saved successfully!");
            } else {
                println!("\nREADME.md was not saved.");
            }
        },
        Err(e) => {
            eprintln!("Error generating README: {}", e);
        }
    }

    Ok(())
}
