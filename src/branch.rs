use std::{
    fs::{self},
    io::{self},
    path::Path,
};

// Get list of branch or create new branch
pub fn branch(new_branch: Option<&String>) -> io::Result<()> {
    // Check heads dir exist (to store branches)
    let heads_dir = Path::new(".rgit/refs/heads");
    if !heads_dir.exists() {
        println!("No branches available");
        return Ok(());
    }

    if let Some(new_branch) = new_branch {
        // Create new branch from current HEAD branch if new_branch have value
        let master_file = heads_dir.join("master"); // TODO :: Fix heard coded branch name to dynamic
        let new_branch_file = heads_dir.join(new_branch);

        if new_branch_file.exists() {
            println!("Branch {new_branch} already exist");
            return Ok(());
        } else {
            fs::copy(master_file, new_branch_file)?;
            println!("Branch {} created.", new_branch);
        }
    } else {
        // Show branches that have in heads dir
        println!("Branches:");
        for entry in fs::read_dir(heads_dir)? {
            let entry = entry?;
            let branch_name = entry.file_name().into_string().unwrap();
            println!("  {branch_name}");
        }
    }

    Ok(())
}

pub fn delete_branch(branch_name: &str) -> io::Result<()> {
    // Check heads dir exist (to store branches)
    let heads_dir = Path::new(".rgit/refs/heads");
    if !heads_dir.exists() {
        println!("No branches available to delete");
        return Ok(());
    }

    let delete_branch_file = heads_dir.join(branch_name);

    // TODO :: Do I need check the current active branch is our want to delete branch
    if !delete_branch_file.exists() {
        println!("Branch '{branch_name}' not found");
        return Ok(());
    } else {
        // Delete branch file
        fs::remove_file(delete_branch_file)?;
        println!("Branch '{branch_name}' successfully deleted");
    }

    Ok(())
}
