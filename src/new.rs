use std::fs;
use std::io::Cursor;
use std::path::Path;
use zip::ZipArchive;


pub fn create_new_project(name: &str) -> Result<(), Box<dyn std::error::Error>> {
    let root = Path::new(name);

    // Create the project directory
    fs::create_dir_all(root)?;

    let response = reqwest::blocking::get(
        "https://github.com/turbotoad12/sebuilder/raw/refs/heads/main/templates/project.zip"
    )?;

    if !response.status().is_success() {
        return Err(format!("Failed to download file: HTTP {}", response.status()).into());
    }


    let bytes = response.bytes()?;
    let reader = Cursor::new(bytes.as_ref());

    // Open the ZIP archive
    let mut archive = ZipArchive::new(reader)?;

    // Extract ZIP contents into the new project folder
    archive.extract(root)?;

    println!("Created new project: {}", name);
    Ok(())
}


