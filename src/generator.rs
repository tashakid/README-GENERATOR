use std::fs;
use std::path::Path;
use serde_json::json;
use std::thread;
use std::time::Duration;
use reqwest::blocking::Client;

pub fn generate_readme(project_dir: &str, api_key: &str, model: &str) -> Result<String, Box<dyn std::error::Error>> {
    let project_info = gather_project_info(project_dir)?;
    let readme_content = generate_readme_with_huggingface(&project_info, api_key, model)?;
    Ok(readme_content)
}

fn gather_project_info(project_dir: &str) -> Result<String, std::io::Error> {
    let mut info = String::new();

    info.push_str(&format!("Project Name: {}\n\n", Path::new(project_dir).file_name().unwrap().to_string_lossy()));

    info.push_str("Project Structure:\n");
    for entry in fs::read_dir(project_dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() || path.is_dir() {
            info.push_str(&format!("- {}\n", path.file_name().unwrap().to_string_lossy()));
        }
    }

    let files_to_check = vec!["Cargo.toml", "package.json", "requirements.txt", "setup.py"];
    for file in files_to_check {
        if let Ok(content) = fs::read_to_string(Path::new(project_dir).join(file)) {
            info.push_str(&format!("\nContents of {file}:\n{content}\n"));
        }
    }

    Ok(info)
}

fn generate_readme_with_huggingface(project_info: &str, api_key: &str, model: &str) -> Result<String, Box<dyn std::error::Error>> {
    println!("Generating README using the Hugging Face model: {}...", model);

    let prompt = format!(
        "Generate a comprehensive README.md file for the following project. Include sections for Project Name, Description, Installation, Usage, Features, Contributing, and License. Use proper Markdown formatting.\n\nProject Information:\n{}\n\nREADME.md:",
        project_info
    );

    for attempt in 1..=3 {
        match try_generate_readme_huggingface(&prompt, api_key, model) {
            Ok(content) => return Ok(content),
            Err(e) => {
                eprintln!("Error on attempt {}: {}. Retrying...", attempt, e);
                thread::sleep(Duration::from_secs(5));
            }
        }
    }

    Err("Failed to generate README after multiple attempts".into())
}

fn try_generate_readme_huggingface(prompt: &str, api_key: &str, model: &str) -> Result<String, Box<dyn std::error::Error>> {
    let client = Client::new();
    let response = client.post(format!("https://api-inference.huggingface.co/models/{}", model))
        .header("Authorization", format!("Bearer {}", api_key))
        .header("Content-Type", "application/json")
        .json(&json!({
            "inputs": prompt,
            "parameters": {
                "max_new_tokens": 1000,
                "temperature": 0.7,
                "top_p": 0.95,
                "do_sample": true
            }
        }))
        .send()?;

    let response_text = response.text()?;
    println!("API Response: {}", response_text); // Debug print
    
    let response_json: serde_json::Value = serde_json::from_str(&response_text)?;

    if let Some(error) = response_json.get("error") {
        return Err(format!("API Error: {}", error).into());
    }

    if let Some(generated_text) = response_json[0]["generated_text"].as_str() {
        Ok(generated_text.trim().to_string())
    } else {
        Err(format!("Unexpected API response format: {}", response_text).into())
    }
}
