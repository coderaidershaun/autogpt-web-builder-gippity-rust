use proc_macro::function_to_string;


// Converts a request into a Factsheet
#[function_to_string]
pub fn print_backend_webserver_code(_project_description_and_template: &str) {
  /// INPUT: Takes in a PROJECT_DESCRIPTION and CODE TEMPLATE for a website backend build
  /// FUNCTION: Takes an existing set of code marked as CODE_TEMPLATE and updates or re-writes it to work for the purpose in the PROJECT_DESCRIPTION
  /// IMPORTANT: The following libraries are already installed
  ///   reqwest, serde, serde_json, tokio, actix-web, async-trait
  /// Therefore, this function can only work with code from the standard Rust library or the above as per shown in the CODE_TEMPLATE
  /// OUTPUT: Print ONLY the code, nothing else. This function ONLY prints code.
  println!(OUTPUT)
}


// Converts a request into a Factsheet
#[function_to_string]
pub fn print_improved_webserver_code(_website_specification: &str) {
  /// INPUT: Takes in a specification for a website backend and compares it to code written by a Junior developer
  /// FUNCTION: Performs the following tasks:
  ///   1. Removes any bugs in the code
  ///   2. Makes sure everything requested in the spec from a backend standpoint was followed. If not, add the feature. No code should be implemented later. Everything should be written now.
  ///   3. ONLY writes the code. No commentary.
  ///   4. ONLY saves data to database.json, if a db_schema was provided with info. Otherwise code is corrected to return data rather than to save it.
  println!(OUTPUT)
}
