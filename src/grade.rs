use crate::pdf::parse_pdf;
use openai_api_rs::v1::api::OpenAIClient;
use openai_api_rs::v1::chat_completion::{
    self, ChatCompletionMessage, ChatCompletionRequest, MessageRole,
};
use openai_api_rs::v1::common::GPT4;
use std::env;
use std::error::Error;
use std::fs;
use std::path::Path;
use walkdir::WalkDir;

/// Function to read relevant project files (HTML, CSS, and JS) and format them with filename, extension, and content.
async fn format_project_files(project_dir: &Path) -> Result<String, Box<dyn Error>> {
    let mut output = String::new();

    // Walk through the directory and process only HTML, CSS, and JS files recursively
    for entry in WalkDir::new(project_dir) {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() {
            continue; // Skip directories
        }

        let extension = path.extension().and_then(|s| s.to_str()).unwrap_or("");

        // Only process files with extensions "html", "css", or "js"
        if extension == "html" || extension == "css" || extension == "js" {
            let filename = path.display().to_string();
            let contents = match fs::read_to_string(&path) {
                Ok(contents) => contents,
                Err(_) => continue, // Skip files that cannot be read
            };

            let formatted = format!("`{}`\n\n```{}\n{}\n```\n\n", filename, extension, contents);
            output.push_str(&formatted);
        }
    }

    Ok(output)
}

/// Function to process each deliverable, combining the assignment description, grading criteria, and the student's project files.
pub async fn grade_directory(
    destination_dir: &Path,
    description_pdf: &Path,
    criteria_pdf: &Path,
) -> Result<(), Box<dyn Error>> {
    // Parse the assignment description PDF
    let description_text = parse_pdf(description_pdf)?;

    // Parse the grading criteria PDF
    let criteria_text = parse_pdf(criteria_pdf)?;

    // Create an OpenAI client using the API key from the environment
    let api_key = env::var("OPENAI_API_KEY")?;
    let client = OpenAIClient::new(api_key);

    // Process each deliverable directory
    let deliverables_dir = destination_dir.join("deliverables");
    for entry in fs::read_dir(&deliverables_dir)? {
        let entry = entry?;
        let student_dir = entry.path();

        if student_dir.is_dir() {
            println!("Processing deliverable for: {}", student_dir.display());

            // Format the project files for the current student deliverable
            let formatted_project_files = format_project_files(&student_dir).await?;

            // Construct the AI prompt
            let prompt = format!(
                "Du har mottatt en studentinnlevering sammen med prosjektbeskrivelsen og vurderingskriteriene for et prosjekt i webteknologi (HTML, CSS, JS). \
                Gå gjennom innleveringen og evaluer hvor godt den oppfyller følgende krav fra oppgavebeskrivelsen og vurderingskriteriene. \
                For hver del, gi en kort og konsis forklaring på hvordan oppgaven tilfredsstiller eller ikke tilfredsstiller kravene. \
                Hvis en del mangler, forklar hva som mangler og hvordan det bør implementeres. \
                Forklaringen skal IKKE formuleres som 'Feilmeldingen indikerer at ...', men heller direkte og kort. \
                IKKE list opp feilene som en punktliste, men skriv en sammenhengende tekst med nye linjer mellom feil. \
                Hold eksemplene korte (maks 1-5 linjer). Svarene skal være på norsk.\n\n \
                DERSOM studentene skal besvare spesifikke spørsmål, ta et øyeblikk og tenk over om deres avgitte svar er korrekte. \
                Avslutt med å gi et forslag til antall poeng, BASERT PÅ vurderingskriteriene og hver dels oppfyllelse av kravene. \
                Formuler forslaget slik: 'Foreslått poengsum: X av Y'.\n\n \
                Bruk 'studentene' i flertall for å referere til studentene. \
                Oppgavebeskrivelse:\n\n{}\n\n \
                Vurderingskriterier:\n\n{}\n\n \
                Studentens innlevering:\n\n{}",
                description_text, criteria_text, formatted_project_files
            );

            // Create the chat completion request for GPT
            let request = ChatCompletionRequest::new(
                GPT4.to_string(),
                vec![ChatCompletionMessage {
                    role: MessageRole::user,
                    content: chat_completion::Content::Text(prompt),
                    name: None,
                    tool_calls: None,
                    tool_call_id: None,
                }],
            );

            // Send the request to GPT and get the feedback
            let result = client.chat_completion(request).await?;
            let feedback = result
                .choices
                .get(0)
                .and_then(|choice| choice.message.content.as_deref())
                .unwrap_or("Ingen tilbakemelding generert.")
                .to_string();

            // Save the feedback to a file
            let feedback_file_path = student_dir.join("feedback.txt");
            fs::write(&feedback_file_path, feedback)?;
        }
    }

    Ok(())
}
