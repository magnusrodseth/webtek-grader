use std::fs::{self, File};
use std::io::{self, BufReader};
use std::path::Path;
use tar::Archive as TarArchive;
use unrar::Archive as RarArchive;
use zip::read::ZipArchive;

/// Function to extract the username from the assignment filename.
fn get_username(assignment: &str) -> String {
    // Attempt to extract the username by splitting on "_"
    assignment.split('_').nth(1).unwrap_or("").to_string()
}

/// Function to sanitize filenames to ensure they are valid for extracting.
fn sanitize_filename(filename: &str) -> String {
    filename
        .chars()
        .filter(|c| c.is_ascii_alphanumeric() || *c == '_' || *c == '-')
        .collect()
}

/// Function to extract the main archive file (ZIP, TAR, or RAR) and organize student deliverables.
pub fn extract_files(
    archive_file_path: &Path,
    destination_dir: &Path,
) -> Result<(), Box<dyn std::error::Error>> {
    // Determine the archive type and extract accordingly
    match archive_file_path.extension().and_then(|s| s.to_str()) {
        Some("zip") => extract_zip(archive_file_path, destination_dir)?,
        Some("tar") => extract_tar(archive_file_path, destination_dir)?,
        Some("rar") => extract_rar(archive_file_path, destination_dir)?,
        _ => {
            return Err(Box::new(io::Error::new(
                io::ErrorKind::InvalidInput,
                "Unsupported archive type",
            )));
        }
    }

    // Create necessary directories
    fs::create_dir_all(destination_dir.join("deliverables"))?;

    // Process each extracted assignment in the destination directory
    let mut count = 0;
    let entries: Vec<_> = fs::read_dir(destination_dir)?
        .filter_map(Result::ok)
        .filter(|entry| entry.path().is_file())
        .collect();
    let target = entries.len();

    for entry in entries {
        let path = entry.path();
        let filename = path.file_name().unwrap().to_str().unwrap();
        let mut username = get_username(filename);

        if username.is_empty() {
            username = sanitize_filename(filename);
        }

        let student_deliverable_dir = destination_dir.join("deliverables").join(&username);

        // Handle the different extracted files accordingly
        if path.extension().and_then(|s| s.to_str()) == Some("zip") {
            extract_zip(&path, &student_deliverable_dir)?;
        } else if path.extension().and_then(|s| s.to_str()) == Some("tar") {
            extract_tar(&path, &student_deliverable_dir)?;
        } else if path.extension().and_then(|s| s.to_str()) == Some("rar") {
            extract_rar(&path, &student_deliverable_dir)?;
        }

        count += 1;
        println!(
            "> Extracted {}'s deliverable. ({}/{})",
            username, count, target
        );
    }

    // Cleanup: Remove all non-deliverable files and directories from the output directory
    for entry in fs::read_dir(destination_dir)? {
        let path = entry?.path();
        if path.is_dir() && path.file_name().and_then(|s| s.to_str()) != Some("deliverables") {
            fs::remove_dir_all(path)?;
        } else if path.is_file() {
            fs::remove_file(path)?;
        }
    }

    println!("> Finished extracting files and cleaned up intermediary files!");
    Ok(())
}

fn extract_zip(
    zip_file_path: &Path,
    destination_dir: &Path,
) -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open(zip_file_path)?;
    let mut archive = ZipArchive::new(file)?;

    fs::create_dir_all(destination_dir)?;
    archive.extract(destination_dir)?;

    Ok(())
}

fn extract_tar(
    tar_file_path: &Path,
    destination_dir: &Path,
) -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open(tar_file_path)?;
    let mut archive = TarArchive::new(BufReader::new(file));

    fs::create_dir_all(destination_dir)?;
    archive.unpack(destination_dir)?;

    Ok(())
}

fn extract_rar(
    rar_file_path: &Path,
    destination_dir: &Path,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut archive = RarArchive::new(rar_file_path.to_str().unwrap())
        .open_for_processing()
        .expect("Failed to open RAR archive");

    while let Some(header) = archive.read_header()? {
        let entry = header.entry();
        let entry_filename = entry.filename.to_string_lossy();

        // Create the full path for the file to be extracted
        let output_path = destination_dir.join(&*entry_filename);

        // Ensure that the parent directory exists
        if let Some(parent_dir) = output_path.parent() {
            fs::create_dir_all(parent_dir)?;
        }

        if entry.is_file() {
            println!("Extracting file to: {}", output_path.display());
            archive = header.extract_to(output_path)?;
        } else {
            println!("Skipping non-file entry: {}", entry_filename);
            archive = header.skip()?;
        }
    }

    Ok(())
}
