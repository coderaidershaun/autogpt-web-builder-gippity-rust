use proc_macro::function_to_string;


// Converts a request into a Factsheet
#[function_to_string]
pub fn list_database_tables(_website_purpose: &str) {
  /// Input: Takes in a specification for a website build
  /// Function: Outputs a list in JSON format of database table names required for the given website purpose
  /// Important: Lists a maximum of 5 table names. Fewer is better if possible
  /// Output: Prints a response in array format as follows:
  /// ["table_name_1", "table_name_2", ...]
  /// Example:
  ///   website_purpose = "website_purpose: Some("\"Provides TODO functionality.\"",)"
  ///   returns ["users_table", "todo_tracking_table", "reminders_table"]
  println!(OUTPUT)
}


// Converts a request into a Factsheet
#[function_to_string]
pub fn database_schema(_database_context_and_tables: &str) {
  /// Input: Takes in a list of database tables required for a project
  /// Function: Outputs a list in JSON format of a JSON schema for each database table provided by the user and context.
  /// Important: Lists a maximum of 5 table names. Fewer is better if possible
  /// Output: Prints a response in array format as follows:
  /// ["table_name_1", "table_name_2", ...]
  /// Example:
  ///   website_team_spec = "Original User request: Build me a website. Purpose: provides ecommerce functionality. Database tables list: [\"users_table\", \"products_table\", \"orders_table\", \"shopping_cart_table\", \"payment_details_table\"]"
  ///   returns: 
  ///   [
  ///     users_table: {
  ///       id: int,
  ///       name: string,
  ///       email: string,
  ///       password: string,
  ///       hashed_password: string
  ///     },
  ///     products_table: {
  ///       product_id: int,
  ///       product_name: string,
  ///       product_description: string
  ///     }
  ///     ... // continue for other tables here
  ///   ]
  println!(OUTPUT)
}
