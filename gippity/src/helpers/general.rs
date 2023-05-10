use crate::models::general::llm::Message;
use reqwest::Client;

use std::fs;

pub const BACKEND_CODE_DIR: &str = "/Users/shaun/Code/DEVELOPMENT/autogippity/website/backend";

/// Takes in both the string version of an AI function
/// Combines this with the user input to encourage a structured printout in a program-like response
pub fn extend_ai_function(ai_func: fn(&str) -> &'static str, func_input: &str) -> Message {

  // Extract AI function text
  let ai_function_str: &str = ai_func(func_input);

  // Combine and AI function as string with Instruction
  let msg: String = format!("FUNCTION: {} 
    INSTRUCTION: You are a function printer. You only print the results of functions. Nothing else. No commentary.  
    Here is the input to the function: '{}'. Print out what the function will return.", 
    ai_function_str, func_input);

  // Return result in Message format
  Message {
    role: "system".to_string(),
    content: msg
  }
}


// Check reqwest status code
pub async fn check_status_code(client: &Client, url: &str) -> Result<u16, reqwest::Error> {
  let response: reqwest::Response = client.get(url).send().await?;
  Ok(response.status().as_u16())
}


// Get backend working directory
fn backend_working_directory() {
  let directory: &str = "/Users/shaun/Code/DEVELOPMENT/autogippity/website/backend";

}


// Get code template
pub fn read_code_template_contents() -> String {
  let path: String = format!("{}/src/codetemplate.rs", BACKEND_CODE_DIR);
  fs::read_to_string(path).expect("Something went wrong reading the file")
}

// Save backend code
pub fn save_backend_code(contents: &String) {
  let path: String = format!("{}/src/main.rs", BACKEND_CODE_DIR);
  fs::write(path, contents)
    .expect("Something went wrong saving the file");
}

// Save json api endpoint schema
pub fn save_api_endpoints(api_endpoints: &String) {
  let path: String = format!("{}/api_endpoints.json", BACKEND_CODE_DIR);
  fs::write(path, api_endpoints)
    .expect("Something went wrong saving the file");
}
