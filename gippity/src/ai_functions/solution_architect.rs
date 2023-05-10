use proc_macro::function_to_string;


#[function_to_string]
pub fn print_project_scope(_project_description: &str) {
  /// Input: Takes in a user request to build a website project description
  /// Function: Converts user request into JSON response of information items required for a website build.
  /// Important: At least one of the bool results must be true
  /// Output: Prints an object response in the following format:
  ///   {
  ///     "is_crud_required": bool, // true if site needs CRUD functionality
  ///     "is_user_login_and_logout": bool // true if site needs users to be able to log in and log out
  ///     "is_external_urls_required": bool // true if site needs to fetch data from third part providers
  ///   }
  /// Example 1:
  ///   user_request = "I need a full stack website that accepts users and gets stock price data"
  ///   prints:
  ///   {
  ///     "is_crud_required": true
  ///     "is_user_login_and_logout": true
  ///     "is_external_urls_required": bool true
  ///   }
  /// Example 2:
  ///   user_request = "I need a simple TODO app"
  ///   prints:
  ///   {
  ///     "is_crud_required": true
  ///     "is_user_login_and_logout": false
  ///     "is_external_urls_required": bool false
  ///   }
  println!(OUTPUT)
}
