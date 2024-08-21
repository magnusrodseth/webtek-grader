use std::fs::{self, File};
use std::io::{self, BufReader};
use std::path::{Path, PathBuf};
use tar::Archive as TarArchive;
use unrar::Archive as RarArchive;
use zip::read::ZipArchive;

/// Enum representing different types of archives.
enum ArchiveType {
    Zip,
    Tar,
    Rar,
}

/// Trait that defines the behavior for extracting files from an archive.
trait ArchiveExtractor {
    fn extract(
        &self,
        archive_path: &Path,
        destination_dir: &Path,
    ) -> Result<(), Box<dyn std::error::Error>>;
}

/// Struct for extracting ZIP files.
struct ZipExtractor;

impl ArchiveExtractor for ZipExtractor {
    fn extract(
        &self,
        archive_path: &Path,
        destination_dir: &Path,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let file = File::open(archive_path)?;
        let mut archive = ZipArchive::new(file)?;

        fs::create_dir_all(destination_dir)?;
        archive.extract(destination_dir)?;

        Ok(())
    }
}

/// Struct for extracting TAR files.
struct TarExtractor;

impl ArchiveExtractor for TarExtractor {
    fn extract(
        &self,
        archive_path: &Path,
        destination_dir: &Path,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let file = File::open(archive_path)?;
        let mut archive = TarArchive::new(BufReader::new(file));

        fs::create_dir_all(destination_dir)?;
        archive.unpack(destination_dir)?;

        Ok(())
    }
}

/// Struct for extracting RAR files.
struct RarExtractor;

impl ArchiveExtractor for RarExtractor {
    fn extract(
        &self,
        archive_path: &Path,
        destination_dir: &Path,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut archive = RarArchive::new(archive_path.to_str().unwrap())
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
}

/// Factory for creating the appropriate extractor based on the file extension.
struct ArchiveExtractorFactory;

impl ArchiveExtractorFactory {
    fn create_extractor(archive_type: ArchiveType) -> Box<dyn ArchiveExtractor> {
        match archive_type {
            ArchiveType::Zip => Box::new(ZipExtractor),
            ArchiveType::Tar => Box::new(TarExtractor),
            ArchiveType::Rar => Box::new(RarExtractor),
        }
    }

    fn from_extension(extension: &str) -> Result<ArchiveType, io::Error> {
        match extension {
            "zip" => Ok(ArchiveType::Zip),
            "tar" => Ok(ArchiveType::Tar),
            "rar" => Ok(ArchiveType::Rar),
            _ => Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "Unsupported archive type",
            )),
        }
    }
}

/// Function to extract the username from the assignment filename.
fn get_username(assignment: &str) -> String {
    assignment.split('_').nth(1).unwrap_or("").to_string()
}

/// Function to sanitize filenames to ensure they are valid for extracting.
fn sanitize_filename(filename: &str) -> String {
    filename
        .chars()
        .filter(|c| c.is_ascii_alphanumeric() || *c == '_' || *c == '-')
        .collect()
}

/// Function to remove __MACOSX directories from the destination directory.
fn remove_macosx_directories(directory: &Path) -> Result<(), Box<dyn std::error::Error>> {
    for entry in fs::read_dir(directory)? {
        let path = entry?.path();
        if path.is_dir() {
            if path.file_name().and_then(|s| s.to_str()) == Some("__MACOSX") {
                fs::remove_dir_all(&path)?;
            } else {
                // Recursively check inside subdirectories
                remove_macosx_directories(&path)?;
            }
        }
    }
    Ok(())
}

/// Function to extract the main archive file (ZIP, TAR, or RAR) and organize student deliverables.
pub fn extract_files(
    archive_file_path: &Path,
    destination_dir: &Path,
) -> Result<(), Box<dyn std::error::Error>> {
    // Determine the archive type and get the corresponding extractor
    let extension = archive_file_path
        .extension()
        .and_then(|s| s.to_str())
        .ok_or_else(|| {
            io::Error::new(
                io::ErrorKind::InvalidInput,
                "Missing or invalid file extension",
            )
        })?;

    let archive_type = ArchiveExtractorFactory::from_extension(extension)?;
    let extractor = ArchiveExtractorFactory::create_extractor(archive_type);

    extractor.extract(archive_file_path, destination_dir)?;

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
        if let Ok(archive_type) = ArchiveExtractorFactory::from_extension(
            path.extension().and_then(|s| s.to_str()).unwrap_or(""),
        ) {
            let extractor = ArchiveExtractorFactory::create_extractor(archive_type);
            extractor.extract(&path, &student_deliverable_dir)?;
        }

        count += 1;
        println!(
            "> Extracted {}'s deliverable. ({}/{})",
            username, count, target
        );
    }

    // Remove __MACOSX directories if found
    remove_macosx_directories(destination_dir)?;

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
