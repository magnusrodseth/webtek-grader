use std::path::Path;

use pdf_extract::extract_text_from_mem;

pub fn parse_pdf(file_path: &Path) -> Result<String, Box<dyn std::error::Error>> {
    let bytes = std::fs::read(file_path)?;
    let extracted_text = extract_text_from_mem(&bytes)?;
    Ok(extracted_text)
}
