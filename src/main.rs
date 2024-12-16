use std::process::Command;
use std::fs;
use std::env;
use std::path::Path;
use std::io::{Write, Read};
use chrono::{Utc, Duration};
use reqwest;
use serde_json::Value;

const CURRENT_VERSION: &str = "1.0.1"; // Update this with your current version
const REPO_URL: &str = "https://github.com/CooperDActor-bytes/sadl";
const CONFIG_FILE: &str = ".sadl_config";

fn main() {
    let args: Vec<String> = env::args().collect();

    // Check for updates automatically every 5 days
    if should_check_for_update() {
        if let Err(e) = check_for_update() {
            eprintln!("Failed to check for updates: {}", e);
        }
    }

    if args.len() > 1 {
        match args[1].as_str() {
            "--update" => {
                if let Err(e) = check_for_update() {
                    eprintln!("Failed to update: {}", e);
                }
            }
            "--help" => {
                println!("Usage: sadl [--update] [other commands]");
            }
            _ => {
                eprintln!("Unknown command: {}", args[1]);
            }
        }
    } else {
        println!("Usage: sadl [--update] [other commands]");
    }
}

/// Checks if it’s been more than 5 days since the last update check
fn should_check_for_update() -> bool {
    let config_path = dirs::home_dir().unwrap().join(CONFIG_FILE);

    if let Ok(mut file) = fs::File::open(&config_path) {
        let mut contents = String::new();
        if file.read_to_string(&mut contents).is_ok() {
            if let Ok(last_check) = contents.trim().parse::<i64>() {
                let last_check_date = chrono::NaiveDateTime::from_timestamp(last_check, 0);
                let now = Utc::now().naive_utc();

                if now - last_check_date < Duration::days(5) {
                    return false;
                }
            }
        }
    }

    // Update the last check timestamp if it's due
    if let Err(e) = update_last_check_timestamp(&config_path) {
        eprintln!("Failed to update last check timestamp: {}", e);
    }

    true
}

/// Updates the last update check timestamp in the config file
fn update_last_check_timestamp(config_path: &std::path::PathBuf) -> std::io::Result<()> {
    let now = Utc::now().timestamp();
    let mut file = fs::File::create(config_path)?;
    write!(file, "{}", now)?;
    Ok(())
}

/// Checks for a new version and prompts the user to update if needed
fn check_for_update() -> Result<(), Box<dyn std::error::Error>> {
    println!("Checking for updates...");

    // Fetch the latest release tag from GitHub API
    let api_url = format!("{}/releases/latest", REPO_URL);
    let response = reqwest::blocking::get(&api_url)?
        .json::<Value>()?;

    if let Some(latest_version) = response["tag_name"].as_str() {
        if latest_version != CURRENT_VERSION {
            println!(
                "New version available: {} (current version: {})",
                latest_version, CURRENT_VERSION
            );

            // Ask user for confirmation
            println!("Do you want to update? [y/N]");
            let mut input = String::new();
            std::io::stdin().read_line(&mut input)?;
            if input.trim().to_lowercase() == "y" {
                download_and_install(latest_version)?;
            } else {
                println!("Update canceled.");
            }
        } else {
            println!("You are already running the latest version: {}", CURRENT_VERSION);
        }
    } else {
        eprintln!("Failed to fetch latest version information.");
    }

    Ok(())
}

/// Downloads and installs the latest version
fn download_and_install(version: &str) -> Result<(), Box<dyn std::error::Error>> {
    let tarball_url = format!(
        "{}/archive/refs/tags/{}.tar.gz",
        REPO_URL, version
    );
    let tarball_path = format!("/tmp/sadl-{}.tar.gz", version);

    println!("Downloading new version from {}...", tarball_url);

    // Download the tarball
    let mut response = reqwest::blocking::get(&tarball_url)?;
    let mut file = fs::File::create(&tarball_path)?;
    std::io::copy(&mut response, &mut file)?;

    println!("Extracting and installing...");

    // Extract and install the new version
    Command::new("tar")
        .args(&["-xzf", &tarball_path, "-C", "/tmp"])
        .status()?;

    let extracted_path = format!("/tmp/sadl-{}", version);
    Command::new("cargo")
        .args(&["install", "--path", &extracted_path])
        .status()?;

    println!("Update to version {} complete!", version);

    // Clean up
    fs::remove_file(&tarball_path)?;

    Ok(())
}
