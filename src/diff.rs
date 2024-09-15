use std::{
    fs::File,
    io::{self, BufRead, BufReader},
    path::Path,
};

use crate::helper::compute_file_hash;

// TODO: Implement full logic for diff command
pub fn diff() -> io::Result<()> {
    let index_file_path = Path::new(".rgit/index");
    let index_file = File::open(index_file_path)?;
    let index_file_rdr = BufReader::new(index_file);

    // Loop through the contents of index file
    for line in index_file_rdr.lines() {
        let line = line?;
        let mut parts = line.split_whitespace();
        // Get hash value and file path that store in the index file
        let (hash_value, file_path) = (parts.next().unwrap(), parts.next().unwrap());

        let current_file_path = Path::new(file_path);
        // Get current file hash value
        if let Ok(current_hash_value) = compute_file_hash(current_file_path) {
            // If current file hash value string is not equal with hash value that store inside index file, then file change detected
            if current_hash_value != hash_value {
                println!("File '{}' has changed", file_path);
            }
        }
    }

    Ok(())
}
