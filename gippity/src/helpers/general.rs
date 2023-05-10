use crate::models::general::llm::Message;
use reqwest::Client;


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