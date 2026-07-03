use std::fs;
use std::io::Cursor;
use std::path::Path;
use ctr_rsf::{load_rsf_safe, sanitize_product_code, save_rsf, Rsf};
use ctr_rsf::rng::generate_unique_id;
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

    let rsf_path = format!("{}/assets/3ds/makerom.rsf.in", root.to_str().unwrap());
    // load rsf_path file
    let rsf_file = std::fs::read_to_string(&rsf_path)?;
    let mut rsf: Rsf = load_rsf_safe(&rsf_file).map_err(|e| {
        std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            format!("Failed to load RSF file: {e:?}"),
        )
    })?;

    rsf.basic_info.title = name.to_string();
    rsf.basic_info.set_product_code(&sanitize_product_code(name)).expect("Failed to set product code.");
    rsf.title_info.unique_id = generate_unique_id();

    save_rsf(rsf_path, &rsf).map_err(|e| {
        std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("Failed to save RSF file: {e:?}"),
        )
    })?;

    println!("Created new project: {}", name);


    Ok(())
}


