use reqwest::blocking;
use reqwest::Error;
use std::collections::HashMap;


#[derive(Debug)]
pub struct SubdomainResult {
    ip_origin: String,
    status_code: String,
    header: String,
    fail: String,
}

impl SubdomainResult {

    fn build_subdomain_result(ip_origin: String, status_code: String, header: String, fail: String) -> SubdomainResult {
        Self { ip_origin , status_code , header, fail}
    }
    
    pub async fn subdomain_analyze(subdomain_list: Vec<String>) -> Result<HashMap<String, SubdomainResult>, Error> {

        // Create result analyze 
        let mut analyze_result = HashMap::new();
    
        // Iterate over every subdomain file
        for subdomain in subdomain_list.iter() {
            
            // Setting url
            let url: String = format!("http://{}", &subdomain);
            
            // Get subdomain response
            let _response = match blocking::get(url)  {
                Ok(res_success) => {

                    let subdomain_result = Self::build_subdomain_result(
                        res_success.remote_addr().unwrap().to_string(),
                        res_success.status().to_string(),
                        format!("{:?}", res_success.headers()),
                        String::from("None"),
                    );
                    // Print succes on Terminal
                    let display_console = format!(
                        r#"
                        IP-Origin: {}
                        Status Code: {}
                        Header: {}
                        fail: {}
                        "#,
                        subdomain_result.ip_origin,
                        subdomain_result.status_code,
                        subdomain_result.header,
                        subdomain_result.fail
                    );
                    // Store data struct
                    analyze_result.insert(subdomain.clone(), subdomain_result);
                    println!("Success:\n{}", display_console);
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
                        fail: {}
                        "#,
                        subdmain_error_result.ip_origin,
                        subdmain_error_result.status_code,
                        subdmain_error_result.header,
                        subdmain_error_result.fail
                    );
                    // Store data struct
                    analyze_result.insert(subdomain.clone(), subdmain_error_result);
                    println!("Failed:\n{}", display_console);
                }
            };
        }
        Ok(analyze_result)
    }
        
}
