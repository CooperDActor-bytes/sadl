use std::process::Command;
use std::fs::File;
use std::io::{self, BufRead};

fn download_reels(urls: Vec<String>) {
    for url in urls {
        println!("Downloading: {}", url);
        let status = Command::new("yt-dlp")
            .arg(url.clone())  // Clone the url here to avoid moving it
            .status()
            .expect("Failed to start yt-dlp");
        
        if !status.success() {
            eprintln!("Error downloading {}", url); // Now we can use url without any issue
        }
    }
}

fn read_urls(file_path: &str) -> io::Result<Vec<String>> {
    let file = File::open(file_path)?;
    let reader = io::BufReader::new(file);
    let mut urls = Vec::new();

    for line in reader.lines() {
        urls.push(line?);
    }

    Ok(urls)
}

fn main() {
    let urls = read_urls("insta.txt").expect("Failed to read URLs from file");
    download_reels(urls);
}

