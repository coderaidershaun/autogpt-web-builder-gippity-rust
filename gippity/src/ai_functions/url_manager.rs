use proc_macro::function_to_string;


// Converts a request into a Factsheet
#[function_to_string]
pub fn print_site_urls(_project_description: &str) {
  /// Input: Takes in a project description of a website build
  /// Function: Outputs a list of external public API endpoints that should be used in the building of the website
  /// Important: Only selects url endpoint(s) which do not require any API Keys at all
  /// Output: Prints a list response of external urls in the following format:
  /// ["url1", "url2", "url3", ...]
  /// Example:
  ///   website_team_spec = "website_purpose: Some("\"Provides Crypto Price Data from Binance and Kraken\"",)"
  ///   prints:
  /// ["https://api.binance.com/api/v3/exchangeInfo", "https://api.binance.com/api/v3/klines?symbol=BTCUSDT&interval=1d"]
  println!(OUTPUT)
}
