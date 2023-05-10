use crate::models::agent_basic::basic_agent::{BasicAgent, AgentState};
use crate::models::agent_basic::basic_traits::BasicTraits;
use crate::models::agents::agent_traits::{SpecialFunctions, FactSheet};
use crate::models::general::llm::Message;
use crate::ai_functions::url_manager::print_site_urls;
use crate::helpers::general::{extend_ai_function, check_status_code};
use crate::apis::call_request::call_gpt;
use async_trait::async_trait;

use reqwest::Client;
use std::time::Duration;


// Solution Architect
#[derive(Debug)]
pub struct AgentUrlManager {
  attributes: BasicAgent,
}

impl AgentUrlManager {
  pub fn new() -> Self {

    // Define attributes
    let attributes: BasicAgent = BasicAgent {
      objective: "Discovers and lists external urls if needed".to_string(),
      position: "URL Manager".to_string(),
      state: AgentState::Discovery,
      memory: vec![]
    };

    // Return Self
    Self {
      attributes
    }
  }
}


#[async_trait]
impl SpecialFunctions for AgentUrlManager {

  fn get_attributes_from_agent(&self) -> &BasicAgent {
    &self.attributes
  }

  async fn execute(&mut self, factsheet: &mut FactSheet) -> Result<(), Box<dyn std::error::Error>> {

    // Extract Project Scope
    let (is_external_urls_required, project_description) = match &factsheet.project_scope {
      Some(project_scope) => {
        (project_scope.is_external_urls_required, &factsheet.project_description)
      },
      None => panic!("Project Scope required before calling Agent")
    };

    // Continue until finished
    // !!! WARNING !!!
    while self.attributes.state != AgentState::Finished {

      // Execute logic based on Agent State
      match &self.attributes.state {

        // Confirm if work is required for this agent
        AgentState::Discovery => {

          // Guard: Ensure External URLS are required
          if !is_external_urls_required {
            self.attributes.state = AgentState::Finished;
          } else {
            self.attributes.state = AgentState::Working;
          }
        }

        // Determine URLs required for site
        AgentState::Working => {

          // Extract is site urls required function
          let func_message: Message = extend_ai_function(print_site_urls, project_description.as_str());

          // Call GPT - Get external API urls
          println!("{} Agent: Constructing API endpoint urls...", {self.attributes.get_position()});
          let api_endpoints_str: String = call_gpt(vec!(func_message)).await.expect("Failed to get response from LLM on API endpoints");

          // Decode response
          let api_endpoints: Vec<String> = serde_json::from_str(api_endpoints_str.as_str())
            .expect("Expected Vec<String> response from LLM decoding");
          
          // Set state
          factsheet.external_urls = Some(api_endpoints);
          self.attributes.state = AgentState::UnitTesting;
        },

        // Test URLs are working
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
            let new_urls = factsheet.external_urls.as_ref().unwrap()
              .iter().filter(|url| !exclude_urls.contains(&url)).cloned().collect();
            factsheet.external_urls = Some(new_urls);
          }

          // Confirm done
          self.attributes.state = AgentState::Finished;
        },

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
  use crate::models::agents::agent_traits::{ProjectScope};

  #[test]
  fn creates_new_agent_url_manager() {
    let agent: AgentUrlManager = AgentUrlManager::new();
    assert_eq!(agent.attributes.position, "URL Manager");
  }

  #[tokio::test]
  async fn confirms_if_urls_required() {

    // Create agent instance and site purpose
    let mut agent: AgentUrlManager = AgentUrlManager::new();

    // Get initial Spec
    let project_scope: ProjectScope = ProjectScope {
      is_crud_required: true,
      is_user_login_and_logout: true,
      is_external_urls_required: true
    };

    // Initialze Factsheet
    let mut factsheet: FactSheet = FactSheet {
      project_description: "Build a full stack website with user login and logout that shows latest Forex prices".to_string(),
      project_scope: Some(project_scope),
      external_urls: None,
      backend_code: None,
      frontend_code: None,
      json_db_schema: None
    };


    // Execute running agent
    agent.execute(&mut factsheet).await.expect("Unable to execute running agent");
    let json_factsheet: String = serde_json::to_string(&factsheet).unwrap();
    println!("{:?}", json_factsheet);
    assert!(factsheet.external_urls.unwrap().len() > 0);
  }
}

