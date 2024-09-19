use clap::{Parser, Subcommand};
use dotenv::dotenv;
use std::env;
use std::path::PathBuf;

mod extract;
mod grade;
mod pdf;
mod schemas;
mod validate;

/// CLI structure using `clap`
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Extract and validate without AI
    WithoutAI {
        /// The archive file to extract (supports ZIP, TAR, RAR)
        archive_file: PathBuf,
        /// The destination directory
        destination_dir: PathBuf,
    },
    /// Extract, validate, and grade with AI
    WithAI {
        /// The archive file to extract (supports ZIP, TAR, RAR)
        archive_file: PathBuf,
        /// The destination directory
        destination_dir: PathBuf,
        /// Path to the assignment description PDF
        description_file: PathBuf,
        /// Path to the grading criteria PDF
        criteria_file: PathBuf,
    },
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::WithoutAI {
            archive_file,
            destination_dir,
        } => {
            if let Err(e) = extract::extract_files(archive_file, destination_dir) {
                eprintln!("Error extracting file: {:?}", e);
            }

            if let Err(e) = validate::validate_directory(destination_dir, false).await {
                eprintln!("Error during validation: {:?}", e);
            }

            println!("✅ Finished extracting and validating deliverables.");
        }
        Commands::WithAI {
            archive_file,
            destination_dir,
            description_file,
            criteria_file,
        } => {
            dotenv().ok(); // Ensure .env is loaded
            if env::var("OPENAI_API_KEY").is_err() {
                eprintln!("Error: OPENAI_API_KEY environment variable is not set.");
                std::process::exit(1);
            }

            if let Err(e) = extract::extract_files(archive_file, destination_dir) {
                eprintln!("Error extracting file: {:?}", e);
            }

            // Validate the extracted files
            if let Err(e) = validate::validate_directory(destination_dir, true).await {
                eprintln!("Error during validation: {:?}", e);
            }

            // Now call the grade function with the description, criteria, and deliverables
            if let Err(e) =
                grade::grade_directory(destination_dir, description_file, criteria_file).await
            {
                eprintln!("Error during grading: {:?}", e);
            }

            println!("✅ Finished extracting, validating, and grading deliverables.");
        }
    }
}
