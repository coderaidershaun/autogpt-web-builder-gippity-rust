use crate::models::agent_basic::basic_agent::{BasicAgent, AgentState};
use crate::models::agent_basic::basic_traits::BasicTraits;
use crate::models::agents::agent_traits::{SpecialFunctions, FactSheet, ProjectScope};
use crate::models::general::llm::Message;
use crate::ai_functions::solution_architect::print_project_scope;
use crate::helpers::general::{ai_task_request_decoded, AIFuncResponse};
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

        // Define agent message
        let msg_context: String = format!("{:?}", factsheet.project_description);
        
        // Get AI Response
        let ai_response: ProjectScope = ai_task_request_decoded::<ProjectScope>(
          msg_context, &self.attributes.position, get_function_string!(print_project_scope), print_project_scope).await;

        // Store results
        factsheet.project_scope = Some(ai_response);
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
      api_endpoint_schema: None
    };

    // Execute running agent
    agent.execute(&mut factsheet).await.expect("Unable to execute running agent");
    assert!(factsheet.project_scope != None);
  }
}

