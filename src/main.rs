use colored::Colorize;
use std::collections::HashMap;
use std::fs;
use std::io::{stdin, Write};
use std::process::{Command, Output};


fn prompt(msg: &str){
    let intake = String::new();
    println!("{}", msg.red());
    stdin::read_line(&mut intake).expect("Failed to read line.")
    intake
}
fn main() {

    let ASCII_ART = r#"
        

__________                   .___                 ________                                   __                
\______   \ ____ _____     __| _/_____   ____    /  _____/  ____   ____   ________________ _/  |_  ___________ 
 |       _// __ \\__  \   / __ |/     \_/ __ \  /   \  ____/ __ \ /    \_/ __ \_  __ \__  \\   __\/  _ \_  __ \
 |    |   \  ___/ / __ \_/ /_/ |  Y Y  \  ___/  \    \_\  \  ___/|   |  \  ___/|  | \// __ \|  | (  <_> )  | \/
 |____|_  /\___  >____  /\____ |__|_|  /\___  >  \______  /\___  >___|  /\___  >__|  (____  /__|  \____/|__|   
        \/     \/     \/      \/     \/     \/          \/     \/     \/     \/           \/                   
                                                                  - By Prashidha0O1 (rawalprashidha@gmail.com)
        "#;
    println!("{}", ASCII_ART.green().bold(),);

    let project_name = prompt("Enter the project name:");
    let project_logo = prompt("Enter the image url of the project logo (Leave blank if none):");
    let project_desc = prompt("Enter the short description (It should be short and concise.):");
    let project_image_url = prompt("Enter the image url (Leave blank if none):");
    let license = prompt("Enter the license - (MIT/Apache/GPL / Enter any custom license) (Leave blank if none):");
    let project_demo = prompt("Link for image/gif demonstrating your project (Leave blank if none):");
    let install_command = prompt("Enter the install command. You can add multiple commands by separating with && (If left blank, I will generate one for you automatically):");

    assert!(!project_name.trim().is_empty(), "Project name cannot be empty");
    assert!(!project_desc.trim().is_empty(), "Short description cannot be empty");

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

    project_name: &str
    project_desc: &str
    project_image_url: &str
    license: &str
    project_demo: &str
    install_command: &str
){
 let mut file = fs::File::create("README.md").expect("Failed to create file");
    let mut content = String::with_capacity(1024);

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

    // License badge handling
    let licenses = create_license_badges();
    let license_badge = licenses
        .get(&license.to_lowercase()[..])
        .unwrap_or(&format!(
            "<img alt=\"License: {}\" src=\"https://img.shields.io/badge/License-{}-blue\" />",
            license, license
        ));

    content.push_str(license_badge);
    content.push_str("<br>\n<br>\n");

    content.push_str(&format!("{}\n",project_desc));

// Add project image if provided
    if project_image_url.len() > 5 {
        content.push_str(&format!(
            "<img src=\"{}\" alt=\"{}\" width=\"500\" height=\"500\">\n",
            project_image_url, project_name
        ));
    }

    content.push_str("</div>\n\n***\n");

    // Add demo if provided
    if demo.len() > 5 {
        content.push_str(&format!("![{}]({})\n", project_name, project_demo));
    }

    content.push_str("\n### Installation\n");
    if install_command.len() > 5 {
        content.push_str(&format!("```\n{}\n```\n", install_command.replace("&& ", "\n")));
    } else {
        content.push_str(&generate_installation_instructions(project_name));
    }

    content.push_str(&format!("\n### Usage\n```\n{}\n```\n", generate_usage_command()));

    content.push_str("\n### Contributing\n\n");
    content.push_str("\n### License\n");
    content.push_str(&format!(
        "This project is licensed under the {} license\n",
        license
    ));

    content.push_str("\n### Show your support\nLeave a ⭐ if you like this project\n");
    content.push_str("\n***\nReadme made with 💖 using [README Generator](https://github.com/Prashidha0O1/readme-rs.git)");

    // Save the file
    file.write_all(content.as_bytes()).expect("Failed to write to file");

}
