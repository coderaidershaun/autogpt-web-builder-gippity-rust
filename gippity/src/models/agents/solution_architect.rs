use crate::models::agent_basic::basic_agent::{BasicAgent, AgentState};
use crate::models::agent_basic::basic_traits::BasicTraits;
use crate::models::agents::agent_traits::{SpecialFunctions, InitialSpec, FactSheet};
use crate::models::general::llm::Message;
use crate::ai_functions::solution_architect::{convert_request_to_json_facts, get_website_purpose};
use crate::helpers::command_line::{get_user_response, request_database_preference};
use crate::helpers::general::extend_ai_function;
use crate::apis::call_request::call_gpt;
use async_trait::async_trait;


// Solution Architect
#[derive(Debug)]
pub struct AgentSolutionArchitect {
  attributes: BasicAgent,
  initial_spec: Option<InitialSpec>
}

impl AgentSolutionArchitect {
  pub fn new() -> Self {

    // Define attributes
    let attributes: BasicAgent = BasicAgent {
      objective: "gather information and design solutions for website builds using rust on backend and react on frontend".to_string(),
      position: "Solutions Architect".to_string(),
      state: AgentState::Discovery,
      memory: vec![]
    };

    // Return Self
    Self {
      attributes,
      initial_spec: None
    }
  }
}


#[async_trait]
impl SpecialFunctions for AgentSolutionArchitect {

  fn get_attributes_from_agent(&self) -> &BasicAgent {
    &self.attributes
  }

  async fn execute(&mut self, factsheet: &mut FactSheet) -> Result<(), Box<dyn std::error::Error>> {

    // Execute logic based on Agent State
    match &self.attributes.state {

      // Handle discovery process
      AgentState::Discovery => {

        // Get message from memory
        let msg_context: String = format!("{:?}", factsheet.project_goal);
        let func_message: Message = extend_ai_function(convert_request_to_json_facts, &msg_context);

        // Call GPT - Construct factsheet
        println!("{} Agent: Structuring factsheet...", {self.attributes.get_position()});
        let initial_spec_ser: String = call_gpt(vec!(func_message)).await.expect("Failed to get response from LLM on factsheet");

        // Construct factsheet
        let mut initial_spec: Vec<InitialSpec> = serde_json::from_str(initial_spec_ser.as_str()).expect("Failed to decode LLM response message");
        
        // Ensure the user is asked if they have a database preference
        if initial_spec[0].preferred_data_storage == None {
          initial_spec[0].preferred_data_storage = request_database_preference();
        }

        // Ensure the website has a purpose
        if initial_spec[0].website_purpose == None {

          // Get purpose of website from User
          let user_website_purpose: String = get_user_response("What is the purpose of the website?").to_string();
          let func_message: Message = extend_ai_function(get_website_purpose, &user_website_purpose);

          // Call GPT: Reword purpose
          println!("{} Agent: Confirming Website Purpose...", {self.attributes.get_position()});
          let site_purpose: String = call_gpt(vec!(func_message)).await.expect("Failed to get response from LLM on website purpose");

          // Store purpose in Factsheet
          initial_spec[0].website_purpose = Some(site_purpose);
        }

        // Append factsheet
        factsheet.initial_spec = Some(initial_spec[0].clone());

        // Update agent state to finished
        self.attributes.update_state(AgentState::Finished);
      },
      _ => {}
    }
    Ok(())
  }
}



#[cfg(test)]
pub mod tests {
  use super::*;

  #[test]
  fn creates_new_agent_solution_architect() {
    let agent: AgentSolutionArchitect = AgentSolutionArchitect::new();
    assert_eq!(agent.attributes.position, "Solutions Architect");
  }

  #[tokio::test]
  async fn gathers_initial_spec() {

    // Create agent instance and append memory
    let mut agent: AgentSolutionArchitect = AgentSolutionArchitect::new();

    // Initialze Factsheet
    let mut factsheet: FactSheet = FactSheet {
      project_goal: "Build an awesome website".to_string(),
      initial_spec: None,
      urls: None,
      db_schema: None,
      cargo_imports: None,
      yarn_imports: None,
      backend_rest_api_urls: None
    };

    // Execute running agent
    agent.execute(&mut factsheet).await.expect("Unable to execute running agent");
    dbg!(&factsheet);
    assert!(factsheet.initial_spec.unwrap().website_purpose != None);
  }
}

