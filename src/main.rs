use clap::{Parser, Subcommand};
use std::path::PathBuf;

mod grade;
mod unzip;

/// CLI structure using `clap`
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Unzip a file to a destination directory
    Unzip {
        /// The zip file to unzip
        zip_file: PathBuf,
        /// The destination directory
        destination_dir: PathBuf,
    },
    /// Grade a directory
    Grade {
        /// The directory to grade
        directory: PathBuf,
    },
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Unzip {
            zip_file,
            destination_dir,
        } => {
            if let Err(e) = unzip::unzip_files(zip_file, destination_dir) {
                eprintln!("Error unzipping file: {:?}", e);
            }
        }
        Commands::Grade { directory } => {
            if let Err(e) = grade::validate_directory(directory).await {
                eprintln!("Error grading directory: {:?}", e);
            }
        }
    }
}
