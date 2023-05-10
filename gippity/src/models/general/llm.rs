use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Clone)]
pub struct Message {
  pub role: String,
  pub content: String,
}


#[derive(Debug, Serialize)]
pub struct ChatCompletion {
  pub model: String,
  pub messages: Vec<Message>,
}


#[derive(Debug, Deserialize)]
pub struct ApiMessage {
  pub content: String,
}


#[derive(Debug, Deserialize)]
pub struct APIResponse {
  pub choices: Vec<ApiChoice>,
}


#[derive(Debug, Deserialize)]
pub struct ApiChoice {
  pub message: ApiMessage,
}