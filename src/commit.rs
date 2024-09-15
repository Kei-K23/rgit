use std::{
    fs::File,
    io::{self, Write},
    path::Path,
    time::{SystemTime, UNIX_EPOCH},
};

use crate::{
    config::get_config,
    helper::{create_tree, get_current_ref_branch, get_parent_commit, hash_and_store_obj},
};

pub fn commit(message: &str) -> io::Result<()> {
    let index_path = ".rgit/index";
    // Check changes file to commit
    if !Path::new(index_path).exists() {
        println!("No changes to commit (no staged files)");
        return Ok(());
    }

    // Create the tree object and get back the tree hash value
    let tree_hash = create_tree()?;

    // Get the current time
    // TODO :: Change to human readable date
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?
        .as_secs();

    // Get the parent commit (if any)
    let parent_commit = get_parent_commit()?;

    // Get author name and email form configuration
    let author_name = get_config("[user]", "name")?.unwrap_or("default".to_string());
    let author_email = get_config("[user]", "email")?.unwrap_or("default@email.com".to_string());

    // Create the commit object contents
    let mut commit_contents = format!(
        "Tree: {}\nAuthor: {} <{}> {} +0000\nMessage: {}",
        tree_hash, author_name, author_email, now, message
    );

    // If parent commit exist, then add to commit content
    if let Some(parent) = parent_commit {
        commit_contents = format!("Parent: {}\n{}", parent, commit_contents);
    }

    let commit_hash = hash_and_store_obj("commit", &commit_contents)?;

    // Create the reference file or update to link with current commit
    let branch_ref = get_current_ref_branch()?.unwrap();
    let branch_ref_path = format!(".rgit/{}", branch_ref);

    let mut branch_ref_file = File::create(&branch_ref_path)?;

    branch_ref_file.write_all(&commit_hash.as_bytes())?;

    println!("Committed with: {}", commit_hash);
    Ok(())
}
