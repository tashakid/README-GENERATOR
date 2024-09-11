# README Generator

## Project Description
This project is a README generator that automatically creates comprehensive README.md files for software projects. It uses the Hugging Face API to generate content based on the project's structure and configuration files.

## Features
- Gathers project information automatically
- Generates README content using AI (Hugging Face models)
- Supports multiple project types (Rust, Node.js, Python)
- Retries on API failures

## Installation
To install this project, follow these steps:

1. Clone the repository:
   ```
   git clone [your-repo-url]
   cd [your-project-name]
   ```

2. Ensure you have Rust installed on your system. 
   If not, install it from [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install)

3. Build the project:
   ```
   cargo build --release
   ```
4. Run the program:
    ```
    cargo run
    ``` 
## Usage
The readme generator will prompt you to:
    1. Enter the path to your project directory. 
    2. Enter your Hugging Face API key.
    3. Enter the Hugging Face model to use.

## Configuration
The generator looks for the following configuration files in your project:
- `Cargo.toml` (for Rust projects)
- `package.json` (for Node.js projects)
- `requirements.txt` or `setup.py` (for Python projects)

Ensure these files are present and up-to-date in your project for the best results.

## Contributing
Contributions to this project are welcome. Please follow these steps:

1. Fork the repository
2. Create a new branch (`git checkout -b feature/your-feature`).
3. Make your changes.
4. Commit your changes (`git commit -am 'Add some feature'`).
5. Push to the branch (`git push origin feature/your-feature`).
6. Create a new Pull Request.

## Contact
[email: kinyuanatasha657@gmail.com]