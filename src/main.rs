use clap::{Arg, Command};
use sha1::{Digest, Sha1};
use std::{
    fs::{self, File, OpenOptions},
    io::{self, BufRead, BufReader, Read, Write},
    path::Path,
    time::SystemTime,
};

// Git init command implementation
fn init() -> io::Result<()> {
    // Check if the .rgit directory already exists
    let git_file_dir = Path::new(".rgit");
    if git_file_dir.exists() {
        println!("Rgit repository already initialized.");
        return Ok(());
    }

    // Create necessary files and folders tracked by rgit
    fs::create_dir(".rgit")?;
    fs::create_dir(".rgit/objects")?;
    fs::create_dir_all(".rgit/refs/heads")?;

    File::create(".rgit/index")?;
    let mut head_file = File::create(".rgit/HEAD")?;
    head_file.write_all(b"ref: refs/heads/master")?;
    println!("Initialized empty rgit repository.");
    Ok(())
}

// Git add command implementation
fn add(file_path: &str) -> io::Result<()> {
    // Open the file to add (e.g. git add main.rs)
    let mut file = File::open(file_path)?;
    let mut contents: Vec<u8> = vec![];
    // Read the content of the file
    file.read_to_end(&mut contents)?;

    // Get the SHA1 hash of the content
    let mut hasher = Sha1::new();
    hasher.update(&contents);
    let hash = hasher.finalize();
    let hash_str = format!("{:x}", hash);

    // Create a blob folder under .rgit/objects
    let blob_dir_name = &hash_str[0..2];
    let blob_file_name = &hash_str[2..];
    fs::create_dir_all(format!(".rgit/objects/{}", blob_dir_name))?;
    let mut blob_file = File::create(format!(
        ".rgit/objects/{}/{}",
        blob_dir_name, blob_file_name
    ))?;
    // Write the content to the blob file
    blob_file.write_all(&contents)?;

    // Append the hash and file path to the index file
    let mut index_file = File::options().append(true).open(".rgit/index")?;
    writeln!(index_file, "{} {}", hash_str, file_path)?;

    println!("File {} added to the staging area.", file_path);
    Ok(())
}

fn commit(message: &str) -> io::Result<()> {
    // Read index file to get hash values and files names that reach to staging area
    let index_path = Path::new(".rgit/index");
    // Check index file exist
    if !index_path.exists() {
        println!("No file in staging area to commit");
        return Ok(());
    }
    let index_file = File::open(index_path)?;
    let index_file_rdr = BufReader::new(index_file); // Read buffer for index file
    let mut files: Vec<(String, String)> = vec![]; // Vec to store tuple of index file hash value and file path

    // Loop through index file content
    for line in index_file_rdr.lines() {
        if let Ok(line) = line {
            // Split in whitespace (e.g HASH_VALUE FILE, HASH_VALUE will be hash_value and FILE will be file_path)
            let mut parts = line.split_whitespace();
            if let (Some(hash_value), Some(file_path)) = (parts.next(), parts.next()) {
                // Push those value into tuple format
                files.push((hash_value.to_string(), file_path.to_string()));
            }
        }
    }

    // Create tree object
    let mut tree_content = String::new();

    // Loop through to add hash value and filepath to the tree content
    for (hash_value, file_path) in &files {
        tree_content.push_str(&format!("blob {} {}\n", hash_value, file_path));
    }

    // Hash tree object
    let mut tree_hasher = Sha1::new();
    tree_hasher.update(tree_content.as_bytes());
    let tree_hash_str = format!("{:x}", tree_hasher.finalize());

    // Write tree object to .rgit/objects/
    let tree_dir_name = &tree_hash_str[0..2];
    let tree_file_name = &tree_hash_str[2..];
    fs::create_dir_all(format!(".rgit/objects/{}", tree_dir_name))?;
    let mut tree_obj = File::create(format!(
        ".rgit/objects/{}/{}",
        tree_dir_name, tree_file_name
    ))?;
    tree_obj.write_all(tree_content.as_bytes())?;

    // Create commit object
    let author = "Kei-K23 keiksl2301@gmail.com";
    let timestamp = SystemTime::now();
    // Commit object in string format
    let commit_content = format!(
        "tree {}\n\
        author {} {:?}\n\
        committer {} {:?}\n\n\
        {}\n",
        tree_hash_str, author, timestamp, author, timestamp, message
    );

    let mut commit_hasher = Sha1::new();
    commit_hasher.update(commit_content.as_bytes());
    let commit_hash_str = format!("{:x}", commit_hasher.finalize());

    // Write commit hash tree to .rgit/objects
    let commit_dir_name = &commit_hash_str[0..2];
    let commit_file_name = &commit_hash_str[2..];
    fs::create_dir_all(format!(".rgit/objects/{}", commit_dir_name))?;
    let mut commit_obj = File::create(format!(
        ".rgit/objects/{}/{}",
        commit_dir_name, commit_file_name
    ))?;
    commit_obj.write_all(commit_content.as_bytes())?;

    // Update head file in refs heads file
    let mut head_file = File::create(".rgit/refs/heads/master")?;
    head_file.write_all(commit_hash_str.as_bytes())?;

    println!("Committed with message {}", message);
    Ok(())
}

// Configuration command
fn set_config(key: &str, value: &str) -> io::Result<()> {
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
        if line.trim() == "[user]" {
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

    println!("Configuration updated: {}={}", key, value);

    Ok(())
}

fn get_config(key: &str) -> io::Result<Option<String>> {
    let config_file_path = ".rgit/config";

    if let Ok(config_file) = File::open(config_file_path) {
        // Create buffer to read line by line to config file content
        let config_file_rdr = BufReader::new(config_file);
        let mut in_user_section = true; // Indicator for '[user]' section

        for line in config_file_rdr.lines() {
            let line = line?;
            if line.trim() == "[user]" {
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

    println!("No configuration found for '{}'", key);
    Ok(None)
}

fn main() {
    // CLI interface
    let matches = Command::new("rgit")
        .version("0.1.0")
        .about("Rgit is a Git implementation in Rust")
        .author("Kei-K23")
        .subcommand(
            Command::new("init")
                .about("Create an empty Git repository or reinitialize an existing one"),
        )
        .subcommand(
            Command::new("add")
                .about("Add file contents to the index")
                .arg(Arg::new("file").required(true).help("The file to add")),
        )
        .subcommand(
            Command::new("commit")
                .about("Record changes to the repository")
                .arg(Arg::new("message").required(true).help("Commit message")),
        )
        .get_matches();

    // Handle the init command
    if let Some(_) = matches.subcommand_matches("init") {
        if let Err(e) = init() {
            eprintln!("Error initializing repository: {}", e);
        }
    }

    // Handle the add command
    if let Some(add_matches) = matches.subcommand_matches("add") {
        if let Some(file_path) = add_matches.get_one::<String>("file") {
            if let Err(e) = add(file_path) {
                eprintln!("Error adding file to the staging area: {}", e);
            }
        }
    }

    // Handle the commit command
    if let Some(add_matches) = matches.subcommand_matches("commit") {
        if let Some(message) = add_matches.get_one::<String>("message") {
            if let Err(e) = commit(&message) {
                eprintln!("Error committing file to the repository: {}", e);
            }
        }
    }
}
