use std::{
    fs::{self, File},
    io::{self, Write},
    path::Path,
};

// Git init command implementation
fn git_init() -> io::Result<()> {
    // Check .git file already exist
    let git_file_dir = Path::new(".rgit");
    if git_file_dir.exists() {
        println!("Rgit repository already initialized.");
        return Ok(());
    }

    // Create necessary file and folder the track by rgit
    fs::create_dir(".rgit")?;
    fs::create_dir(".rgit/objects")?;
    fs::create_dir_all(".rgit/refs/heads")?;

    let mut head_file = File::create(".rgit/HEAD")?;
    head_file.write_all(b"ref: refs/heads/master")?;
    println!("Initialized empty rgit repository.");
    Ok(())
}

fn main() {
    println!("Hello, world!");
}
