use crate::schemas::ValidationResult;
use openai_api_rs::v1::api::OpenAIClient;
use openai_api_rs::v1::chat_completion::{
    self, ChatCompletionMessage, ChatCompletionRequest, MessageRole,
};
use openai_api_rs::v1::common::GPT4_O_MINI;
use reqwest::Client;
use std::env;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use walkdir::WalkDir;

/// Function to traverse a directory and validate HTML, CSS, and JS files.
pub async fn validate_directory(
    destination_dir: &Path,
    with_ai: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();

    let deliverables_dir = destination_dir.join("deliverables");
    for entry in fs::read_dir(&deliverables_dir)? {
        let entry = entry?;
        let student_dir = entry.path();

        if student_dir.is_dir() {
            println!("Validating student directory: {}", student_dir.display());
            // Collect validation issues for the student directory
            let validation_issues = collect_and_validate_files(&student_dir, &client).await?;

            // If with_ai is true, generate AI feedback and write to validate.txt
            if with_ai {
                validate_with_ai(&student_dir, &validation_issues).await?;
            }
        }
    }

    Ok(())
}

async fn collect_and_validate_files(
    student_dir: &Path,
    client: &Client,
) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let mut validation_issues = Vec::new();

    for entry in WalkDir::new(&student_dir) {
        let entry = entry?;
        let path = entry.path();

        if path.is_file() {
            let filename = path.file_name().unwrap().to_str().unwrap();
            if filename.ends_with(".html")
                || filename.ends_with(".css")
                || filename.ends_with(".js")
            {
                println!("> Validating file: {}", path.display());
                let issues = validate_file(&path, client).await?;
                validation_issues.extend(issues);
            }
        }
    }

    Ok(validation_issues)
}

async fn validate_file(
    file_path: &Path,
    client: &Client,
) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let mut validation_issues = Vec::new();

    let filename = file_path.to_str().unwrap();

    // Determine content type based on file extension
    let content_type = if filename.ends_with(".html") {
        "text/html"
    } else if filename.ends_with(".css") {
        "text/css"
    } else if filename.ends_with(".js") {
        "text/javascript"
    } else {
        return Ok(validation_issues);
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

    // Parse the validation response and collect issues
    let validation_result: ValidationResult = serde_json::from_str(&response)?;

    for message in validation_result.messages {
        if message.message_type == "error" || message.subtype.as_deref() == Some("warning") {
            validation_issues.push(format!("{}", message.message));
        }
    }

    Ok(validation_issues)
}

async fn validate_with_ai(
    project_dir: &Path,
    validation_issues: &Vec<String>,
) -> Result<(), Box<dyn std::error::Error>> {
    let client = OpenAIClient::new(env::var("OPENAI_API_KEY")?.to_string());

    let feedback_file_path = project_dir.join("validate.txt");

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
    let mut feedback_file = File::create(&feedback_file_path)?;
    writeln!(
        feedback_file,
        "Tilbakemelding om validering: \n\n{}",
        feedback
    )?;

    // Also write the same feedback to final.txt (overwriting if it exists)
    let final_file_path = project_dir.join("final.txt");
    let mut final_file = File::create(&final_file_path)?;
    writeln!(final_file, "Tilbakemelding om validering: \n\n{}", feedback)?;

    println!("Feedback written to {:?}", &feedback_file_path);

    Ok(())
}
