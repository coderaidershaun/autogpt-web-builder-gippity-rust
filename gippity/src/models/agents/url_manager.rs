use crate::models::agent_basic::basic_agent::{BasicAgent, AgentState};
use crate::models::agent_basic::basic_traits::BasicTraits;
use crate::models::agents::agent_traits::{SpecialFunctions, UrlItem, FactSheet};
use crate::models::general::llm::Message;
use crate::ai_functions::url_manager::{is_site_urls_required, provide_site_urls};
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
      objective: "detect what urls will be required for use within the website application".to_string(),
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

    // Concatenate site purpose with Project goal
    let project_goal: &String = &factsheet.project_goal;
    let site_purpose_concat: String = format!("Site Project Goal: {}, Site Purpose: {}", 
      project_goal, site_purpose);

    // Continue until finished
    // !!! WARNING !!!
    while self.attributes.state != AgentState::Finished {

      // Execute logic based on Agent State
      match &self.attributes.state {

        // Confirm if work is required for this agent
        AgentState::Discovery => {

          // Extract is site urls required function
          let func_message: Message = extend_ai_function(is_site_urls_required, site_purpose_concat.as_str());

          // Call GPT - Confirm if Site purpose
          println!("{} Agent: Confirming if external urls are required...", {self.attributes.get_position()});
          let is_urls_required_str: String = call_gpt(vec!(func_message)).await.expect("Failed to get response from LLM on whether urls are required");

          // Decode response
          let is_urls_required: bool = serde_json::from_str(is_urls_required_str.as_str())
            .expect("Expected a boolean response from LLM decoding");
          
          // Set state
          if is_urls_required {
              self.attributes.state = AgentState::Working;
              continue;
          } else {
              self.attributes.state = AgentState::Finished;
          }
        }

        // Determine URLs required for site
        AgentState::Working => {

          // Extract Site Purpose function
          let func_message: Message = extend_ai_function(provide_site_urls, site_purpose_concat.as_str());

          // Call GPT - Confirm if Site purpose
          println!("{} Agent: Constructing API endpoint urls...", {self.attributes.get_position()});
          let api_endpoints_str: String = call_gpt(vec!(func_message)).await.expect("Failed to get response from LLM on API endpoints");

          // Decode response
          let api_endpoints: Vec<UrlItem> = serde_json::from_str(api_endpoints_str.as_str())
            .expect("Expected Vec<UrlItem> response from LLM decoding");

          // Test URLs
          if api_endpoints.len() > 0 {
            factsheet.urls = Some(api_endpoints);
            self.attributes.state = AgentState::UnitTesting;
          } else {
            self.attributes.state = AgentState::Finished;
          }
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
          let urls: &Vec<UrlItem> = factsheet.urls.as_ref().expect("No URL object on factsheet");
          for url_item in urls {
            let url: &String = &url_item.rest_api_endpoint;
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
            let new_urls: Vec<UrlItem> = factsheet.urls.as_ref().unwrap()
              .iter().filter(|url| !exclude_urls.contains(&url.rest_api_endpoint)).cloned().collect();
            factsheet.urls = Some(new_urls);
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
  use crate::models::agents::agent_traits::{InitialSpec};

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
    let initial_spec: InitialSpec = InitialSpec {
      user_login_required: false,
      website_purpose: Some("Get the latest prices from Binance and Kraken".to_string()),
      preferred_data_storage: None,
      main_colors: None,
      other_info_database: None,
      other_info_backend: None,
      other_info_frontend: None
    };

    // Initialze Factsheet
    let mut factsheet: FactSheet = FactSheet {
      project_goal: "Build a cool SAAS site for my crypto project".to_string(),
      initial_spec: Some(initial_spec),
      urls: None,
      db_schema: None,
      cargo_imports: None,
      yarn_imports: None,
      backend_rest_api_urls: None
    };

    // Execute running agent
    agent.execute(&mut factsheet).await.expect("Unable to execute running agent");
    dbg!(&factsheet);
    assert!(factsheet.urls.unwrap().len() > 0);
  }
}

