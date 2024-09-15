// All helper and reusable functions are stored

use std::{
    fs::{self, OpenOptions},
    io::{self, Write},
};

use flate2::write::ZlibEncoder;
use flate2::Compression;
use sha1::{Digest, Sha1};

// Compute hash, compress, store the object and return hash value
pub fn hash_and_store_obj(content_type: &str, content: &str) -> io::Result<String> {
    // Header information
    let header = format!("{} {}\0", content_type, content.len());

    let mut obj = vec![];
    obj.extend_from_slice(header.as_bytes());
    obj.extend_from_slice(content.as_bytes());

    // Compute SHA-1 hash of the content
    let mut hasher = Sha1::new();
    hasher.update(&obj);
    let hash_value_str = format!("{:x}", hasher.finalize());

    // Compress the object using the DEFLATE
    let mut encoder = ZlibEncoder::new(Vec::new(), Compression::default());
    encoder.write_all(&obj)?;
    let compressed_obj = encoder.finish()?;

    // Create object file and folder
    let object_dir = format!(".rgit/objects/{}", &hash_value_str[0..2]);
    let object_path = format!("{}/{}", object_dir, &hash_value_str[2..]);

    fs::create_dir_all(object_dir)?; // Make sure to create object dir

    // Create and write object file
    let mut object_file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open(&object_path)?;

    object_file.write_all(&compressed_obj)?;

    Ok(hash_value_str)
}
