use crate::models::agent_basic::basic_agent::{BasicAgent, AgentState};
use crate::models::agent_basic::basic_traits::BasicTraits;
use crate::models::agents::agent_traits::{SpecialFunctions, FactSheet, ProjectScope};
use crate::models::general::llm::Message;
use crate::ai_functions::solution_architect::print_project_scope;
use crate::helpers::general::extend_ai_function;
use crate::apis::call_request::call_gpt;
use async_trait::async_trait;


// Solution Architect
#[derive(Debug)]
pub struct AgentSolutionArchitect {
  attributes: BasicAgent
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
      attributes
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
        let msg_context: String = format!("{:?}", factsheet.project_description);
        let func_message: Message = extend_ai_function(print_project_scope, &msg_context);

        // Call GPT - Construct factsheet
        println!("{} Agent: Scoping project...", {self.attributes.get_position()});
        let project_scope_ser: String = call_gpt(vec!(func_message)).await.expect("Failed to get response from LLM on factsheet");

        // Construct factsheet
        let project_scope: ProjectScope = serde_json::from_str(project_scope_ser.as_str()).expect("Failed to decode LLM response message");
        
        // Append factsheet
        factsheet.project_scope = Some(project_scope);

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
      project_description: "Build a full stack website with user login and logout that shows latest Forex prices".to_string(),
      project_scope: None,
      external_urls: None,
      backend_code: None,
      frontend_code: None,
      json_db_schema: None
    };

    // Execute running agent
    agent.execute(&mut factsheet).await.expect("Unable to execute running agent");
    assert!(factsheet.project_scope != None);
  }
}

