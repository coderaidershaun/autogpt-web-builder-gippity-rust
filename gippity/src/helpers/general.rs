use serde::de::DeserializeOwned;
use crate::models::general::llm::Message;
use crate::apis::call_request::call_gpt;
use crate::helpers::command_line::print_agent_message;
use reqwest::Client;

use std::fs;


// Constant Directories
pub const BACKEND_CODE_DIR: &str = "/Users/shaun/Code/DEVELOPMENT/autogippity/website/backend";
pub const FRONTEND_CODE_DIR: &str = "/Users/shaun/Code/DEVELOPMENT/autogippity/website/frontend";


// Provide AI call response
pub enum AIFuncResponse<T> {
  Decoded(T),
  Raw(String),
}

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


// Performs call to backend GPT
pub async fn ai_task_request_decoded<T: DeserializeOwned>(
  msg_context: String,
  agent_position: &str,
  agent_operation: &str,
  function_pass: for<'a> fn(&'a str) -> &'static str,
) -> T {

  // Call GPT - Structure AI function
  let func_message: Message = extend_ai_function(function_pass, &msg_context);

  // Get name of AI function being called
  let ai_function_name: &str = get_function_string!(function_pass);

  // Print agent statement
  print_agent_message(agent_position, agent_operation);
  let agent_response: String = call_gpt(vec!(func_message.clone())).await
    .expect("Failed to get response from procider");
  
  // Decode and return message
  let decoded_response: T = serde_json::from_str(agent_response.as_str())
    .expect("Failed to decode ai response from serde_json");
  return decoded_response;
}


// Check reqwest status code
pub async fn check_status_code(client: &Client, url: &str) -> Result<u16, reqwest::Error> {
  let response: reqwest::Response = client.get(url).send().await?;
  Ok(response.status().as_u16())
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

// Save frontend code
pub fn save_frontend_code(frontend_path: &String, contents: &String) {
  let path: String = format!("{}{}", FRONTEND_CODE_DIR, frontend_path);
  fs::write(path, contents)
    .expect("Something went wrong saving the file");
}

// Get existing frontend code
pub fn read_frontend_code_contents(frontend_path: &String) -> String {
  let path: String = format!("{}{}", FRONTEND_CODE_DIR, frontend_path);
  fs::read_to_string(path).expect("Something went wrong reading the file")
}
