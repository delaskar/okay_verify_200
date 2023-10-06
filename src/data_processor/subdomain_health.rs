use reqwest::blocking;
use reqwest::Error as Err;
use reqwest::StatusCode;
use serde_json;
use std::collections::{HashMap, HashSet};

pub fn subdomains_status(
    subdomains_list: &Vec<String>,
) -> Result<HashMap<String, StatusCode>, Err> {
    // To track success subdomains.
    let mut subdomains_status: HashMap<String, StatusCode> = HashMap::new();
    // To track failed subdomains.
    let mut failed_subdomains: HashSet<String> = HashSet::new();

    for subdomain in subdomains_list {
        let subdomain_http = format!("https://{}", subdomain); //add "https" protocol
        println!("Verifying URL: {}", subdomain_http);
        match blocking::get(subdomain_http) {
            Ok(response) => {
                let status = response.status();
                subdomains_status.insert(subdomain.clone(), status);

                // If the status code is not successful (2xx), register the subdomain as failed
                if !status.is_success() {
                    failed_subdomains.insert(subdomain.clone());
                }
            }
            Err(err) => {
                eprintln!("Error verifying {}: {:?}", subdomain, err);
                // You can handle errors here, such as logging or tracking them.
                failed_subdomains.insert(subdomain.clone());
            }
        }
    }
    // Report failed subdomains.
    if !failed_subdomains.is_empty() {
        // Serialize errors in JSON format and display them.
        let error_info: Vec<(&String, &str)> = failed_subdomains
            .iter()
            .map(|subdomain| (subdomain, "Request error!"))
            .collect();
        let error_json =
            serde_json::to_string_pretty(&error_info).expect("Error serializing errors to JSON");
        eprintln!("Subdomains that failed:\n{}", error_json);
    }
    Ok(subdomains_status)
}
