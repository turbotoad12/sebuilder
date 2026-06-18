use std::fs;
use std::io::Cursor;
use std::path::Path;
use zip::ZipArchive;

// Embed your template ZIP directly into the binary
static PROJECT_TEMPLATE_ZIP: &[u8] = include_bytes!("../templates/project.zip");

pub fn create_new_project(name: &str) -> Result<(), Box<dyn std::error::Error>> {
    let root = Path::new(name);

    // Create the project directory
    fs::create_dir_all(root)?;

    // Load ZIP from embedded bytes
    let reader = Cursor::new(PROJECT_TEMPLATE_ZIP);
    let mut archive = ZipArchive::new(reader)?;

    // Extract ZIP contents into the new project folder
    archive.extract(root)?;

    println!("Created new project: {}", name);
    Ok(())
}


