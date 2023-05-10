use proc_macro::function_to_string;


// Converts a request into a Factsheet
#[function_to_string]
pub fn print_backend_webserver_code(_website_specification: &str) {
  /// INPUT: Takes in a specification for a website backend build
  /// FUNCTION: Writes all the rest api endpoints for a website backend that a frontend can access
  /// 
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
