use crate::models::agent_basic::basic_agent::{BasicAgent, AgentState};
use crate::models::agent_basic::basic_traits::BasicTraits;
use crate::models::agents::agent_traits::{SpecialFunctions, FactSheet};
use crate::models::general::llm::Message;
use crate::ai_functions::backend_developer::{develop_backend_website, improve_backend_code};
use crate::helpers::general::{extend_ai_function, check_status_code};
use crate::apis::call_request::call_gpt;
use async_trait::async_trait;

use std::fs;
use std::process::{Command, Stdio};
use std::time::Duration;
use tokio::time;
use reqwest::Client;

// Solution Architect
#[derive(Debug)]
pub struct AgentBackendDeveloper {
  attributes: BasicAgent,
  backend_code: Option<String>,
}

impl AgentBackendDeveloper {
  pub fn new() -> Self {

    // Define attributes
    let attributes: BasicAgent = BasicAgent {
      objective: "structure the database format for website build".to_string(),
      position: "Database Architect".to_string(),
      state: AgentState::Discovery,
      memory: vec![]
    };

    // Return Self
    Self {
      attributes,
      backend_code: None,
    }
  }
}


#[async_trait]
impl SpecialFunctions for AgentBackendDeveloper {

  fn get_attributes_from_agent(&self) -> &BasicAgent {
    &self.attributes
  }

  async fn execute(&mut self, factsheet: &mut FactSheet) -> Result<(), Box<dyn std::error::Error>> {

    // Extract initial spec
    let initial_spec: &Option<String> = match &factsheet.initial_spec {
      Some(initial_spec) => &initial_spec.website_purpose,
      None => panic!("No initial spec found")
    };

    // Extract site purpose
    let site_purpose: &String = match initial_spec {
      Some(site_purp) => site_purp,
      None => &factsheet.project_goal
    };

    // Continue until finished
    // !!! WARNING !!!
    while self.attributes.state != AgentState::Finished {

      // Execute logic based on Agent State
      match &self.attributes.state {

        // Write initial backend code
        AgentState::Discovery => {

          // Guard: Ensure DB is required
          let input_str: String = format!("{:?}", factsheet);

          // Extract list tables required
          let func_message: Message = extend_ai_function(develop_backend_website, input_str.clone().as_str());

          // Call GPT - Confirm tables required
          println!("{} Agent: Writing first draft of backend code...", {self.attributes.get_position()});
          let backend_code: String = call_gpt(vec!(func_message)).await
            .expect("Failed to get response from LLM for writing backend code");

          // Update tables required
          self.backend_code = Some(backend_code);

          // Change state to working
          self.attributes.state = AgentState::Working;
          continue;
        }

        // Check and improve upon code
        AgentState::Working => {

          // Extract database ai function message
          let msg: String = format!("Junior Developer Code: {:?}, Original Spec: {:?}. 
            DO NOT WRITE CHAT. THIS FUNCTION ONLY OUTPUTS CODE. JUST OUTPUT THE CODE.", self.backend_code, factsheet);
          let func_message: Message = extend_ai_function(improve_backend_code, msg.as_str());

          // Call GPT - Database Schema
          println!("{} Agent: Senior developer making code adjustments...", {self.attributes.get_position()});
          let updated_backend_code: String = call_gpt(vec!(func_message)).await
            .expect("Failed to get response from LLM database JSON schema");

          // Save Rust Code
          fs::write("/Users/shaun/Code/DEVELOPMENT/autogippity/website/backend/src/main.rs", &updated_backend_code)
            .expect("Failed to save file in requested location. Check your file path.");

          // Store code in memory and move to unit testing
          self.backend_code = Some(updated_backend_code);
          self.attributes.state = AgentState::UnitTesting;
          continue;
        },

        // Check and improve upon code
        AgentState::UnitTesting => {

          // Extract url endpoints from backend code for calling/testing

          // Build backend application
          println!("Backend Unit Testing: building...");
          let mut backend_server: std::process::Child = Command::new("cargo")
            .arg("run")
            .current_dir("/Users/shaun/Code/DEVELOPMENT/autogippity/website/backend")
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .spawn() // or .status()
            .expect("Failed to run the backend application");

          // Sleep for 5 seconds
          println!("Calling endpoints...");
          let seconds_sleep: Duration = Duration::from_secs(10);
          time::sleep(seconds_sleep).await;

          // Create client with timeout
          let client: Client = Client::builder()
            .timeout(Duration::from_secs(5))
            .build()
            .unwrap();

          // Check status code
          match check_status_code(&client, "http://localhost:8080/klines").await {
            Ok(status_code) => {
              if status_code != 200 {
                panic!("Failed to call backend url");
              }
            }
            Err(e) => {
              // kill $(lsof -t -i:8080)
              backend_server.kill().expect("Failed to kill the backend web server");
              println!("Error checking backend: {}", e)
            },
          }

          // Kill backend server
          backend_server.kill().expect("Failed to kill the backend web server");

          // Update agent state to finished
          self.attributes.state = AgentState::Finished;
        }

        // Ensure all cases are covered
        _ => {}
      }
    }
    Ok(())
  }
}



#[cfg(test)]
pub mod tests {
  use super::*;

  #[test]
  fn creates_new_agen_db_architect() {
    let agent: AgentBackendDeveloper = AgentBackendDeveloper::new();
    assert_eq!(agent.attributes.position, "Backend Developer");
  }

  #[tokio::test]
  async fn develops_and_saves_website_backend() {

    // Create agent instance and site purpose
    let mut agent: AgentBackendDeveloper = AgentBackendDeveloper::new();

    // Initialze Factsheet
    let mut factsheet: FactSheet = serde_json::from_str("{\"project_goal\":\"build a website that fetches Binance crypto prices and provides a full-stack solution\",\"initial_spec\":{\"user_login_required\":false,\"website_purpose\":\"fetches Binance crypto prices\",\"preferred_data_storage\":\"JSON\",\"main_colors\":null,\"other_info_database\":null,\"other_info_backend\":\"full-stack solution\",\"other_info_frontend\":null},\"urls\":[{\"rest_api_endpoint\":\"https://api.binance.com/api/v3/exchangeInfo\",\"rest_api_purpose\":\"Returns crypto currency symbols with data relates to that symbol\"},{\"rest_api_endpoint\":\"https://api.binance.com/api/v3/klines?symbol=BTCUSDT&interval=1d\",\"rest_api_purpose\":\"Returns crypto currency symbols with data relates to that symbol\"}],\"db_schema\":\"[\\n  \\\"users_table: {\\n    id: int,\\n    name: string,\\n    email: string,\\n    password: string,\\n    hashed_password: string\\n  },\\n  crypto_prices_table: {\\n    product_id: int,\\n    product_name: string,\\n    product_description: string\\n  },\\n  binance_api_table: {\\n    api_key: string,\\n    secret_key: string\\n  },\\n  price_history_table: {\\n    id: int,\\n    date_created: timestamp,\\n    crypto_price: string,\\n    user_id: int\\n  },\\n  refresh_token_table: {\\n    id: int,\\n    refresh_token: string,\\n    user_id: int\\n  }\\n]\",\"cargo_imports\":null,\"yarn_imports\":null,\"backend_rest_api_urls\":null}").unwrap();

    // Execute running agent
    agent.execute(&mut factsheet).await.expect("Unable to execute running agent");
    let contents: String = fs::read_to_string("/Users/shaun/Code/DEVELOPMENT/autogippity/website/backend/src/main.rs")
      .expect("Failed to read code");
    assert!(contents.len() > 100);
  }

  #[tokio::test]
  async fn tests_code_written_code() {

    // Create agent instance and site purpose
    let mut agent: AgentBackendDeveloper = AgentBackendDeveloper::new();
    agent.attributes.state = AgentState::UnitTesting;

    // Initialze Factsheet
    let mut factsheet: FactSheet = serde_json::from_str("{\"project_goal\":\"build a website that fetches Binance crypto prices and provides a full-stack solution\",\"initial_spec\":{\"user_login_required\":false,\"website_purpose\":\"fetches Binance crypto prices\",\"preferred_data_storage\":\"JSON\",\"main_colors\":null,\"other_info_database\":null,\"other_info_backend\":\"full-stack solution\",\"other_info_frontend\":null},\"urls\":[{\"rest_api_endpoint\":\"https://api.binance.com/api/v3/exchangeInfo\",\"rest_api_purpose\":\"Returns crypto currency symbols with data relates to that symbol\"},{\"rest_api_endpoint\":\"https://api.binance.com/api/v3/klines?symbol=BTCUSDT&interval=1d\",\"rest_api_purpose\":\"Returns crypto currency symbols with data relates to that symbol\"}],\"db_schema\":\"[\\n  \\\"users_table: {\\n    id: int,\\n    name: string,\\n    email: string,\\n    password: string,\\n    hashed_password: string\\n  },\\n  crypto_prices_table: {\\n    product_id: int,\\n    product_name: string,\\n    product_description: string\\n  },\\n  binance_api_table: {\\n    api_key: string,\\n    secret_key: string\\n  },\\n  price_history_table: {\\n    id: int,\\n    date_created: timestamp,\\n    crypto_price: string,\\n    user_id: int\\n  },\\n  refresh_token_table: {\\n    id: int,\\n    refresh_token: string,\\n    user_id: int\\n  }\\n]\",\"cargo_imports\":null,\"yarn_imports\":null,\"backend_rest_api_urls\":null}").unwrap();

    // Execute running agent
    agent.execute(&mut factsheet).await.expect("Unable to execute running agent");
  }
}
