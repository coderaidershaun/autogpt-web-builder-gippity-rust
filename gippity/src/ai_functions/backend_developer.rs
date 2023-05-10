use proc_macro::function_to_string;


// Converts a request into a Factsheet
#[function_to_string]
pub fn develop_backend_website(_website_specification: &str) {
  /// INPUT: Takes in a specification for a website backend build
  /// FUNCTION: Writes all the rest api endpoints for a website backend that a frontend can access
  /// FUNCTION EXAMPLE: For example, a TODO app would have CRUD endpoints. A Crypto app would access and return basic crypto data
  /// DATABASE: If a DB_SCHEMA is provided, CRUD data should be saved to a file called "database.json" otherwise no data should be saved
  /// CONSTRAINTS:
  ///   1. CANNOT USE ANY PACKAGES OTHER THAN WHAT IS ALREADY INSTALLED HERE: actix-web, serde, serde_json, reqwest, tokio
  ///   2. ONLY WRITES THE RUST CODE. NO COMMENTARY. JUST CODE.
  ///   3. DO NOT provide cargo files or installation instructions. Only provide the code as you are constrained to the above cargo packages
  /// OUTPUT EXAMPLE:
  /// 
  /// // REPLACE WITH BACKEND API ENDPOINTS THAT FIT WEBSITE PURPOSE AND SPECIFICATION
  /// use actix_web::{web, App, HttpServer, Responder};
  /// async fn index() -> impl Responder {
  ///     "Hello world!"
  /// }
  /// #[actix_web::main]
  /// async fn main() -> std::io::Result<()> {
  ///     HttpServer::new(|| {
  ///         App::new().service(
  ///             // prefixes all resources and routes attached to it...
  ///             web::scope("/app")
  ///                 // ...so this handles requests for `GET /app/index.html`
  ///                 .route("/index.html", web::get().to(index)),
  ///         )
  ///     })
  ///     .bind(("127.0.0.1", 8080))?
  ///     .run()
  ///     .await
  /// }
  /// 
  println!(OUTPUT)
}



// Converts a request into a Factsheet
#[function_to_string]
pub fn improve_backend_code(_website_specification: &str) {
  /// INPUT: Takes in a specification for a website backend and compares it to code written by a Junior developer
  /// FUNCTION: Performs the following tasks:
  ///   1. Removes any bugs in the code
  ///   2. Makes sure everything requested in the spec from a backend standpoint was followed. If not, add the feature. No code should be implemented later. Everything should be written now.
  ///   3. ONLY writes the code. No commentary.
  ///   4. ONLY saves data to database.json, if a db_schema was provided with info. Otherwise code is corrected to return data rather than to save it.
  println!(OUTPUT)
}
