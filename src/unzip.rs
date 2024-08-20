use std::fs::{self, File};
use std::io::{self, Read};
use std::path::{Path, PathBuf};
use zip::read::ZipArchive;

/// Function to extract the username from the assignment filename.
fn get_username(assignment: &str) -> String {
    // Attempt to extract the username by splitting on "_"
    assignment.split('_').nth(1).unwrap_or("").to_string()
}

/// Function to sanitize filenames to ensure they are valid for unzipping.
fn sanitize_filename(filename: &str) -> String {
    filename
        .chars()
        .filter(|c| c.is_ascii_alphanumeric() || *c == '_' || *c == '-')
        .collect()
}

/// Function to unzip the main zip file and organize student deliverables.
pub fn unzip_files(
    zip_file_path: &Path,
    destination_dir: &Path,
) -> Result<(), Box<dyn std::error::Error>> {
    // Ensure the provided file is a zip file
    if zip_file_path.extension().and_then(|s| s.to_str()) != Some("zip") {
        return Err(Box::new(io::Error::new(
            io::ErrorKind::InvalidInput,
            "Not a zip file",
        )));
    }

    // Create the destination directory if it doesn't exist
    fs::create_dir_all(destination_dir)?;

    // Unzip the main zip file into the destination directory
    let file = File::open(zip_file_path)?;
    let mut archive = ZipArchive::new(file)?;

    archive.extract(destination_dir)?;

    // Navigate to the destination directory
    std::env::set_current_dir(destination_dir)?;

    // Create necessary directories
    fs::create_dir_all("deliverables")?;

    // Remove redundant .txt files
    for entry in fs::read_dir(".")? {
        let entry = entry?;
        let path = entry.path();
        if path.extension().and_then(|s| s.to_str()) == Some("txt") {
            fs::remove_file(path)?;
        }
    }

    // Process each zipped assignment in the destination directory
    let mut count = 0;
    let zip_files: Vec<_> = fs::read_dir(".")?
        .filter_map(Result::ok)
        .filter(|entry| entry.path().extension().and_then(|s| s.to_str()) == Some("zip"))
        .collect();
    let target = zip_files.len();

    for entry in zip_files {
        let path = entry.path();
        let filename = path.file_name().unwrap().to_str().unwrap();
        let mut username = get_username(filename);

        if username.is_empty() {
            username = sanitize_filename(filename);
        }

        let student_deliverable_dir = Path::new("deliverables").join(&username);

        // Unzip student file
        let file = File::open(&path)?;
        let mut student_archive = ZipArchive::new(file)?;

        if student_archive.extract(&student_deliverable_dir).is_ok() {
            count += 1;
            println!(
                "> Unzipped {}'s deliverable. ({}/{})",
                username, count, target
            );
        } else {
            println!("> Could not unzip student's zipped assignment.");
        }
    }

    // Generate feedback files for each student
    for entry in fs::read_dir(".")? {
        let path = entry?.path();
        if path.extension().and_then(|s| s.to_str()) == Some("zip") {
            let filename = path.file_name().unwrap().to_str().unwrap();
            let mut username = get_username(filename);

            if username.is_empty() {
                username = sanitize_filename(filename);
            }

            let feedback_file_path = Path::new("feedback").join(format!("{}.txt", username));
            fs::write(
                &feedback_file_path,
                format!("Tilbakemelding til {} (__%)", username),
            )?;
        }
    }

    // Final cleanup: Remove any remaining zip files in the destination directory
    for entry in fs::read_dir(".")? {
        let path = entry?.path();
        if path.extension().and_then(|s| s.to_str()) == Some("zip") {
            fs::remove_file(path)?;
        }
    }

    println!("> Finished unzipping files!");
    Ok(())
}
