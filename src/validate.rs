use crate::schemas::ValidationResult;
use async_recursion::async_recursion;
use openai_api_rs::v1::api::OpenAIClient;
use openai_api_rs::v1::chat_completion::{
    self, ChatCompletionMessage, ChatCompletionRequest, MessageRole,
};
use openai_api_rs::v1::common::GPT4_O_MINI;
use reqwest::Client;
use serde_json::from_str;
use std::env;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::Path;

/// Function to traverse a directory and validate HTML, CSS, and JS files.
pub async fn validate_directory(
    directory: &Path,
    with_ai: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();
    traverse_and_validate(directory, &client, with_ai).await?;
    Ok(())
}

/// Recursive function to traverse directories and validate files.
#[async_recursion]
async fn traverse_and_validate(
    directory: &Path,
    client: &Client,
    with_ai: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    for entry in fs::read_dir(directory)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() {
            traverse_and_validate(&path, client, with_ai).await?;
        } else if path.is_file() {
            let filename = path.file_name().unwrap().to_str().unwrap();
            if filename.ends_with(".html")
                || filename.ends_with(".css")
                || filename.ends_with(".js")
            {
                validate_file(&path, client, with_ai).await?;
            }
        }
    }
    Ok(())
}

async fn validate_file(
    file_path: &Path,
    client: &Client,
    with_ai: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    let filename = file_path.to_str().unwrap();
    let project_dir = file_path.ancestors().nth(1).unwrap(); // Extracting project directory

    // Ensure the validation file is written to the student's directory
    let feedback_file_path = project_dir.join("validate.txt");

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
        .body(content.clone())
        .send()
        .await?
        .text()
        .await?;

    let validate_file_path = file_path.with_extension("json");
    let mut file = File::create(&validate_file_path)?;
    file.write_all(response.as_bytes())?;

    println!("> Wrote response to {:?}", validate_file_path);

    // If the --with-ai flag is set, grade the deliverable using AI
    if with_ai {
        validate_with_ai(project_dir, &feedback_file_path).await?;
    }

    Ok(())
}

async fn validate_with_ai(
    project_dir: &Path,
    feedback_file_path: &Path,
) -> Result<(), Box<dyn std::error::Error>> {
    let client = OpenAIClient::new(env::var("OPENAI_API_KEY")?.to_string());

    let mut validation_issues = vec![];

    // Traverse the project directory for .json files
    for entry in fs::read_dir(project_dir)? {
        let entry = entry?;
        let path = entry.path();

        // Skip the "deliverables" directory itself
        if path.file_name().unwrap_or_default() == "deliverables" {
            continue;
        }

        if path.extension().and_then(|s| s.to_str()) == Some("json") {
            let file_content = fs::read_to_string(&path)?;
            let validation_result: ValidationResult = from_str(&file_content)?;

            for message in validation_result.messages {
                if message.message_type == "error" || message.subtype.as_deref() == Some("warning")
                {
                    validation_issues.push(format!("{}", message.message));
                }
            }
        }
    }

    let feedback = if validation_issues.is_empty() {
        "Ingen formelle feil funnet.".to_string()
    } else {
        // Construct the AI prompt
        let prompt = format!(
            "Du har mottatt en liste med HTML/CSS/JS-valideringsfeil og advarsler fra en W3C Validator. \
            For hver feilmelding, vennligst gi en kort forklaring på hva feilen betyr og et eksempel på hvordan man kan fikse det. \
            Forklaringen skal IKKE formuleres som 'Feilmeldingen indikerer at ...', men heller en direkte, kort og konsis forklaring. \
            Når du omtaler begreper innen HTML, CSS, og JS, som for eksempel 'table' etc., sørg for å bruke de engelske begrepene. \
            IKKE gjenfortell feilmeldingen. Forklar kun hva feilen betyr. \
            IKKE list opp feilene som en punktliste, men skriv en sammenhengende tekst med nye linjer mellom feil. \
            DERSOM det ikke er noen feilmeldinger, skriv 'Ingen formelle feil funnet'. \
            Hold eksempelet så kort som mulig (maks 1-5 linjer). \
            Bruk 'de' og 'dere' i flertall for å referere til studentene, ikke 'studentene'. \
            Avslutt svaret ditt med følgende setning: 'Det anbefales å bruke W3 Validator for å sjekke at HTML, CSS og JS oppfyller beste praksis.'. \
            Svarene skal være på norsk.\n\nFeilmeldinger:\n\n{}", 
            validation_issues.join("\n\n") 
        );

        let req = ChatCompletionRequest::new(
            GPT4_O_MINI.to_string(),
            vec![ChatCompletionMessage {
                role: MessageRole::user,
                content: chat_completion::Content::Text(prompt),
                name: None,
                tool_calls: None,
                tool_call_id: None,
            }],
        );

        let result = client.chat_completion(req).await?;
        result
            .choices
            .get(0)
            .and_then(|choice| choice.message.content.as_deref())
            .unwrap_or("Ingen formelle feil funnet.")
            .to_string()
    };

    // Write the feedback to the student's feedback file
    let mut feedback_file = File::create(feedback_file_path)?;
    writeln!(
        feedback_file,
        "Tilbakemelding om validering: \n\n{}",
        feedback
    )?;

    println!("Feedback written to {:?}", feedback_file_path);

    Ok(())
}
