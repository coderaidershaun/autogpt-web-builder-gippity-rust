use serde::Deserialize;
use proc_macro::function_to_string;


// Converts a request into a Factsheet
#[function_to_string]
pub fn provide_site_urls(_website_purpose: &str) {
  /// Input: Takes in a specification for a website
  /// Function: Outputs a list of backend REST API endpoints that should be used in the building of the website
  /// Important: Only selects url endpoints which do not require any API Keys
  /// Output: Prints a JSON response in the following format:
  /// [
  ///   {
  ///     "rest_api_endpoint": String,
  ///     "rest_api_purpose": String,
  ///   },
  ///   ...
  /// ]
  /// Example:
  ///   website_team_spec = "website_purpose: Some("\"Provides Crypto Price Data from Binance and Kraken\"",)"
  ///   returns [
  ///   {
  ///     "rest_api_endpoint": https://api.binance.com/api/v3/exchangeInfo,
  ///     "rest_api_purpose": "Returns crypto currency symbols with data relates to that symbol",
  ///   },
  ///   {
  ///     "rest_api_endpoint": https://api.binance.com/api/v3/klines?symbol=BTCUSDT&interval=1d,
  ///     "rest_api_purpose": "Returns crypto currency symbols with data relates to that symbol",
  ///   },
  ///   ...
  /// ]
  println!(OUTPUT)
}



// Converts a request into a Factsheet
#[function_to_string]
pub fn is_site_urls_required(_website_purpose: &str) {
  /// Input: Takes in a specification for a website
  /// Function: Outputs a boolean true or false as to whether the website will need to get data from third party data sources
  /// Important: Only selects url endpoints which do not require any API Keys
  /// Output: Prints a string response in the following format: false
  /// VERY IMPORTANT: There should ONLY be a boolean response. Nothing else.
  /// Example:
  ///   website_purpose = "website_purpose: Some("\"Provides Crypto Price Data from Binance and Kraken\"",)
  ///   returns true
  println!(OUTPUT)
}
