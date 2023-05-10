use crate::models::agent_basic::basic_agent::{BasicAgent, AgentState};
use crate::models::agents::agent_traits::{SpecialFunctions, FactSheet};
use crate::models::agents::solution_architect::AgentSolutionArchitect;
use crate::models::agents::url_manager::AgentUrlManager;
use crate::models::agents::database_architect::AgentDatabaseArchitect;
use crate::models::general::llm::Message;
use crate::ai_functions::managing_agent::convert_user_input_to_goal;
use crate::helpers::general::extend_ai_function;
use crate::apis::call_request::call_gpt;


#[derive(Debug)]
pub struct ManagingAgent {
  attributes: BasicAgent,
  factsheet: FactSheet,
  agents: Vec<Box<dyn SpecialFunctions>>,
}

impl ManagingAgent {

  // Create new instance of managing agent
  pub async fn new(usr_req: String) -> Result<Self, Box<dyn std::error::Error>> {

    // Define attributes
    let attributes: BasicAgent = BasicAgent {
      objective: "manage agents who are building a website for an end user".to_string(),
      position: "Project Manager".to_string(),
      state: AgentState::Discovery,
      memory: vec![]
    };

    // Convert AI Function to Goal
    let func_message: Message = extend_ai_function(convert_user_input_to_goal, &usr_req);
    let project_goal: String = call_gpt(vec!(func_message)).await?;

    // Initialize agents
    let agents: Vec<Box<dyn SpecialFunctions>> = vec![];

    // Initialze Factsheet
    let factsheet: FactSheet = FactSheet {
      project_goal,
      initial_spec: None,
      urls: None,
      db_schema: None,
      cargo_imports: None,
      yarn_imports: None,
      backend_rest_api_urls: None
    };

    // Initialize 

    // Return Self
    Ok(Self {
      attributes,
      factsheet,
      agents
    })
  }


  // Private: Creates an instance of all agents
  // Important: Creates agents in order of project task execution
  fn create_agents(&mut self) {
    self.add_agent(Box::new(AgentSolutionArchitect::new()));
    self.add_agent(Box::new(AgentUrlManager::new()));
    self.add_agent(Box::new(AgentDatabaseArchitect::new()));
  }

  // Private: Adds an agent
  fn add_agent(&mut self, agent: Box<dyn SpecialFunctions>) {
    self.agents.push(agent);
  }

  // Public: Creates and manages project
  pub async fn execute_project(&mut self) {

    // Create agents
    self.create_agents();

    // Execute program for each agent
    for agent in &mut self.agents {

      // Handle if Solutions Architect
      agent.execute(&mut self.factsheet).await;

      // if agent.get_attributes_from_agent().position == "URL Manager" {
      //   break;
      // }
    }
  }
}



#[cfg(test)]
pub mod tests {
  use super::*;

  #[tokio::test]
  async fn creates_new_managing_agent() {
    let usr_request: &str = "need a website that looks great and handles storing user data";
    let managing_agent = ManagingAgent::new(usr_request.to_string()).await.expect("Error creating agent");
    dbg!(&managing_agent);
    assert_eq!(managing_agent.attributes.position, "Project Manager")
  }

  #[tokio::test]
  async fn executes_building_a_website() {
    let usr_request: &str = "need a full stack app that fetches binance crypto prices";
    let mut managing_agent: ManagingAgent = ManagingAgent::new(usr_request.to_string()).await.expect("Error creating agent");

    managing_agent.execute_project().await;

    let encoded_factsheet = serde_json::to_string(&managing_agent.factsheet).unwrap();

    println!("{:?}", encoded_factsheet);
  }
}