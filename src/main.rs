mod generator;
mod cli;

use std::fs;
use std::path::Path;
use colored::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    print_banner();
    println!("{}", "Generates a README for your project using GPT-4o-mini model".bright_blue());
    println!();

    let project_dir = cli::prompt_for_project_folder();
    let github_token = cli::get_github_token();
    
    println!("Generating README... This may take a moment.");
    
    match generator::generate_readme(&project_dir, &github_token).await {
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