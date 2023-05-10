use proc_macro::function_to_string;


#[function_to_string]
pub fn print_backend_webserver_code(_project_description_and_template: &str) {
  /// INPUT: Takes in a PROJECT_DESCRIPTION and CODE_TEMPLATE for a website backend build
  /// FUNCTION: Takes an existing set of code marked as CODE_TEMPLATE and updates or re-writes it to work for the purpose in the PROJECT_DESCRIPTION
  /// IMPORTANT: The following libraries are already installed
  ///   reqwest, serde, serde_json, tokio, actix-web, async-trait
  /// Therefore, this function can only work with code from the standard Rust library or the above as per shown in the CODE_TEMPLATE
  /// OUTPUT: Print ONLY the code, nothing else. This function ONLY prints code.
  println!(OUTPUT)
}


#[function_to_string]
pub fn print_improved_webserver_code(_project_description_and_template: &str) {
  /// INPUT: Takes in a PROJECT_DESCRIPTION and CODE_TEMPLATE for a website backend build
  /// FUNCTION: Performs the following tasks:
  ///   1. Removes any bugs in the code and adds minor additional functionality
  ///   2. Makes sure everything requested in the spec from a backend standpoint was followed. If not, add the feature. No code should be implemented later. Everything should be written now.
  ///   3. ONLY writes the code. No commentary.
  /// IMPORTANT: The following libraries are already installed. Does not use ANY libraries other than what was provided in the template
  ///   reqwest, serde, serde_json, tokio, actix-web, async-trait
  println!(OUTPUT)
}


#[function_to_string]
pub fn print_fixed_code(_broken_code_with_bugs: &str) {
  /// INPUT: Takes in Rust BROKEN_CODE and the ERROR_BUGS found
  /// FUNCTION: Removes bugs from code
  /// IMPORTANT: Only prints out the new and improved code. No commentary or anything else
  println!(OUTPUT)
}


#[function_to_string]
pub fn get_endpoints(_code_input: &str) {
  /// INPUT: Takes in Rust webserver code based on actix web
  /// FUNCTION: Prints out the JSON schema for url endpoints and their respective types
  /// IMPORTANT: Only prints out the JSON schema. No commentary or anything else
  /// EXAMPLE:
  /// INPUT_CODE:
  /// ...
  /// pub struct Task {
  ///   pub id: u64,
  ///   pub name: String,
  ///   pub completed: bool,
  /// }
  /// pub struct User {
  ///   pub id: u64,
  ///   pub username: String,
  ///   pub password: String,
  /// }
  /// ...
  /// HttpServer::new(move || {
  ///   App::new()
  ///       .app_data(data.clone())
  ///       .route("/item", web::post().to(create_task))
  ///       .route("/item", web::put().to(create_task))
  ///       .route("/signup", web::post().to(register))
  ///       .route("/login", web::post().to(login))
  ///       .route("/forex", web::get().to(forex)) 
  println!(OUTPUT)
}
