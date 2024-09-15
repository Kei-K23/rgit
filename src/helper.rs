// All helper and reusable functions are stored

use std::{
    fs::{self, File, OpenOptions},
    io::{self, BufRead, BufReader, Read, Write},
    path::Path,
};

use flate2::write::ZlibEncoder;
use flate2::Compression;
use sha1::{Digest, Sha1};

pub fn hash_and_store_obj(content_type: &str, content: &str) -> io::Result<String> {
    // Header information
    let header = format!("{} {}\0", content_type, content.len());

    let mut obj: Vec<u8> = vec![];
    obj.extend_from_slice(header.as_bytes());
    obj.extend_from_slice(content.as_bytes());

    // Compute SHA-1 hash of the content
    let mut hasher = Sha1::new();
    hasher.update(&obj);
    let hash_value_str = format!("{:x}", hasher.finalize());

    // Compress the object using the DEFLATE (Git uses zlib compression)
    let mut encoder = ZlibEncoder::new(Vec::new(), Compression::default());
    encoder.write_all(&obj)?; // Compress the header + content
    let compressed_obj = encoder.finish()?; // Get the compressed bytes

    // Create object file and folder
    let object_dir = format!(".rgit/objects/{}", &hash_value_str[0..2]);
    let object_path = format!("{}/{}", object_dir, &hash_value_str[2..]);

    fs::create_dir_all(&object_dir)?; // Make sure to create object dir

    // Create and write object file
    let mut object_file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open(&object_path)?;

    object_file.write_all(&compressed_obj)?;

    Ok(hash_value_str)
}

pub fn create_tree() -> io::Result<String> {
    let index_path = ".rgit/index";
    let mut tree_contents = vec![];

    let index_file = File::open(index_path)?;
    let index_rdr = BufReader::new(index_file);

    for line in index_rdr.lines() {
        let line = line?;
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() != 2 {
            continue;
        }
        let (hash, file_path) = (parts[0], parts[1]);

        // Example tree line: "100644 blob <hash>\t<file_name>"
        tree_contents.push(format!("100644 blob {}\t{}", hash, file_path));
    }

    let tree_contents_str = tree_contents.join("\n");

    let tree_hash = hash_and_store_obj("tree", &tree_contents_str)?;

    Ok(tree_hash)
}

pub fn get_current_ref_branch() -> io::Result<Option<String>> {
    let head_path = ".rgit/HEAD";
    // Check if HEAD file exists
    if Path::new(head_path).exists() {
        // Read the current branch from HEAD file
        let mut head_file = File::open(head_path)?;
        let mut head_file_content = String::new();
        head_file.read_to_string(&mut head_file_content)?;
        // Extract the branch reference (assuming it's the second part)
        let parts: Vec<&str> = head_file_content.split_whitespace().collect();

        if parts.len() < 2 {
            return Ok(None); // Invalid format in HEAD file
        }

        let branch_ref_file = parts[1].to_string();
        Ok(Some(branch_ref_file))
    } else {
        Ok(None)
    }
}

pub fn get_parent_commit() -> io::Result<Option<String>> {
    if let Some(branch_ref_file) = get_current_ref_branch()? {
        let branch_ref_path = format!(".rgit/{}", branch_ref_file);

        // If there is not commit yet! return with None
        if !Path::new(&branch_ref_path).exists() {
            return Ok(None);
        }

        // Read the commit hash from the branch reference file
        let mut ref_file = File::open(branch_ref_path)?;
        let mut previous_commit = String::new();
        ref_file.read_to_string(&mut previous_commit)?;

        Ok(Some(previous_commit.trim().to_string()))
    } else {
        Ok(None)
    }
}

// Helper function to compute file hashing (SHA-1)
pub fn compute_file_hash(file_path: &Path) -> io::Result<String> {
    let mut file = File::open(file_path)?;
    let mut hasher = Sha1::new();
    let mut buffer = vec![0; 1024];

    loop {
        let bytes_read = file.read(&mut buffer)?;
        if bytes_read == 0 {
            break;
        }
        hasher.update(&buffer[..bytes_read]);
    }

    Ok(format!("{:x}", hasher.finalize()))
}

// Function to get latest staged file hash value
pub fn get_latest_staged_hash(file_name: &str) -> io::Result<Option<String>> {
    let index_path = ".rgit/index";

    let index_file = File::open(index_path)?;
    let index_rdr = BufReader::new(index_file);

    let mut latest_hash_value: Option<String> = None;

    for line in index_rdr.lines() {
        let line = line?;
        let parts: Vec<&str> = line.split_whitespace().collect();

        if parts.len() == 2 && parts[1] == file_name {
            latest_hash_value = Some(parts[0].to_string());
        }
    }

    Ok(latest_hash_value)
}
