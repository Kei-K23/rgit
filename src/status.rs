use std::{
    fs::{self, File},
    io::{self, BufRead, BufReader},
};

use crate::helper::{compute_file_hash, get_latest_staged_hash};

pub fn status() -> io::Result<()> {
    let index_file_path = ".rgit/index";
    let mut staged_files: Vec<(String, String)> = vec![]; // (file_path, file_hash)

    let index_file = File::open(index_file_path)?;
    let index_file_rdr = BufReader::new(index_file);

    // Collect all staged files and their hashes from the index file
    for line in index_file_rdr.lines() {
        let line = line?;
        let mut parts = line.split_whitespace();
        let hash = parts.next().unwrap_or("");
        if let Some(file_path) = parts.next() {
            staged_files.push((file_path.to_string(), hash.to_string()));
        }
    }

    let mut untracked_files = vec![];
    let mut modified_files = vec![];

    // Check the current working directory for status
    for entry in fs::read_dir(".")? {
        let entry = entry?;
        let path = entry.path();

        if path.is_file() {
            let original_path = path.to_string_lossy().to_string();
            // Remove "./" when comparing file path
            let path_str = original_path.replace("./", "");
            // Check if the file is untracked
            if !staged_files
                .iter()
                .any(|(staged_file, _)| staged_file == &path_str)
            {
                untracked_files.push(original_path);
            } else {
                // Check if the file has been modified (compare current hash with staged hash)
                if let Ok(current_hash) = compute_file_hash(&path) {
                    let staged_hash_opt = get_latest_staged_hash(path_str.as_str())?;

                    if let Some(staged_hash) = staged_hash_opt {
                        if current_hash != staged_hash {
                            modified_files.push(path_str.clone());
                        }
                    }
                }
            }
        }
    }

    // Display untracked files
    if !untracked_files.is_empty() {
        println!("Untracked files:");
        for file in &untracked_files {
            println!("  {}", file);
        }
    }

    // Display modified files
    if !modified_files.is_empty() {
        println!("\nModified files:");
        for file in &modified_files {
            println!("  {}", file);
        }
    }

    Ok(())
}
