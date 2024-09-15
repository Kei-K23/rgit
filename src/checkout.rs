use std::{
    fs::File,
    io::{self, Write},
    path::Path,
};

pub fn checkout(branch_or_commit: &str) -> io::Result<()> {
    let head_path = ".rgit/HEAD";

    let branch_path = format!(".rgit/refs/heads/{}", branch_or_commit);

    // Branch exist, then perform branch switching
    if Path::new(&branch_path).exists() {
        let mut head_file = File::create(head_path)?;
        head_file.write_all(format!("ref: refs/heads/{}", branch_or_commit).as_bytes())?;

        println!("Switched to branch {}", branch_or_commit);
    } else {
        // Commit check out here
        let commit_obj_path = format!(
            ".rgit/objects/{}/{}",
            &branch_or_commit[0..2],
            &branch_or_commit[2..]
        );

        if Path::new(&commit_obj_path).exists() {
            // Commit obj exist
            let mut head_file = File::create(head_path)?;
            // Update HEAD file content to user passed commit
            head_file.write_all(branch_or_commit.as_bytes())?;
            println!("Checked out commit {}", branch_or_commit);
        } else {
            return Err(io::Error::new(
                io::ErrorKind::NotFound,
                "Error: branch or commit not found",
            ));
        }
    }
    Ok(())
}
