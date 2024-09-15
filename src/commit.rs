use std::{
    fs::{self, File},
    io::{self, BufRead, BufReader, Write},
    path::Path,
    time::SystemTime,
};

use sha1::{Digest, Sha1};

// TODO:: Add parent commit hash value to track parent commit
pub fn commit(message: &str) -> io::Result<()> {
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
