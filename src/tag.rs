use std::{
    fs::{self, File},
    io::{self, Write},
    path::Path,
};

pub fn tag(tag_name: &str) -> io::Result<()> {
    // TODO: Change heard code head_file value (currently master) to dynamic according to active branch
    let head_content = fs::read_to_string(".rgit/refs/heads/master")?;
    let tag_folder_path = Path::new(".rgit/refs/tags");

    // If tags folder not created yet, then create it
    if !tag_folder_path.exists() {
        fs::create_dir_all(tag_folder_path)?;
    }

    // Create new tag file inside tags dir
    let tag_file_path = tag_folder_path.join(tag_name);
    // If tag file name already exist them skip the creation
    if tag_file_path.exists() {
        println!("Tag '{tag_name}' already exist");
        return Ok(());
    }

    let mut tag_file = File::create(tag_file_path)?;

    // Store latest hashed commit blob to new tag file
    tag_file.write_all(head_content.as_bytes())?;

    println!("Tag '{}' created", tag_name);
    Ok(())
}

pub fn list_tags() -> io::Result<()> {
    let tag_dir_path = Path::new(".rgit/refs/tags");

    if tag_dir_path.exists() {
        for entry in fs::read_dir(tag_dir_path)? {
            let entry = entry?;
            let tag_name = entry.file_name().into_string().unwrap();

            println!("{}", tag_name);
        }
    } else {
        eprintln!("No tags found");
    }

    Ok(())
}

pub fn delete_tag(tag_name: &str) -> io::Result<()> {
    // Check heads dir exist (to store branches)
    let tag_folder_path = Path::new(".rgit/refs/tags");
    if !tag_folder_path.exists() {
        println!("No tag available to delete");
        return Ok(());
    }

    let delete_tag_file = tag_folder_path.join(tag_name);

    if !delete_tag_file.exists() {
        println!("Tag '{tag_name}' not found");
        return Ok(());
    } else {
        // Delete branch file
        fs::remove_file(delete_tag_file)?;
        println!("Tag '{tag_name}' successfully deleted");
    }

    Ok(())
}
