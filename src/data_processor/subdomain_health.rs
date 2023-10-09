use reqwest::blocking;
use reqwest::Error;
use std::collections::HashMap;
use serde::Serialize;


#[derive(Serialize, Debug)]
pub struct SubdomainResult {
    ip_origin: String,
    status_code: String,
    header: String,
    operation: String,
}

impl SubdomainResult {

    fn build_subdomain_result(ip_origin: String, status_code: String, header: String, operation: String) -> SubdomainResult {
        Self { ip_origin , status_code , header, operation}
    }
    
    pub fn subdomain_analyze(subdomain_list: Vec<String>) -> Result<HashMap<String, SubdomainResult>, Error> {

        // Create result analyze 
        let mut analyze_result = HashMap::new();
    
        // Iterate over every subdomain file
        for subdomain in subdomain_list.iter() {
            
            // Setting url
            let url: String = format!("http://{}", &subdomain);

            // Get subdomain response
            match blocking::get(url)  {
                Ok(res_success) => {

                    let subdomain_result = Self::build_subdomain_result(
                        res_success.remote_addr().unwrap().to_string(),
                        res_success.status().to_string(),
                        format!("{:?}", res_success.headers()),
                        String::from("Request Success!"),
                    );

                    // Print succes on Terminal
                    let display_console = format!(
                        r#"
                        IP-Origin: {}
                        Status Code: {}
                        Header: {}
                        operation: {}
                        "#,
                        subdomain_result.ip_origin,
                        subdomain_result.status_code,
                        subdomain_result.header,
                        subdomain_result.operation
                    );
                    // Store data struct
                    analyze_result.insert(subdomain.clone(), subdomain_result);
                    println!("{}:\n{}", &subdomain, display_console);
                }
                Err(error_url) => {
                    let error_info = format!("{}", error_url);
                    let subdmain_error_result = Self::build_subdomain_result(
                        String::from("Subdomain Error!"),
                        String::from("Subdomain Error!"),
                        String::from("Subdomain Error!"),
                        error_info,
                    );

                    // Print Error on Terminal
                    let display_console = format!(
                        r#"
                        IP-Origin: {}
                        Status Code: {}
                        Header: {}
                        operation: {}
                        "#,
                        subdmain_error_result.ip_origin,
                        subdmain_error_result.status_code,
                        subdmain_error_result.header,
                        subdmain_error_result.operation
                    );
                    // Store data struct
                    analyze_result.insert(subdomain.clone(), subdmain_error_result);
                    println!("{}:\n{}", &subdomain, display_console);
                }
            };
        }
        Ok(analyze_result)
    }
        
}
