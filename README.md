# okay_verify_200 Project

**Okay-Verify-200** is a program designed to process data related to subdomains and generate reports on their health status. The program is currently in development, and there are plans for future updates.

### Description
Data Processor is a tool designed to streamline the processing of files containing information about subdomains. Its primary goal is to analyze this information and provide a detailed report on the health status of each subdomain.

### Usage
The program is initiated by the main() function, which performs the following operations:

Displays a welcome message in the console.

Prompts the user to enter the location of subdomain files.

Processes the specified files and conducts an analysis of the subdomains.

Generates a report in JSON format with the analysis results.

The report is saved in a file named "Subdomain_Response.txt" in the current working directory.

Dependencies
The project relies on the following dependencies:

  * regex = "1.9.6"
  * reqwest = { version = "0.11.22", features = ["blocking", "json"] }
  * serde = { version = "1.0.188", features = ["serde_derive", "derive"] }
  * serde_json = "1.0.107"

### Execution
You can run the program using Rust and Cargo. Make sure you have Rust installed on your system and then follow these steps:

- Clone the project's repository.

- Open a terminal and navigate to the project directory.

- Run the program using the following command: `cargo run`

### Project Status
This program is currently in development, and future updates are expected to introduce new features and enhancements. If you encounter issues or have suggestions, feel free to contribute to the project or report issues in the repository's issues section.

Thank you for your interest in **okay_verify_200**!
