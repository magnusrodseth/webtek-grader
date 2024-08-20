use clap::{Parser, Subcommand};
use dotenv::dotenv;
use std::env;
use std::path::PathBuf;

mod grade;
mod schemas;
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
        /// Optionally grade with AI
        #[arg(long)]
        with_ai: bool,
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
        Commands::Grade { directory, with_ai } => {
            // Load the .env file if the --with-ai flag is set
            if *with_ai {
                dotenv().ok();
            }

            // Check for the OPENAI_API_KEY environment variable
            if *with_ai && env::var("OPENAI_API_KEY").is_err() {
                eprintln!("Error: --with-ai flag is set, but OPENAI_API_KEY environment variable is not set.");
                std::process::exit(1);
            }

            if let Err(e) = grade::validate_directory(directory, *with_ai).await {
                eprintln!("Error grading directory: {:?}", e);
            }
        }
    }
}
