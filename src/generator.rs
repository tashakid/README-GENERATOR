use std::fs;
use std::path::Path;
use serde_json::json;
use reqwest::blocking::Client;
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_TYPE};

pub fn generate_readme(project_dir: &str, github_token: &str, model_name: &str) -> Result<String, Box<dyn std::error::Error>> {
    let project_info = gather_project_info(project_dir)?;
    let readme_content = generate_readme_with_model(&project_info, github_token, model_name)?;
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

fn generate_readme_with_model(project_info: &str, github_token: &str, model_name: &str) -> Result<String, Box<dyn std::error::Error>> {
    println!("Generating README using {}...", model_name);

    let endpoint = "https://models.inference.ai.azure.com";

    let mut headers = HeaderMap::new();
    headers.insert(AUTHORIZATION, HeaderValue::from_str(&format!("Bearer {}", github_token))?);
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));

    let client = Client::builder()
        .default_headers(headers)
        .build()?;

    let system_message = "You are an expert README generator. Your task is to create comprehensive, well-structured README files for software projects. Include all necessary sections such as Project Name, Description, Installation, Usage, Features, Contributing, and License. Use proper Markdown formatting.";

    let prompt = format!(
        "Generate a README.md file for the following project:\n\nProject Information:\n{}\n\nREADME.md:",
        project_info
    );

    let response = client.post(format!("{}/v1/chat/completions", endpoint))
        .json(&json!({
            "messages": [
                {"role": "system", "content": system_message},
                {"role": "user", "content": prompt}
            ],
            "model": model_name,
            "temperature": 0.7,
            "max_tokens": 1000,
            "top_p": 0.95
        }))
        .send()?;

    if !response.status().is_success() {
        return Err(format!("API error: {}", response.status()).into());
    }

    let response_json: serde_json::Value = response.json()?;
    
    if let Some(content) = response_json["choices"][0]["message"]["content"].as_str() {
        Ok(content.trim().to_string())
    } else {
        Err("Unexpected API response format".into())
    }
}