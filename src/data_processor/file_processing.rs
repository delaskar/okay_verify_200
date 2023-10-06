use regex::Regex;
use std::error::Error;
use std::fs::{self, File};
use std::io::{self, BufRead, BufReader};
use std::path::{Path, PathBuf};

pub fn process_file(file_path: &PathBuf) -> Result<Vec<String>, Box<dyn Error>> {
    // Load File
    let path: &Path = Path::new(file_path);

    // Subdomains List Vector
    let mut subdomains = Vec::new();

    // Verify type file
    if path.is_file() && path.extension() == Some("txt".as_ref()) {
        let file = File::open(path)?;

        // Read file
        let reader = BufReader::new(file);

        // Subdomain Search
        let subdomain_search = Regex::new(r"(?:[-*]\s*)?((?:[a-zA-Z0-9_-]+\.)+[a-zA-Z0-9.-]+)")
            .expect("Error in the regular expression");

        // Cleaning File
        for line in reader.lines() {
            let line = line?;
            if let Some(captures) = subdomain_search.captures(&line) {
                if let Some(subdomain) = captures.get(1) {
                    subdomains.push(subdomain.as_str().to_string());
                }
            }
        }
        Ok(subdomains)
    } else {
        Err("Not a valid File".into())
    }
}

pub fn process_directory(dir_path: &str) -> Result<Vec<String>, Box<dyn Error>> {
    // Load Directory
    let dir = Path::new(dir_path);

    // Verify Directory
    if dir.is_dir() {
        // Subdomain list vector
        let mut subdomains = Vec::new();

        // Access Directory
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();

            // Clear Directory Files
            if let Some(extension) = path.extension() {
                if extension == "txt" {
                    let file_subdomains = process_file(&path)?;
                    subdomains.extend(file_subdomains);
                }
            }
        }

        Ok(subdomains)
    } else {
        Err("Not a valid directory".into())
    }
}

pub fn input(prompt: &str) -> String {
    println!("{}", prompt);
    let mut input: String = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line!");
    input.trim().to_string()
}
