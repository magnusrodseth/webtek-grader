# Web Technologies Grader

## What is it?

Developed out of frustration related to the tedious process of downloading and grading student deliverables.

This repository aids in the process of unzipping the student deliverables, and leverages GPT to generate a proposal for the student feedback.

## Getting started

### Prerequisites

Ensure yoou have Rust and Cargo installed on your machine. If not, you can install it by following the instructions [here](https://www.rust-lang.org/tools/install).

**If you want to use the `Grade with AI` feature**, ensure you have an OpenAI API key. If not, you can get one by following the instructions [here](https://beta.openai.com/signup/). Next, create a `.env` file in the root of the project directory and add the following:

```sh
OPENAI_API_KEY=<your-openai-api-key>
```

Alternatively, you can set the `OPENAI_API_KEY` environment variable in your terminal using `export OPENAI_API_KEY=<your-openai-api-key>`, followed by running the application.

> **I recommend using the `.env` file approach as it is easier to manage across sessions.**

### Running the application

```sh
# Navigate to the project directory
cd webtek-grader

# Display the help message
cargo run -- --help

# Unzip a deliverable
cargo run -- unzip <path-to-zip-file> <output-directory>

# Grade a deliverable (with AI)
cargo run -- grade --with-ai <path-to-unzipped-directory>

# Grade a deliverable (without AI)
cargo run -- grade <path-to-unzipped-directory>
```

## Developer Information

Developed by [Magnus Rødseth](https://github.com/magnusrodseth).
