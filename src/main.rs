mod data_processor;
pub use crate::data_processor::file_processing;
pub use crate::data_processor::subdomain_health;
use std::path::PathBuf;

fn main() {
    println!("Okay-Verify-200");

    let user_path: String =
        file_processing::input("[🍳] Please, enter the location of the subdomain file(s):");
    let path_file: PathBuf = PathBuf::from(&user_path);

    let file_domains = file_processing::process_file(&path_file).unwrap();
    let domain_status = subdomain_health::subdomains_status(&file_domains);

    println!("{:?}", domain_status);
}
