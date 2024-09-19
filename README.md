# Web Technologies Grader

## What is it?

This project is developed out of frustration related to the tedious process of downloading and grading student deliverables. It aids in the process of unzipping the student deliverables, and leverages GPT to generate a proposal for the student feedback.

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

# Unzip a deliverable
webtek-grader extract <path-to-zip-file> <output-directory>

# Grade a deliverable (with AI)
webtek-grader grade --with-ai <path-to-unzipped-directory>

# Grade a deliverable (without AI)
webtek-grader grade <path-to-unzipped-directory>
```

## Example

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

### Generated feedback

```txt
Feedback til <student username> (__%):

Verdien “300px” for attributten “width” på elementet “img” er ugyldig. Attributter for bredde og høyde skal kun spesifiseres med tall, så her skal “300” være brukt uten “px”. Eksempel: `<img src="bilde.jpg" width="300">`.

Et “img”-element må ha et “alt”-attributt for å gi tekstalternativer til bilder, noe som er viktig for tilgjengelighet. Eksempel: `<img src="bilde.jpg" alt="Beskrivelse av bildet">`.

Det finnes ikke noe “p”-element i scope, men det er funnet en avsluttende “p”-tag. Dette betyr at det er en feil bruk av parantes, og avsluttende tagger bør kun brukes hvis det er et tilhørende åpningstag. Eksempel: Hvis det er en ubrukt “p”-tag, fjern den eller legg til en matchende åpningstag.

Det anbefales å bruke “h1”-elementet kun som et toppnivå overskrift, da skjermlesere og verktøy betrakter alle “h1”-elementer som toppnivå overskrifter. Bruk riktig hierarki, for eksempel: `<h1>Tittel</h1>` for hovedtittelen.

Det kan være nyttig å legge til et “lang”-attributt i “html”-starttaggen for å deklarere språket i dokumentet. Dette forbedrer tilgjengeligheten for brukere som bruker skjermlesere. Eksempel: `<html lang="no">`.
```

## Developer Information

Developed by [Magnus Rødseth](https://github.com/magnusrodseth).
