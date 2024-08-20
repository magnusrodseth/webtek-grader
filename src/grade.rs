use async_recursion::async_recursion;
use reqwest::Client;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::Path;

/// Function to traverse a directory and validate HTML, CSS, and JS files.
pub async fn validate_directory(directory: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();
    traverse_and_validate(directory, &client).await?;
    Ok(())
}

/// Recursive function to traverse directories and validate files.
#[async_recursion]
async fn traverse_and_validate(
    directory: &Path,
    client: &Client,
) -> Result<(), Box<dyn std::error::Error>> {
    for entry in fs::read_dir(directory)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() {
            traverse_and_validate(&path, client).await?;
        } else if path.is_file() {
            let filename = path.file_name().unwrap().to_str().unwrap();
            if filename.ends_with(".html")
                || filename.ends_with(".css")
                || filename.ends_with(".js")
            {
                validate_file(&path, client).await?;
            }
        }
    }
    Ok(())
}

/// Function to validate a file using the W3C Validator.
async fn validate_file(
    file_path: &Path,
    client: &Client,
) -> Result<(), Box<dyn std::error::Error>> {
    let filename = file_path.to_str().unwrap();

    // Determine content type based on file extension
    let content_type = if filename.ends_with(".html") {
        "text/html"
    } else if filename.ends_with(".css") {
        "text/css"
    } else if filename.ends_with(".js") {
        "text/javascript"
    } else {
        return Ok(());
    };

    println!("> Posting file to W3 Validator: {}", filename);

    let content = fs::read_to_string(file_path)?;
    let response = client
        .post("https://validator.w3.org/nu/?out=json")
        .header("Content-Type", format!("{}; charset=utf-8", content_type))
        .header("User-Agent", "Mozilla/5.0 (compatible; Validator/1.0)")
        .body(content)
        .send()
        .await?
        .text()
        .await?;

    let validate_file_path = file_path.with_extension("json");
    let mut file = File::create(&validate_file_path)?;
    file.write_all(response.as_bytes())?;

    println!("> Wrote response to {:?}", validate_file_path);

    Ok(())
}
