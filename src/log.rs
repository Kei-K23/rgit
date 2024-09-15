use std::{
    fs::{self, File},
    io::{self, Read},
    path::Path,
};

use flate2::bufread::ZlibDecoder;

pub fn log() -> io::Result<()> {
    // TODO :: Need to check for current branch like (main or dev) or show all commit combine with all branches
    // Default to master branch
    let head_path = Path::new(".rgit/refs/heads/master");

    if !head_path.exists() {
        println!("No commits found");
        return Ok(());
    }
    // Get current commit hash value
    let mut current_commit = fs::read_to_string(head_path)?.trim().to_string();

    while !current_commit.is_empty() {
        let mut parent_commit: Option<String> = None;
        // Retrieve the commit object path
        let commit_path = format!(
            ".rgit/objects/{}/{}",
            &current_commit[0..2],
            &current_commit[2..]
        );

        // Read the commit file as raw bytes
        let mut commit_object_file = File::open(&commit_path)?;
        let mut encoded_commit_object = Vec::new(); // Use a byte vector
        commit_object_file.read_to_end(&mut encoded_commit_object)?;

        // Decompress the commit object using ZlibDecoder
        let mut d = ZlibDecoder::new(&encoded_commit_object[..]);
        let mut commit_contents = Vec::new(); // Use a byte vector for decompressed data
        d.read_to_end(&mut commit_contents)?;

        // Convert the decompressed contents to a string (if valid UTF-8)
        let commit_contents_str = String::from_utf8(commit_contents).map_err(|_| {
            io::Error::new(
                io::ErrorKind::InvalidData,
                "Commit content is not valid UTF-8",
            )
        })?;

        println!("commit {}", current_commit);
        for line in commit_contents_str.lines() {
            if line.starts_with("Author:") || line.starts_with("Message:") || line.is_empty() {
                println!("{}", line);
            } else if line.contains("Parent:") {
                // TODO :: Check and fix incorrect value for parent
                parent_commit = line.split_whitespace().nth(2).map(String::from);
            } else if line.starts_with("Tree:") {
                // Skip the commit tree (commit tree indicate the files that contain blobs)
            }
        }
        println!();

        // If no parent commit exists, end the traversal (first commit case)
        if parent_commit.is_none() {
            break;
        }

        // Move to the parent commit for the next iteration
        current_commit = parent_commit.unwrap();
    }

    Ok(())
}
