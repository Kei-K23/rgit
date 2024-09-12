use std::{
    fs::{self},
    io,
    path::Path,
};

// Git init command implementation
fn git_init() -> io::Result<()> {
    // Check .git file already exist
    let git_file_dir = Path::new(".rgit");
    if git_file_dir.exists() {
        println!("Git repository already initialized.");
        Ok(())
    }

    // Creat
    fs::create_dir(".rgit")?;
    fs::create_dir(".rgit/objects")?;
    fs::create_dir_all(".rgit/refs/heads")?;
}

fn main() {
    println!("Hello, world!");
}
