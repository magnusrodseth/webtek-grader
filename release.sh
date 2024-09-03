#!/bin/bash

# Function to display versioning information
function show_version_info() {
  echo "Versioning Guide:"
  echo "[1] Major - Increment the first number (e.g., 1.0.0 -> 2.0.0)"
  echo "[2] Minor - Increment the second number (e.g., 1.0.0 -> 1.1.0)"
  echo "[3] Patch - Increment the third number (e.g., 1.0.0 -> 1.0.1)"
}

# Function to bump the version based on user input
function bump_version() {
  # Extract current version from Cargo.toml
  current_version=$(grep '^version =' Cargo.toml | sed -E 's/version = "(.*)"/\1/')

  # Split the version into its components
  IFS='.' read -r -a version_parts <<<"$current_version"

  # Ask the user for the type of version bump
  echo "Select the version type to bump:"
  show_version_info
  read -p "Enter [1] for Major, [2] for Minor, [3] for Patch: " version_type

  case $version_type in
  1)
    version_parts[0]=$((version_parts[0] + 1))
    version_parts[1]=0
    version_parts[2]=0
    ;;
  2)
    version_parts[1]=$((version_parts[1] + 1))
    version_parts[2]=0
    ;;
  3)
    version_parts[2]=$((version_parts[2] + 1))
    ;;
  *)
    echo "Invalid option selected."
    exit 1
    ;;
  esac

  # Construct the new version string
  new_version="${version_parts[0]}.${version_parts[1]}.${version_parts[2]}"

  # Update the version in Cargo.toml
  sed -i.bak -E "s/version = \"$current_version\"/version = \"$new_version\"/" Cargo.toml

  echo "Version bumped from $current_version to $new_version"

  # Clean up the backup file created by sed on macOS
  rm Cargo.toml.bak
}

# Function to commit the change
function commit_version_change() {
  git add Cargo.toml
  git commit -m "Bump version to $new_version"
  echo "Committed the new version."
}

# Function to publish the new version
function publish_crate() {
  cargo publish
  echo "Published version $new_version to crates.io."
}

# Check for uncommitted changes
if [[ -n $(git status --porcelain) ]]; then
  echo "You have uncommitted changes. Please commit or stash them before running this script."
  exit 1
fi

# Run the functions
bump_version
commit_version_change
publish_crate
