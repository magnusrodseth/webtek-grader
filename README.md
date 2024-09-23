# Web Technologies Grader

## What is it?

This project is developed out of frustration related to the tedious process of downloading and grading student deliverables. It aids in the process of unzipping the student deliverables, and leverages GPT to generate a proposal for the student feedback.

## Features

- 📂 **Extract deliverables**: Extracts the student deliverables from a compressed file.
- 🧪 **Validate deliverables**: Validates the HTML, CSS and JS using the W3C Validator API.
- 🧠 **Grade deliverables with AI**: Grades the deliverables using the project description, all project files for the deliverable, and the grading criteria. This is optional, and can be run without AI.

## 🚨 Very important to note

This project is only meant to be a guideline when grading assignments, **not a one-stop shop**. It is important to review the generated feedback and adjust it to fit the student's deliverable. Every single deliverable requires a human eye to evaluate the points given and the feedback provided.

**Please do not use this blindly without reviewing the feedback generated. 🫶🏽**

## Getting started

### Prerequisites

Ensure you have Rust and Cargo installed on your machine. If not, you can install it by following the instructions [here](https://www.rust-lang.org/tools/install). If you are on MacOS or Linux, you can follow the instructions below:

```sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

If you're on Windows, you can download the installer from [here](https://forge.rust-lang.org/infra/other-installation-methods.html#other-ways-to-install-rustup).

**If you want to use the `Grade with AI` feature**, ensure you have an OpenAI API key. If not, you can get one by following the instructions [here](https://platform.openai.com/docs/quickstart). Next, create a `.env` file in the root of the project directory and add the following:

```sh
OPENAI_API_KEY=<your-openai-api-key>
```

Alternatively, you can set the `OPENAI_API_KEY` environment variable in your terminal using `export OPENAI_API_KEY=<your-openai-api-key>`, followed by running the application.

> **I recommend using the `.env` file approach as it is easier to manage across sessions.**

For instance, if you're grading deliverables in a directory called `webtek`, the directory structure should look like this:

```sh
webtek/
└── .env
```

### Running the application

```sh
# Install the application
cargo install webtek-grader

# Display the help message
webtek-grader --help

# Extract and validate without AI
webtek-grader without-ai <archive-file> <destination-directory>

# Extract, validate and grade with AI
webtek-grader with-ai <archive-file> <destination-directory> <description-file> <criteria-file>
```

## How does grading with AI work?

As described above, ensure you have an `OPENAI_API_KEY` environment variable set in your terminal or a `.env` file in the root of the project directory.

The `archive-file` is the path to the compressed file containing the student deliverables.

The `destination-directory` is the directory where the deliverables will be extracted, e.g. `assignment-1`.

The `description-file` is the path to a PDF file for the assignment description. **Ensure this is a PDF file, and not any other file extension**.

The `criteria-file` is the path to the grading criteria for the assignment. **Ensure this is a PDF file, and not any other file extension**.

### The pipeline when grading with AI

1. The script starts by **extracting the deliverables**.

2. Next, it **validates** the HTML, CSS and JS using the W3C Validator API. When running this with AI, the errors and warning from W3C Validator are input to the GPT model, and a `validate.txt` file is generated with the validation feedback for that group.

3. Next, the deliverable is **graded** using the project description, all project files for the deliverable, and the grading criteria. The GPT model outputs feedback and a suggested score for the deliverable in the `feedback.txt` file.

4. **For your convenience, the script creates a `final.txt` which contains the validation feedback and grading feedback for the deliverable in one file.**

## Example of validating with AI

Here is an example of some errors and warnings from W3C Validator, and the respective generated feedback:

### Response from W3C Validator

```json
{
  "messages": [
    {
      "type": "error",
      "message": "Bad value “300px” for attribute “width” on element “img”: Expected a digit but saw “p” instead."
    },
    {
      "type": "error",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images."
    },
    {
      "type": "error",
      "message": "No “p” element in scope but a “p” end tag seen."
    },
    {
      "type": "info",
      "subType": "warning",
      "message": "Consider using the “h1” element as a top-level heading only (all “h1” elements are treated as top-level headings by many screen readers and other tools)."
    },
    {
      "type": "info",
      "subType": "warning",
      "message": "Consider adding a “lang” attribute to the “html” start tag to declare the language of this document."
    }
  ]
}
```

### Generated feedback for validating

```txt
Tilbakemelding om validering:

Verdien “300px” for attributten “width” på elementet “img” er ugyldig. Attributter for bredde og høyde skal kun spesifiseres med tall, så her skal “300” være brukt uten “px”. Eksempel: `<img src="bilde.jpg" width="300">`.

Et “img”-element må ha et “alt”-attributt for å gi tekstalternativer til bilder, noe som er viktig for tilgjengelighet. Eksempel: `<img src="bilde.jpg" alt="Beskrivelse av bildet">`.

Det finnes ikke noe “p”-element i scope, men det er funnet en avsluttende “p”-tag. Dette betyr at det er en feil bruk av parantes, og avsluttende tagger bør kun brukes hvis det er et tilhørende åpningstag. Eksempel: Hvis det er en ubrukt “p”-tag, fjern den eller legg til en matchende åpningstag.

Det anbefales å bruke “h1”-elementet kun som et toppnivå overskrift, da skjermlesere og verktøy betrakter alle “h1”-elementer som toppnivå overskrifter. Bruk riktig hierarki, for eksempel: `<h1>Tittel</h1>` for hovedtittelen.

Det kan være nyttig å legge til et “lang”-attributt i “html”-starttaggen for å deklarere språket i dokumentet. Dette forbedrer tilgjengeligheten for brukere som bruker skjermlesere. Eksempel: `<html lang="no">`.
```

## Example of grading with AI

Here is an example of a project description and grading criteria, and the respective generated feedback:

### Excerpt from project description and grading criteria

```md
# Project Description

Create a new section element, below your previous, but above the footer.
Add a header element to it, and fill it with an h2 tag containing the title "Questions".
Then make a table with 2 columns and seven rows.
The first row must be the table header with “Questions” and “Answers”.
In each of the remaining six rows add one of the following questions and write their answers:

...

# Grading Criteria

Is the placement of the section correct? 1.5 points
Is the new header added properly? 1.5 points
Are the questions answered correctly? 1 point for each correct answer
Is the table created correctly? 6 points
Is the table rendering properly? 6 points
Does the columns have headings? 2 points for each heading
```

### Generated feedback for grading

```txt
Denne delen inkluderer en ny seksjon med overskriften (h2) "Questions".
I denne seksjonen inkluderer studenten en tabell med syv rader og to kolonner.
Kan følge lenken til denne seksjonen ved hjelp av ankeret "questions".
Spørsmål og svar på spørsmålene oppgitt i oppgavebeskrivelsen er inkludert i tabellen.
Dette oppfyller alle kravene i del 3.
```

## Developer Information

Developed by [Magnus Rødseth](https://github.com/magnusrodseth).
