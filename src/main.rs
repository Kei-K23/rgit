use clap::{Arg, Command};
use sha1::{Digest, Sha1};
use std::{
    fs::{self, File},
    io::{self, BufRead, BufReader, Read, Write},
    path::Path,
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

    Ok(())
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
}
