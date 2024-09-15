use std::{
    fs::{File, OpenOptions},
    io::{self, BufRead, BufReader, Write},
};

// Configuration command
fn set_config(section_name: &str, key: &str, value: &str) -> io::Result<()> {
    let config_file_path = ".rgit/config"; // config file path

    // Read config file if it exist and get value and store them in lines String Vector
    let mut lines: Vec<String> = vec![];
    if let Ok(config_file) = File::open(config_file_path) {
        let config_file_rdr = BufReader::new(config_file);
        for line in config_file_rdr.lines() {
            lines.push(line?);
        }
    }

    // Find or add '[user]' section and if already have user section then update the value according to key
    let mut is_user_section = false;
    let mut updated = false;

    // Loop through lines of config file
    for line in lines.iter_mut() {
        if line.trim() == section_name {
            is_user_section = true;
        } else if is_user_section && line.trim().starts_with(key) {
            *line = format!("    {} = {}", key, value); // Update the value when match with key
            updated = true;
            break;
        } else if line.trim().is_empty() {
            is_user_section = false;
        }
    }

    // Handle case for new config file or config that does not have '[user]' section
    if !updated {
        if !is_user_section {
            lines.push("[user]".to_string());
        }
        // Add new config for user
        lines.push(format!("    {} = {}", key, value));
    }

    // Write new update config file back to .rgit/config file
    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open(config_file_path)?;

    for line in lines {
        writeln!(file, "{}", line)?;
    }

    Ok(())
}

pub fn get_config(section_name: &str, key: &str) -> io::Result<Option<String>> {
    let config_file_path = ".rgit/config";

    if let Ok(config_file) = File::open(config_file_path) {
        // Create buffer to read line by line to config file content
        let config_file_rdr = BufReader::new(config_file);
        let mut in_user_section = true; // Indicator for '[user]' section

        for line in config_file_rdr.lines() {
            let line = line?;
            if line.trim() == section_name {
                in_user_section = true // By defining in_user_section indicator to true make sure below content will be associated with '[user]' section
            } else if in_user_section {
                // Split string by " = "
                if let Some((config_key, config_value)) = line.trim().split_once(" = ") {
                    // Match with user provided key
                    if config_key == key {
                        return Ok(Some(config_value.to_string()));
                    }
                }
            } else if line.trim().is_empty() {
                in_user_section = false;
            }
        }
    }

    Ok(None)
}

// Handle config commands (set or get)
pub fn handle_config_command(
    action: &str,
    key: &str,
    section_name: &str,
    value: Option<&str>,
) -> io::Result<()> {
    match action {
        "set" => {
            if let Some(val) = value {
                set_config(section_name, key, val)?;
                println!("Configuration set: {} = {}", key, val);
            } else {
                println!("Value required for 'set' command.");
            }
        }
        "get" => {
            if let Some(val) = get_config(section_name, key)? {
                println!("{} = {}", key, val);
            } else {
                println!("No configuration found for '{}'", key);
            }
        }
        _ => {
            println!("Invalid action. Use 'set' or 'get'.");
        }
    }

    Ok(())
}
