mod data_processor;
pub use crate::data_processor::file_processing;
pub use crate::data_processor::subdomain_health;
use std::path::PathBuf;
use serde_json;
use std::fs::File;
use std::io::Write;


fn main() {
    println!("Okay-Verify-200");

    let user_path: String =
        file_processing::input("[🍳] Please, enter the location of the subdomain file(s):");
    let path_file: PathBuf = PathBuf::from(&user_path);

    let file_domains = file_processing::process_file(&path_file).unwrap();
    let analyze = subdomain_health::SubdomainResult::subdomain_analyze(file_domains);
    
    match analyze {
      Ok(success) => {
        let json_string = serde_json::to_string_pretty(&success).expect("Serilize Failed!");
        let mut new_file = File::create("Subdomain_Response.txt").expect("Filed to create file.");
        new_file.write_all(json_string.as_bytes()).expect("Failed to write data.");
      },
      Err(error) => println!("{:?}", error),
    };
}
