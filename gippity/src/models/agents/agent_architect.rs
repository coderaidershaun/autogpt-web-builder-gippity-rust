use crate::models::agent_basic::basic_agent::{BasicAgent, AgentState};
use crate::models::agent_basic::basic_traits::BasicTraits;
use crate::models::agents::agent_traits::{SpecialFunctions, FactSheet, ProjectScope};
use crate::ai_functions::aifunc_architect::{print_project_scope, print_site_urls};
use crate::helpers::command_line::PrintCommand;
use crate::helpers::general::{ai_task_request_decoded, check_status_code};
use async_trait::async_trait;

use reqwest::Client;
use std::time::Duration;

// Solution Architect
#[derive(Debug)]
pub struct AgentSolutionArchitect {
  attributes: BasicAgent
}

impl AgentSolutionArchitect {
  pub fn new() -> Self {

    // Define attributes
    let attributes: BasicAgent = BasicAgent {
      objective: "Gathers information and design solutions for website builds".to_string(),
      position: "Solutions Architect".to_string(),
      state: AgentState::Discovery,
      memory: vec![]
    };

    // Return Self
    Self {
      attributes
    }
  }

  // AI Call: Retrieve project scope
  async fn call_project_scope(&mut self, factsheet: &mut FactSheet) -> ProjectScope {
    let msg_context: String = format!("{:?}", factsheet.project_description);

    let ai_response: ProjectScope = ai_task_request_decoded::<ProjectScope>(
      msg_context, 
      &self.attributes.position, 
      get_function_string!(print_project_scope), 
      print_project_scope).await;

    // Update state and return Project Scope
    factsheet.project_scope = Some(ai_response.clone());
    self.attributes.update_state(AgentState::Finished);
    return ai_response;
  }

  // AI Call: Retrieve external urls
  async fn call_determine_external_urls(&mut self, factsheet: &mut FactSheet, msg_context: String) {
    let ai_response: Vec<String> = ai_task_request_decoded::<Vec<String>>(
      msg_context, 
      &self.attributes.position, 
      get_function_string!(print_site_urls), 
      print_site_urls).await;
    
    // Update state
    factsheet.external_urls = Some(ai_response);
    self.attributes.state = AgentState::UnitTesting;
  }
}


#[async_trait]
impl SpecialFunctions for AgentSolutionArchitect {

  // Get function attributes (useful for Agent Manager)
  fn get_attributes_from_agent(&self) -> &BasicAgent {
    &self.attributes
  }

  // Execute main functions
  async fn execute(&mut self, factsheet: &mut FactSheet) -> Result<(), Box<dyn std::error::Error>> {

    // Continue until finished
    // !!! WARNING - If this loop runs without a stop, you can incur infinite costs with OpenAI !!!
    while self.attributes.state != AgentState::Finished {

      match self.attributes.state {

        // Scope out project
        AgentState::Discovery => {

          // Get project scope
          let project_scope: ProjectScope = self.call_project_scope(factsheet).await;
      
          // Confirm external urls
          if project_scope.is_external_urls_required {
            self.call_determine_external_urls(factsheet, factsheet.project_description.clone()).await;
            self.attributes.state = AgentState::UnitTesting;
            continue;
          }
        }

        // Perform Uint Testing
        AgentState::UnitTesting => {

          // Initialize urls for exclusion
          let mut exclude_urls: Vec<String> = Vec::from([]);

          // Create client with timeout
          let client: Client = Client::builder()
            .timeout(Duration::from_secs(5))
            .build()
            .unwrap();
          
          // Find faulty URLs to exclude
          let urls: &Vec<String> = factsheet.external_urls.as_ref().expect("No URL object on factsheet");
          for url in urls {

            // Print agent statement
            let endpoint_str: String = format!("Testing URL Endpoint: {}", url);
            PrintCommand::UnitTest.print_agent_message(self.attributes.position.as_str(), endpoint_str.as_str());

            // Perform URL test
            match check_status_code(&client, url).await {
              Ok(status_code) => {
                if status_code != 200 {
                  exclude_urls.push(url.clone())
                }
              }
              Err(e) => println!("Error checking {}: {}", url, e),
            }
          }
        
          // Exclude any faulty URLs
          if exclude_urls.len() > 0 {
            let new_urls: Vec<String> = factsheet.external_urls.as_ref().unwrap()
              .iter().filter(|url| !exclude_urls.contains(&url)).cloned().collect();
            factsheet.external_urls = Some(new_urls);
          }

          // Confirm done
          self.attributes.state = AgentState::Finished;
        },

        _ => { self.attributes.state = AgentState::Finished; }
      }
    }

    Ok(())
  }
}



#[cfg(test)]
pub mod tests {
  use super::*;


  #[tokio::test]
  async fn tests_solution_architect() {

    // Create agent instance and append memory
    let mut agent: AgentSolutionArchitect = AgentSolutionArchitect::new();

    // Initialze Factsheet
    let mut factsheet: FactSheet = FactSheet {
      project_description: "Build a full stack website with user login and logout that shows latest Forex prices".to_string(),
      project_scope: None,
      external_urls: None,
      backend_code: None,
      api_endpoint_schema: None
    };

    // Execute running agent
    agent.execute(&mut factsheet).await.expect("Unable to execute running agent");
    assert!(factsheet.project_scope != None);
    assert!(factsheet.external_urls.is_some());
  }
}

