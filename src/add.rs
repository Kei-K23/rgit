use std::{
    fs::{self, File},
    io::{self, Read, Write},
};

use flate2::write::ZlibEncoder;
use flate2::Compression;
use sha1::{Digest, Sha1};

use crate::helper::get_latest_staged_hash;

// Git add command implementation
// TODO: Handle adding same staged file
pub fn add(file_path: &str) -> io::Result<()> {
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

    // Check the file is modified or make changes before adding to staging area
    let current_latest_hash = get_latest_staged_hash(file_path)?.unwrap();

    if current_latest_hash == hash_str {
        println!("No changes detected to add to staging area");
        return Ok(());
    }

    // Create a blob folder under .rgit/objects
    let blob_dir_name = &hash_str[0..2];
    let blob_file_name = &hash_str[2..];
    let blob_file_path = format!(".rgit/objects/{}/{}", blob_dir_name, blob_file_name);
    fs::create_dir_all(format!(".rgit/objects/{}", blob_dir_name))?;
    let mut blob_file = File::create(&blob_file_path)?;

    // Compress the blob using the DEFLATE algorithm
    let mut encoder = ZlibEncoder::new(Vec::new(), Compression::default());
    encoder.write_all(&contents)?;
    let compressed_blob = encoder.finish()?;

    // Write the content to the blob file
    blob_file.write_all(&compressed_blob)?;

    // Append the hash and file path to the index file
    let mut index_file = File::options().append(true).open(".rgit/index")?;
    writeln!(index_file, "{} {}", hash_str, file_path)?;

    println!("File added to the staging: {}", file_path);
    println!("Stored object as: {}", blob_file_path);
    Ok(())
}
