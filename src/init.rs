// Implementation for git init command

use std::{
    fs::{self, File},
    io::{self, Write},
    path::Path,
};

/// Create an empty .rgit repository or reinitialize an existing one
pub fn init() -> io::Result<()> {
    // Check if the .rgit directory already exists
    let git_file_dir = Path::new(".rgit");
    if git_file_dir.exists() {
        println!(".rgit repository already initialized.");
        return Ok(());
    }

    // Create necessary files and folders tracked by rgit
    fs::create_dir(".rgit")?;
    fs::create_dir(".rgit/objects")?;
    fs::create_dir_all(".rgit/refs/heads")?;
    fs::create_dir_all(".rgit/refs/tags")?;

    File::create(".rgit/config")?;
    File::create(".rgit/index")?;
    let mut head_file = File::create(".rgit/HEAD")?;
    head_file.write_all(b"ref: refs/heads/master")?;
    println!("Initialized empty rgit repository.");
    Ok(())
}
