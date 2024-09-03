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
    let install_command = prompt("Enter the installation command(s). You can add multiple commands by separating with && (If left blank, I will generate one for you automatically):");

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
