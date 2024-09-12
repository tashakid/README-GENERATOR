use std::fs;
use std::path::{Path, PathBuf};
use serde_json::json;
use reqwest::Client;
use base64;

const ENDPOINT: &str = "https://models.inference.ai.azure.com";
const MODEL_NAME: &str = "gpt-4o-mini";

pub async fn generate_readme(project_dir: &str, github_token: &str) -> Result<String, Box<dyn std::error::Error>> {
    let (project_info, image_paths) = gather_project_info(project_dir)?;
    let image_descriptions = analyze_images(&image_paths, github_token).await?;
    let readme_content = generate_readme_with_model(&project_info, &image_descriptions, github_token).await?;
    Ok(readme_content)
}

fn gather_project_info(project_dir: &str) -> Result<(String, Vec<PathBuf>), std::io::Error> {
    let mut info = String::new();
    let mut image_paths = Vec::new();

    info.push_str(&format!("Project Name: {}\n\n", Path::new(project_dir).file_name().unwrap().to_string_lossy()));
    info.push_str("Project Structure and Contents:\n");

    gather_dir_info(Path::new(project_dir), Path::new(project_dir), &mut info, &mut image_paths, 0)?;

    Ok((info, image_paths))
}

fn gather_dir_info(base_dir: &Path, current_dir: &Path, info: &mut String, image_paths: &mut Vec<PathBuf>, depth: usize) -> std::io::Result<()> {
    for entry in fs::read_dir(current_dir)? {
        let entry = entry?;
        let path = entry.path();
        let file_name = path.file_name().unwrap().to_string_lossy();

        if file_name.starts_with('.') {
            continue;
        }

        let indent = "  ".repeat(depth);
        info.push_str(&format!("{}{}\n", indent, file_name));

        if path.is_dir() {
            gather_dir_info(base_dir, &path, info, image_paths, depth + 1)?;
        } else if path.is_file() {
            if is_image_file(&path) {
                let relative_path = path.strip_prefix(base_dir).unwrap();
                image_paths.push(relative_path.to_path_buf());
                info.push_str(&format!("{}[IMAGE]: {}\n", indent, relative_path.display()));
            } else if is_text_file(&path) {
                if let Ok(metadata) = fs::metadata(&path) {
                    if metadata.len() < 10000 {
                        if let Ok(content) = fs::read_to_string(&path) {
                            info.push_str(&format!("{}Content of {}:\n", indent, file_name));
                            info.push_str(&format!("{}{}\n", indent, content.lines().take(50).collect::<Vec<_>>().join(&format!("\n{}", indent))));
                        }
                    }
                }
            }
        }
    }
    Ok(())
}

fn is_text_file(path: &Path) -> bool {
    let extension = path.extension().and_then(|e| e.to_str()).unwrap_or("");
    matches!(extension.to_lowercase().as_str(), "txt" | "md" | "rs" | "toml" | "json" | "yaml" | "yml")
}

fn is_image_file(path: &Path) -> bool {
    let extension = path.extension().and_then(|e| e.to_str()).unwrap_or("");
    matches!(extension.to_lowercase().as_str(), "jpg" | "jpeg" | "png" | "gif" | "bmp" | "svg")
}

async fn analyze_images(image_paths: &[PathBuf], github_token: &str) -> Result<String, Box<dyn std::error::Error>> {
    let client = Client::new();

    let mut descriptions = Vec::new();

    for path in image_paths {
        let image_data = fs::read(path)?;
        let base64_image = base64::encode(&image_data);

        let response = client.post(format!("{}/v1/chat/completions", ENDPOINT))
            .header("Authorization", format!("Bearer {}", github_token))
            .json(&json!({
                "messages": [
                    {
                        "role": "system",
                        "content": "You are a helpful assistant that describes images in details.",
                    },
                    {
                        "role": "user",
                        "content": [
                            {
                                "type": "text",
                                "text": "Describe this image concisely in the context of a software project README.",
                            },
                            {
                                "type": "image_url",
                                "image_url": {
                                    "url": format!("data:image/jpeg;base64,{}", base64_image),
                                    "detail": "low"
                                },
                            },
                        ],
                    },
                ],
                "model": MODEL_NAME,
            }))
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(format!("API error: {} - {}", response.status(), response.text().await?).into());
        }

        let response_json: serde_json::Value = response.json().await?;

        if let Some(error) = response_json.get("error") {
            return Err(format!("API error: {}", error).into());
        }

        match response_json.get("choices").and_then(|choices| choices.get(0)).and_then(|choice| choice.get("message")).and_then(|message| message.get("content")) {
            Some(serde_json::Value::String(content)) => descriptions.push(format!("Image: {}\nDescription: {}", path.display(), content)),
            _ => return Err(format!("Unexpected API response format. Response: {}", response_json).into()),
        }
    }

    Ok(descriptions.join("\n\n"))
}

async fn generate_readme_with_model(
    project_info: &str,
    image_descriptions: &str,
    github_token: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    let client = Client::new();

    let system_message = "You are an expert README generator. Your task is to create a comprehensive, well-structured README file for a software project. Use the provided project structure, file contents, and image descriptions to infer the project's purpose, features, and requirements. Include sections for Project Name, Description, Installation, Usage, Features, Contributing, and License. Use proper Markdown formatting. When including images, use the correct relative paths provided and incorporate the image descriptions appropriately.";

    let prompt = format!(
        "Generate a README.md file for the following project. Use the project structure, file contents, and image descriptions to provide accurate and specific information about the project. Include relevant images where appropriate, using the exact paths provided and incorporating their descriptions.

Project Information:
{}

Image Descriptions:
{}

Please generate the README.md content:",
        project_info, image_descriptions
    );

    let response = client.post(format!("{}/v1/chat/completions", ENDPOINT))
        .header("Authorization", format!("Bearer {}", github_token))
        .json(&json!({
            "messages": [
                {"role": "system", "content": system_message},
                {"role": "user", "content": prompt}
            ],
            "model": MODEL_NAME,
            "temperature": 0.7,
            "max_tokens": 1000,
            "top_p": 0.95
        }))
        .send()
        .await?;

    if !response.status().is_success() {
        return Err(format!("API error: {} - {}", response.status(), response.text().await?).into());
    }

    let response_json: serde_json::Value = response.json().await?;

    if let Some(error) = response_json.get("error") {
        return Err(format!("API error: {}", error).into());
    }

    match response_json.get("choices").and_then(|choices| choices.get(0)).and_then(|choice| choice.get("message")).and_then(|message| message.get("content")) {
        Some(serde_json::Value::String(content)) => Ok(content.trim().to_string()),
        _ => Err(format!("Unexpected API response format. Response: {}", response_json).into()),
    }
}