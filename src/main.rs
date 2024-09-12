use core::str;
use std::{
    fs::{self, File},
    io::{self, Read, Write},
    path::Path,
};

use sha1::{Digest, Sha1};

// Git init command implementation
fn git_init() -> io::Result<()> {
    // Check .git file already exist
    let git_file_dir = Path::new(".rgit");
    if git_file_dir.exists() {
        println!("Rgit repository already initialized.");
        return Ok(());
    }

    // Create necessary file and folder the track by rgit
    fs::create_dir(".rgit")?;
    fs::create_dir(".rgit/objects")?;
    fs::create_dir_all(".rgit/refs/heads")?;

    let mut head_file = File::create(".rgit/HEAD")?;
    head_file.write_all(b"ref: refs/heads/master")?;
    println!("Initialized empty rgit repository.");
    Ok(())
}

fn git_add(file_path: &str) -> io::Result<()> {
    // Open the git add file (e.g git add main.rs)
    let mut file = File::open(file_path)?;
    let mut contents: Vec<u8> = vec![];
    // Read content inside file and add them to contents Vec
    file.read_to_end(&mut contents)?;

    // Get hash key from content
    let mut hasher = Sha1::new();
    hasher.update(&contents);
    // Get hash value
    let hash = hasher.finalize();
    // Format hash value to hash string
    let hash_str = format!("{:x}", hash);

    // Create blob folder under .rgit/objects
    // Create new folder with the name of first two character of hash string value and create file under that folder with the rest of the name of hash value
    let blob_dir_name = &hash_str[0..2];
    let blob_file_name = &hash_str[2..];
    // Create folder
    fs::create_dir_all(format!(".rgit/objects/{}", blob_dir_name))?;
    let mut blob_file = File::create(format!(
        ".rgit/objects/{}/{}",
        blob_dir_name, blob_file_name
    ))?;
    // Write contents of bytes to blob file
    blob_file.write_all(&contents)?;

    // Instead of completely writing content, this will only append content to index file
    let mut index_file = File::options().append(true).open(".rgit/index")?;
    writeln!(index_file, "{} {}", hash_str, file_path)?;

    println!("File {} added to staging area.", file_path);
    Ok(())
}

fn main() {
    println!("Hello, world!");
}
