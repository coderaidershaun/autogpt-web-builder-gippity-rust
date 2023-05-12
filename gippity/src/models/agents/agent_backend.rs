use crate::ai_functions::aifunc_backend::{
  print_backend_webserver_code, 
  print_improved_webserver_code, 
  print_fixed_code,
  print_rest_api_endpoints
};
use crate::helpers::general::{
  check_status_code, 
  read_code_template_contents, 
  save_backend_code,
  save_api_endpoints,
  BACKEND_CODE_DIR
};
use crate::helpers::command_line::confirm_safe_code;
use crate::models::agent_basic::basic_agent::{BasicAgent, AgentState};
use crate::helpers::command_line::PrintCommand;
use crate::models::agents::agent_traits::{SpecialFunctions, FactSheet, RouteObject};
use crate::helpers::general::ai_task_request;
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
      objective: "Develops backend code for webserver and json database".to_string(),
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

  // AI Call: Write initial backend webserver code
  async fn call_initial_backend_code(&mut self, factsheet: &mut FactSheet) {

    // Extract Code Template
    let code_template_str: String = read_code_template_contents();

    // Concatenate instruction
    let mut msg_context: String = format!(
      "CODE_TEMPLATE: {} \n PROJECT_DESCRIPTION: {} \n",
      code_template_str, factsheet.project_description);

    // Adjust Instruction - Ignore creating external links
    if factsheet.project_scope.unwrap().is_external_urls_required {
      msg_context = format!("{} IMPORTANT IGNORE EXTERNAL DATA: Even though the PROJECT_DESCRIPTION will connect with external vendors for data,
      you do not need to write any code linking to external data APIS. This webserver purely deals with CRUD operations.", 
      msg_context);
    }

    // Adjust Instruction - Ignore creating external links
    if !factsheet.project_scope.unwrap().is_user_login_and_logout {
      msg_context = format!("{} IMPORTANT IGNORE USER REGISTRATION AND LOGIN: Even though the CODE_TEMPLATE shows how to manage User credentials,
      you can REMOVE this functionality from your code and just use the basic CRUD operations as shown.", 
      msg_context);
    }

    // Adjust Instruction - Ignore creating external links
    if !factsheet.project_scope.unwrap().is_crud_required {
      msg_context = format!("{} IMPORTANT IGNORE USER REGISTRATION AND LOGIN: Even though the CODE_TEMPLATE shows how to use CRUD,
      you can REMOVE this functionality from your code and just use the basic User Registration and Login CRUD operations as shown.", 
      msg_context);
    }

    // Retrieve AI Reponse
    let ai_response: String = ai_task_request(
      msg_context, 
      &self.attributes.position, 
      get_function_string!(print_backend_webserver_code), 
      print_backend_webserver_code).await;
    
    // Save code and update state
    save_backend_code(&ai_response);
    factsheet.backend_code = Some(ai_response);
  }


  // AI Call: Write improved backend webserver code
  async fn call_improved_backend_code(&mut self, factsheet: &mut FactSheet) {

    // Structure message context
    let msg_context: String = format!("CODE_TEMPLATE: {:?}, PROJECT_DESCRIPTION: {:?}. 
      THIS FUNCTION ONLY OUTPUTS CODE. JUST OUTPUT THE CODE.", factsheet.backend_code, factsheet);

    // Retrieve AI Reponse
    let ai_response: String = ai_task_request(
      msg_context, 
      &self.attributes.position, 
      get_function_string!(print_improved_webserver_code), 
      print_improved_webserver_code).await;

    // Update and continue
    save_backend_code(&ai_response);
    factsheet.backend_code = Some(ai_response);
  }


  // AI Call: Fix bugs in code
  async fn call_fix_code_bugs(&mut self, factsheet: &mut FactSheet) {

    // Structure message context
    let msg_context: String = format!("BROKEN_CODE: {:?}, ERROR_BUGS: {:?}. 
      THIS FUNCTION ONLY OUTPUTS CODE. JUST OUTPUT THE CODE.", factsheet.backend_code, self.bug_errors);

    // Retrieve AI Reponse
    let ai_response: String = ai_task_request(
      msg_context, 
      &self.attributes.position, 
      get_function_string!(print_fixed_code), 
      print_fixed_code).await;

    // Update and continue
    save_backend_code(&ai_response);
    factsheet.backend_code = Some(ai_response);
  }


  // AI Call: Extract REST API Endpoints
  async fn call_extract_rest_api_endpoints(&self) -> String {

    // Get latest backend code from file (so can run separately when running cargo test)
    let path: String = format!("{}/src/main.rs", BACKEND_CODE_DIR);
    let backend_code: String = fs::read_to_string(path).expect("Something went wrong reading the file");

    // Structure message context
    let msg_context: String = format!("CODE_INPUT: {:?}", backend_code);

    // Retrieve AI Reponse
    let ai_response: String = ai_task_request(
      msg_context, 
      &self.attributes.position, 
      get_function_string!(print_rest_api_endpoints), 
      print_rest_api_endpoints).await;

    // Return response
    return ai_response;
  }
}


#[async_trait]
impl SpecialFunctions for AgentBackendDeveloper {

  fn get_attributes_from_agent(&self) -> &BasicAgent {
    &self.attributes
  }

  async fn execute(&mut self, factsheet: &mut FactSheet) -> Result<(), Box<dyn std::error::Error>> {

    // Get project scope items
    let (is_crud_required, is_user_login_and_logout): (bool, bool) = match &factsheet.project_scope {
      Some(scope) => (scope.is_crud_required, scope.is_user_login_and_logout),
      None => return Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, "Must contain project scope before starting on Backend work"))),
    };

    // Continue until finished
    // !!! WARNING !!!
    while self.attributes.state != AgentState::Finished {

      // Execute logic based on Agent State
      match &self.attributes.state {

        // Write initial backend code
        AgentState::Discovery => {

          // Guard: Ensure backend is required
          if !is_crud_required && !is_user_login_and_logout {
            self.attributes.state = AgentState::Finished;
            continue;
          }

          // Write initial backend code
          self.call_initial_backend_code(factsheet).await;
          self.attributes.state = AgentState::Working;
          continue;
        }

        // Check and improve upon code
        AgentState::Working => {

          // Check and Enhance Code
          if self.bug_count == 0 {

            // Improve backend code
            self.call_improved_backend_code(factsheet).await;
            self.attributes.state = AgentState::UnitTesting;
            continue;

          // Correct for errors
          } else {

            // Fix code bugs
            self.call_fix_code_bugs(factsheet).await;
            self.attributes.state = AgentState::UnitTesting;
            continue;
          }
        },

        // Check Code Builds
        AgentState::UnitTesting => {

          // Guard: Ensure safe code
          PrintCommand::UnitTest.print_agent_message(self.attributes.position.as_str(), "Backend Unit Testing: ensure safe code...");
          let is_safe_code: bool = confirm_safe_code();
          if !is_safe_code {
            panic!("Better go work on some AI alignment instead...")
          }

          // Build backend application
          PrintCommand::UnitTest.print_agent_message(self.attributes.position.as_str(), "Backend Unit Testing: building...");
          let build_backend_server: std::process::Output = Command::new("cargo")
            .arg("build")
            .current_dir(BACKEND_CODE_DIR)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .output()
            .expect("Failed to run the backend application");

          // Determine if build errors
          if build_backend_server.status.success() {
            self.bug_count = 0;
            PrintCommand::UnitTest.print_agent_message(self.attributes.position.as_str(), "Test server build successful...");
          } else {
            let error_arr: Vec<u8> = build_backend_server.stderr;
            let error_str: String = String::from_utf8(error_arr).unwrap();

            // Update error stats
            self.bug_count += 1;
            self.bug_errors = Some(error_str);

            // Exit if too many bug counts
            if self.bug_count > 2 {
              PrintCommand::Issue.print_agent_message(self.attributes.position.as_str(), "Exiting agent. Too many bugs found in code.");
              panic!("Error: Too many bugs")
            }

            // Pass back for rework
            self.attributes.state = AgentState::Working;
            continue;
          }

          // Extract API Endpoints
          let api_endpoints_str: String = self.call_extract_rest_api_endpoints().await;

          // Convert API Endpoints into Values
          let api_endpoints: Vec<RouteObject> = serde_json::from_str(api_endpoints_str.as_str())
            .expect("Failed to decode API Endpoints");

          // Extract API Endpoints
          let check_endpoints: Vec<RouteObject> = api_endpoints.iter()
            .filter(|&route_object| route_object.method == "get" && route_object.is_route_dynamic == "false")
            .cloned()
            .collect();

          // Store API Endpoints
          factsheet.api_endpoint_schema = Some(check_endpoints.clone());

          // Build backend application
          PrintCommand::UnitTest.print_agent_message(self.attributes.position.as_str(), "Backend Unit Testing: Starting server...");
          let mut run_backend_server: std::process::Child = Command::new("cargo")
            .arg("run")
            .current_dir(BACKEND_CODE_DIR)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .expect("Failed to run the backend application");

          // Sleep for 5 seconds
          PrintCommand::UnitTest.print_agent_message(self.attributes.position.as_str(), "Launching tests on server in 5 seconds...");
          let seconds_sleep: Duration = Duration::from_secs(5);
          time::sleep(seconds_sleep).await;

          // Create client with timeout
          let client: Client = Client::builder()
            .timeout(Duration::from_secs(5))
            .build()
            .unwrap();

          // Check status code
          for endpoint in check_endpoints {

            // Confirm url testing
            let testing_msg: String = format!("Testing endpoint '{}'...", endpoint.route);
            PrintCommand::UnitTest.print_agent_message(self.attributes.position.as_str(), testing_msg.as_str());

            // Test url
            let url: String = format!("http://localhost:8080{}", endpoint.route);
            match check_status_code(&client, &url).await {
              Ok(status_code) => {
                if status_code != 200 {
                  let err_msg: String = format!("WARNING: Failed to call backend url endpoint {}", endpoint.route);
                  PrintCommand::Issue.print_agent_message(self.attributes.position.as_str(), err_msg.as_str());
                }
              }
              Err(e) => {
                // kill $(lsof -t -i:8080)
                run_backend_server.kill().expect("Failed to kill the backend web server");
                let err_msg: String = format!("Error checking backend: {}", e);
                PrintCommand::Issue.print_agent_message(self.attributes.position.as_str(), err_msg.as_str());
              },
            }
          }

          // Save API Endpoints
          save_api_endpoints(&api_endpoints_str);

          // Kill backend server
          PrintCommand::UnitTest.print_agent_message(self.attributes.position.as_str(), "Backend testing complete...");
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
