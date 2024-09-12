use clap::{Arg, Command};
use sha1::{Digest, Sha1};
use std::{
    fs::{self, File},
    io::{self, Read, Write},
    path::Path,
};

// Git init command implementation
fn git_init() -> io::Result<()> {
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
fn git_add(file_path: &str) -> io::Result<()> {
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
        if let Err(e) = git_init() {
            eprintln!("Error initializing repository: {}", e);
        }
    }

    // Handle the add command
    if let Some(add_matches) = matches.subcommand_matches("add") {
        if let Some(file_path) = add_matches.get_one::<String>("file") {
            if let Err(e) = git_add(file_path) {
                eprintln!("Error adding file to the staging area: {}", e);
            }
        }
    }
}
