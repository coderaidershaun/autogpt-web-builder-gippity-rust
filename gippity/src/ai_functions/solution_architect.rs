use proc_macro::function_to_string;


// Converts a request into a Factsheet
#[function_to_string]
pub fn convert_request_to_json_facts(_user_request: &str) {
  /// Input: Takes in a user request to build a website.
  /// Function: Converts user request into JSON response of information items required for a website build.
  /// Output: Prints a JSON response in the following format:
  /// [
  ///   {
  ///     "user_login_required": bool,
  ///     "website_purpose": Option<String>, // (E-Commerce, SAAS, Chat App, ...)
  ///     "preferred_data_storage": Option<String>, // (Supabase, JSON, None)
  ///     "main_colors": Option<Vec<String>>, //  ("#4287f5", "#51db53", "#51c9db") -- must be in HEX format
  ///     "other_info_database": Option<String>, // Any other information that might be relevant to a required database
  ///     "other_info_backend": Option<String>, // Any other information that might be relevant for a backend rest apis
  ///     "other_info_frontend": Option<String> // Any other information that might be relevant for the user interface
  ///   }
  /// ]
  /// Example:
  ///   user_request = "I need a website that lets users login and logout. It needs to look fancy and accept payments."
  ///   returns [
  ///   {
  ///     "user_login_required": true,
  ///     "website_purpose": null,
  ///     "preferred_data_storage": null,
  ///     "main_colors": null,
  ///     "other_info_database": null,
  ///     "other_info_backend": null,
  ///     "other_info_frontend": "needs to look fancy"
  ///   }
  /// ]
  println!(OUTPUT)
}


// Takes user input and gives it a concise summary for what the user wants
#[function_to_string]
pub fn get_website_purpose(_user_request: &str) {
  /// Input: Takes in a user request related to the purpose of a website
  /// Function: Converts user request into a String response
  /// Output: Prints a JSON response in the following format:
  /// Example:
  ///   user_request = "It needs to be an Ecommerce store to sell hats."
  ///   returns "Provides Ecommerce Hat Sales"
  println!(OUTPUT)
}
