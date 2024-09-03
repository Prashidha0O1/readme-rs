use colored::Colorize; 
use std::collections::HashMap; 
use std::fs; 
use std::io::{self, Write};
use std::process::{Command, Output}; 

/// Prompts the user for input with a custom message and returns the input as a `String`.

fn prompt(msg: &str) -> String {
    let mut intake = String::new(); // Create a mutable string to capture input
    println!("{}", msg.red()); // Print the message in red color for emphasis
    io::stdin().read_line(&mut intake).expect("Failed to read line."); // Read user input from stdin

    // TODO: Add input validation here to ensure the input is not just whitespace.
    intake.trim().to_string() // Trim the input to remove leading/trailing whitespace and return it
}

/// Main function that initializes the program and handles user input for generating a README file.
fn main() {
    // TODO: Move the ASCII art to a separate function to clean up `main`.
    let ascii_art = r#"
__________                   .___                 ________                                   __                
\______   \ ____ _____     __| _/_____   ____    /  _____/  ____   ____   ________________ _/  |_  ___________ 
 |       _// __ \\__  \   / __ |/     \_/ __ \  /   \  ____/ __ \ /    \_/ __ \_  __ \__  \\   __\/  _ \_  __ \
 |    |   \  ___/ / __ \_/ /_/ |  Y Y  \  ___/  \    \_\  \  ___/|   |  \  ___/|  | \// __ \|  | (  <_> )  | \/
 |____|_  /\___  >____  /\____ |__|_|  /\___  >  \______  /\___  >___|  /\___  >__|  (____  /__|  \____/|__|   
        \/     \/     \/      \/     \/     \/          \/     \/     \/     \/           \/                   
                                                                  - By Prashidha0O1 (rawalprashidha@gmail.com)
"#;

    println!("{}", ascii_art.green().bold()); // Print ASCII art in green bold text for better visibility

    // Prompt user for various inputs
    let project_name = prompt("Enter the project name:");
    let project_logo = prompt("Enter the image url of the project logo (Leave blank if none):");
    let project_desc = prompt("Enter the short description (It should be short and concise.):");
    let project_image_url = prompt("Enter the image url (Leave blank if none):");
    let license = prompt("Enter the license - (MIT/Apache/GPL / Enter any custom license) (Leave blank if none):");
    let project_demo = prompt("Link for image/gif demonstrating your project (Leave blank if none):");
    let install_command = prompt("Enter the install command. You can add multiple commands by separating with && (If left blank, I will generate one for you automatically):");

    // Ensure mandatory fields are not empty
    assert!(!project_name.trim().is_empty(), "Project name cannot be empty");
    assert!(!project_desc.trim().is_empty(), "Short description cannot be empty");

    // Call the function to generate the README file based on user input
    file_factory(
        project_name.trim(),
        project_desc.trim(),
        project_image_url.trim(),
        license.trim(),
        project_demo.trim(),
        install_command.trim(),
    );
}

fn file_factory(
    project_name: &str,
    project_desc: &str,
    project_image_url: &str,
    license: &str,
    project_demo: &str,
    install_command: &str,
) {
    let mut file = fs::File::create("README.md").expect("Failed to create file"); // Create a new README.md file
    let mut content = String::with_capacity(1024); // Initialize a String with a capacity to avoid reallocations

    // Add project name as a centered header
    content.push_str("<div align=\"center\">\n");
    content.push_str(&format!("<h1 align=\"center\">{}</h1>\n", project_name));

    // Add project logo if provided
    if !project_image_url.is_empty() {
        content.push_str(&format!(
            "<img src=\"{}\" alt=\"{}\" align=\"center\" width=\"80\" height=\"80\">\n",
            project_image_url, project_name
        ));
    }
    content.push_str("<br />\n");

    // Handle license badge
    let licenses = create_license_badges(); // Get the predefined license badges
    let binding = format!(
            "<img alt=\"License: {}\" src=\"https://img.shields.io/badge/License-{}-blue\" />",
            license, license
        ).as_str();
    let license_badge = licenses
        .get(&license.to_lowercase()[..]) // Check if provided license exists in the map
        .unwrap_or(&binding); // If not, create a custom badge

    content.push_str(license_badge);
    content.push_str("<br>\n<br>\n");

    // Add project description
    content.push_str(&format!("{}\n", project_desc));

    // Add project image if provided
    if project_image_url.len() > 5 {
        content.push_str(&format!(
            "<img src=\"{}\" alt=\"{}\" width=\"500\" height=\"500\">\n",
            project_image_url, project_name
        ));
    }

    content.push_str("</div>\n\n***\n");

    // Add demo image or GIF if provided
    if project_demo.len() > 5 {
        content.push_str(&format!("![{}]({})\n", project_name, project_demo));
    }

    // Add installation section
    content.push_str("\n### Installation\n");
    if install_command.len() > 5 {
        content.push_str(&format!("```\n{}\n```\n", install_command.replace("&& ", "\n"))); // Custom commands
    } else {
        content.push_str(&generate_installation_instructions(project_name)); // Auto-generated instructions
    }

    // Add usage section
    content.push_str(&format!("\n### Usage\n```\n{}\n```\n", generate_usage_command()));

    // Add contributing and license sections
    content.push_str("\n### Contributing\n\n");
    content.push_str("\n### License\n");
    content.push_str(&format!(
        "This project is licensed under the {} license\n",
        license
    ));
    content.push_str("\n### Show your support\nLeave a ⭐ if you like this project\n");
    content.push_str("\n***\nReadme made with 😭😭 using [README Generator](https://github.com/Prashidha0O1/readme-rs.git)");

    // Write all the content to the README.md file
    file.write_all(content.as_bytes()).expect("Failed to write to file");
}


/// * A `String` containing installation instructions.
fn generate_installation_instructions(project_name: &str) -> String {
    let mut instructions = String::new();

    // Check if Git is available and get the repository URL
    if let Ok(repo_url) = Command::new("git").arg("remote").arg("get-url").arg("origin").output() {
        instructions.push_str("```\n");
        for line in String::from_utf8_lossy(&repo_url.stdout).lines() {
            instructions.push_str(&format!("git clone {}\n", line)); // Add git clone command
        }

        instructions.push_str(&format!("cd {}", project_name)); // Add command to change directory
        if fs::metadata("Cargo.toml").is_ok() {
            instructions.push_str("cargo install \n"); // Rust projects
        } else if fs::metadata("package.json").is_ok() {
            instructions.push_str("npm install"); // Node.js projects
        } else if fs::metadata("requirements.txt").is_ok() {
            instructions.push_str("pip install -r requirements.txt"); // Python projects
        }
        instructions.push_str("```\n");
    }

    // TODO: Add more checks for other popular project types (e.g., Java projects with `pom.xml` or Gradle).
    instructions
}

/// Generates a sample usage command for demonstration purposes.
fn generate_usage_command() -> String {
    // TODO: Customize the usage example based on the project type.
    "your_executable_name --help".to_string()
}
fn create_license_badges() -> HashMap<&'static str, &'static str> {
    // TODO: Extend this map with more licenses as needed.
    let mut licenses = HashMap::new();
    licenses.insert(
        "mit",
        "<img alt=\"License: MIT\" src=\"https://img.shields.io/badge/License-MIT-yellow.svg\" />",
    );
    licenses.insert(
        "apache",
        "<img alt=\"License: Apache\" src=\"https://img.shields.io/badge/License-Apache%202.0-blue.svg\" />",
    );
    licenses.insert(
        "gpl",
        "<img alt=\"License: GPL\" src=\"https://img.shields.io/badge/License-GPLv3-blue.svg\" />",
    );

    licenses
}
