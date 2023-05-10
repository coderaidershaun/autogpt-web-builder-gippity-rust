use crate::helpers::general::{
  extend_ai_function, 
  check_status_code, 
  read_code_template_contents, 
  save_backend_code, 
  BACKEND_CODE_DIR
};
use crate::ai_functions::backend_developer::{print_backend_webserver_code, print_improved_webserver_code, print_fixed_code};
use crate::models::agent_basic::basic_agent::{BasicAgent, AgentState};
use crate::models::agent_basic::basic_traits::BasicTraits;
use crate::models::agents::agent_traits::{SpecialFunctions, FactSheet};
use crate::models::general::llm::Message;
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
  bug_errors: Option<String>,
  bug_count: u8
}

impl AgentBackendDeveloper {
  pub fn new() -> Self {

    // Define attributes
    let attributes: BasicAgent = BasicAgent {
      objective: "Devels backend code for webserver and json database".to_string(),
      position: "Backend Developer".to_string(),
      state: AgentState::Discovery,
      memory: vec![]
    };

    // Return Self
    Self {
      attributes,
      bug_errors: None,
      bug_count: 0
    }
  }
}


#[async_trait]
impl SpecialFunctions for AgentBackendDeveloper {

  fn get_attributes_from_agent(&self) -> &BasicAgent {
    &self.attributes
  }

  async fn execute(&mut self, factsheet: &mut FactSheet) -> Result<(), Box<dyn std::error::Error>> {

    // Extract Project Scope
    let (project_scope, project_description) = match &factsheet.project_scope {
      Some(project_scope) => {
        (project_scope, &factsheet.project_description)
      },
      None => panic!("Project Scope required before calling Agent")
    };

    // Continue until finished
    // !!! WARNING !!!
    while self.attributes.state != AgentState::Finished {

      // Execute logic based on Agent State
      match &self.attributes.state {

        // Write initial backend code
        AgentState::Discovery => {

          // Guard: Ensure required
          if !project_scope.is_crud_required && !project_scope.is_user_login_and_logout {
            self.attributes.state = AgentState::Finished;
            continue;
          }

          // Extract Code Template
          let code_template_str: String = read_code_template_contents();

          // Concatenate instruction
          let mut instruction: String = format!(
            "CODE_TEMPLATE: {} \n PROJECT_DESCRIPTION: {} \n",
            code_template_str, project_description);

          // Adjust Instruction - Ignore creating external links
          if project_scope.is_external_urls_required {
            instruction = format!("{} IMPORTANT IGNORE EXTERNAL DATA: Even though the PROJECT_DESCRIPTION will connect with external vendors for data,
            you do not need to write any code linking to external data APIS. This webserver purely deals with CRUD operations.", 
            instruction);
          }

          // Adjust Instruction - Ignore creating external links
          if !project_scope.is_user_login_and_logout {
            instruction = format!("{} IMPORTANT IGNORE USER REGISTRATION AND LOGIN: Even though the CODE_TEMPLATE shows how to manage User credentials,
            you can REMOVE this functionality from your code and just use the basic CRUD operations as shown.", 
            instruction);
          }

          // Adjust Instruction - Ignore creating external links
          if !project_scope.is_crud_required {
            instruction = format!("{} IMPORTANT IGNORE USER REGISTRATION AND LOGIN: Even though the CODE_TEMPLATE shows how to use CRUD,
            you can REMOVE this functionality from your code and just use the basic User Registration and Login CRUD operations as shown.", 
            instruction);
          }

          // Extract list tables required
          let func_message: Message = extend_ai_function(print_backend_webserver_code, instruction.as_str());

          // Call GPT - Confirm tables required
          println!("{} Agent: Writing first draft of backend code...", {self.attributes.get_position()});
          let backend_code: String = call_gpt(vec!(func_message)).await
            .expect("Failed to get response from LLM for writing backend code");

          // Update tables required
          save_backend_code(&backend_code);
          factsheet.backend_code = Some(backend_code);

          // Change state to working
          self.attributes.state = AgentState::Working;
          continue;
        }

        // Check and improve upon code
        AgentState::Working => {

          // Check and Enhance Code
          if self.bug_count < 2 {

            // Extract database ai function message
            let msg: String = format!("CODE_TEMPLATE: {:?}, PROJECT_DESCRIPTION: {:?}. 
              DO NOT WRITE CHAT. THIS FUNCTION ONLY OUTPUTS CODE. JUST OUTPUT THE CODE.", factsheet.backend_code, factsheet);
            let func_message: Message = extend_ai_function(print_improved_webserver_code, msg.as_str());
  
            // Call GPT - Code Improvements
            println!("{} Agent: Enhancing code...", {self.attributes.get_position()});
            let updated_backend_code: String = call_gpt(vec!(func_message)).await
              .expect("Failed to get response from LLM for code enhancements");
  
            // Update and continue
            save_backend_code(&updated_backend_code);
            factsheet.backend_code = Some(updated_backend_code);
            self.attributes.state = AgentState::UnitTesting;
            continue;

          // Correct for errors
          } else {

            // Extract database ai function message
            let msg: String = format!("BROKEN_CODE: {:?}, ERROR_BUGS: {:?}. 
              DO NOT WRITE CHAT. THIS FUNCTION ONLY OUTPUTS CODE. JUST OUTPUT THE CODE.", factsheet.backend_code, self.bug_errors);
            let func_message: Message = extend_ai_function(print_fixed_code, msg.as_str());

            // Call GPT - Fix bugs
            println!("{} Agent: Fixing bugs...", {self.attributes.get_position()});
            let updated_backend_code: String = call_gpt(vec!(func_message)).await
              .expect("Failed to get response from LLM for bug fixes");

            // Update and continue
            save_backend_code(&updated_backend_code);
            factsheet.backend_code = Some(updated_backend_code);
            self.attributes.state = AgentState::UnitTesting;
            continue;
          }
        },

        // Check Code Builds
        AgentState::UnitTesting => {

          // Build backend application
          println!("Backend Unit Testing: building...");
          let mut build_backend_server: std::process::Output = Command::new("cargo")
            .arg("build")
            .current_dir(BACKEND_CODE_DIR)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .output()
            .expect("Failed to run the backend application");

          // Determine if build errors
          if build_backend_server.status.success() {
            println!("Test server build successful...");
          } else {
            let error_arr: Vec<u8> = build_backend_server.stderr;
            let error_str: String = String::from_utf8(error_arr).unwrap();

            // Update error stats
            self.bug_count += 1;
            self.bug_errors = Some(error_str);

            // Pass back for rework
            self.attributes.state = AgentState::Working;
          }

          // Get URLS to test

          // Build backend application
          println!("Backend Unit Testing: Starting server...");
          let mut run_backend_server: std::process::Child = Command::new("cargo")
            .arg("run")
            .current_dir(BACKEND_CODE_DIR)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .expect("Failed to run the backend application");

          // Sleep for 5 seconds
          println!("Launching tests on server in 5 seconds...");
          let seconds_sleep: Duration = Duration::from_secs(5);
          time::sleep(seconds_sleep).await;

          // Create client with timeout
          let client: Client = Client::builder()
            .timeout(Duration::from_secs(5))
            .build()
            .unwrap();

          // Check status code
          println!("Testing endpoint...")
          match check_status_code(&client, "http://localhost:8080/forex").await {
            Ok(status_code) => {
              if status_code != 200 {
                panic!("Failed to call backend url");
              }
            }
            Err(e) => {
              // kill $(lsof -t -i:8080)
              run_backend_server.kill().expect("Failed to kill the backend web server");
              println!("Error checking backend: {}", e)
            },
          }

          // Kill backend server
          run_backend_server.kill().expect("Failed to kill the backend web server");

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
    let mut factsheet: FactSheet = serde_json::from_str("{\"project_description\":\"Build a full stack website with user login and logout that shows latest Forex prices\",\"project_scope\":{\"is_crud_required\":true,\"is_user_login_and_logout\":true,\"is_external_urls_required\":true},\"external_urls\":[\"https://api.exchangeratesapi.io/latest\"],\"backend_code\":null,\"frontend_code\":null,\"json_db_schema\":null}").unwrap();

    // Execute running agent
    agent.execute(&mut factsheet).await.expect("Unable to execute running agent");
    let contents: String = fs::read_to_string("/Users/shaun/Code/DEVELOPMENT/autogippity/website/backend/src/main.rs")
      .expect("Failed to read code");
    assert!(contents.len() > 100);
  }

  #[tokio::test]
  async fn tests_written_code() {

    // Create agent instance and site purpose
    let mut agent: AgentBackendDeveloper = AgentBackendDeveloper::new();
    agent.attributes.state = AgentState::UnitTesting;

    // Initialze Factsheet
    let mut factsheet: FactSheet = serde_json::from_str("{\"project_description\":\"Build a full stack website with user login and logout that shows latest Forex prices\",\"project_scope\":{\"is_crud_required\":true,\"is_user_login_and_logout\":true,\"is_external_urls_required\":true},\"external_urls\":[\"https://api.exchangeratesapi.io/latest\"],\"backend_code\":null,\"frontend_code\":null,\"json_db_schema\":null}").unwrap();

    // Execute running agent
    agent.execute(&mut factsheet).await.expect("Unable to execute running agent");
  }
}
