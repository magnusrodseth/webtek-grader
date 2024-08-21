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

**If you want to use the `Grade with AI` feature**, ensure you have an OpenAI API key. If not, you can get one by following the instructions [here](https://beta.openai.com/signup/). Next, create a `.env` file in the root of the project directory and add the following:

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

## Developer Information

Developed by [Magnus Rødseth](https://github.com/magnusrodseth).
