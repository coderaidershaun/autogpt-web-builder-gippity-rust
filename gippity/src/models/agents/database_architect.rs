// use crate::models::agent_basic::basic_agent::{BasicAgent, AgentState};
// use crate::models::agent_basic::basic_traits::BasicTraits;
// use crate::models::agents::agent_traits::{SpecialFunctions, FactSheet, InitialSpec};
// use crate::models::general::llm::Message;
// use crate::ai_functions::database_architect::{list_database_tables, database_schema};
// use crate::helpers::general::{extend_ai_function};
// use crate::apis::call_request::call_gpt;
// use async_trait::async_trait;


// // Solution Architect
// #[derive(Debug)]
// pub struct AgentDatabaseArchitect {
//   attributes: BasicAgent,
//   db_tables: Option<String>,
// }

// impl AgentDatabaseArchitect {
//   pub fn new() -> Self {

//     // Define attributes
//     let attributes: BasicAgent = BasicAgent {
//       objective: "structure the database format for website build".to_string(),
//       position: "Database Architect".to_string(),
//       state: AgentState::Discovery,
//       memory: vec![]
//     };

//     // Return Self
//     Self {
//       attributes,
//       db_tables: None,
//     }
//   }
// }


// #[async_trait]
// impl SpecialFunctions for AgentDatabaseArchitect {

//   fn get_attributes_from_agent(&self) -> &BasicAgent {
//     &self.attributes
//   }

//   async fn execute(&mut self, factsheet: &mut FactSheet) -> Result<(), Box<dyn std::error::Error>> {

//     // Extract initial spec
//     let initial_spec: &Option<String> = match &factsheet.initial_spec {
//       Some(initial_spec) => &initial_spec.website_purpose,
//       None => panic!("No initial spec found")
//     };

//     // Extract site purpose
//     let site_purpose: &String = match initial_spec {
//       Some(site_purp) => site_purp,
//       None => &factsheet.project_goal
//     };

//     // Concatenate site purpose with Project goal
//     let project_goal: &String = &factsheet.project_goal;
//     let site_purpose_concat: String = format!("Site Project Goal: {}, Site Purpose: {}", 
//       project_goal, site_purpose);

//     // Continue until finished
//     // !!! WARNING !!!
//     while self.attributes.state != AgentState::Finished {

//       // Execute logic based on Agent State
//       match &self.attributes.state {

//         // Confirm tables which require a Schema
//         AgentState::Discovery => {

//           // Guard: Ensure DB is required
//           match &factsheet.initial_spec.as_ref().unwrap().preferred_data_storage {
//             Some(solution) => {println!("Initializing {} data setup...", solution)},
//             None => self.attributes.state = AgentState::Finished
//           }

//           // Extract list tables required
//           let func_message: Message = extend_ai_function(list_database_tables, site_purpose_concat.as_str());

//           // Call GPT - Confirm tables required
//           println!("{} Agent: Confirming JSON tables required...", {self.attributes.get_position()});
//           let db_tables_required: String = call_gpt(vec!(func_message)).await
//             .expect("Failed to get response from LLM database tables listing");

//           // Update tables required
//           self.db_tables = Some(db_tables_required);

//           // Change state to working
//           self.attributes.state = AgentState::Working;
//           continue;
//         }

//         // Determine URLs required for site
//         AgentState::Working => {

//           // Extract database ai function message
//           let msg: String = format!("Site Purpose: {}, DB Tables: {}", site_purpose_concat, self.db_tables
//             .as_ref().unwrap());
//           let func_message: Message = extend_ai_function(database_schema, msg.as_str());

//           // Call GPT - Database Schema
//           println!("{} Agent: Confirming JSON database Schema...", {self.attributes.get_position()});
//           let db_json_schema: String = call_gpt(vec!(func_message)).await
//             .expect("Failed to get response from LLM database JSON schema");
          
//           // Update agent
//           factsheet.db_schema = Some(db_json_schema);

//           // Confirm done
//           self.attributes.state = AgentState::Finished;
//         },

//         // Ensure all cases are covered
//         _ => {}
//       }
//     }
//     Ok(())
//   }
// }



// #[cfg(test)]
// pub mod tests {
//   use super::*;

//   #[test]
//   fn creates_new_agen_db_architect() {
//     let agent: AgentDatabaseArchitect = AgentDatabaseArchitect::new();
//     assert_eq!(agent.attributes.position, "Database Architect");
//   }

//   #[tokio::test]
//   async fn constructs_database_json_schema() {

//     // Create agent instance and site purpose
//     let mut agent: AgentDatabaseArchitect = AgentDatabaseArchitect::new();

//     // Get initial Spec
//     let initial_spec: InitialSpec = InitialSpec {
//       user_login_required: false,
//       website_purpose: Some("Get the latest prices from Binance and Kraken".to_string()),
//       preferred_data_storage: Some("JSON".to_string()),
//       main_colors: None,
//       other_info_database: None,
//       other_info_backend: None,
//       other_info_frontend: None
//     };

//     // Initialze Factsheet
//     let mut factsheet: FactSheet = FactSheet {
//       project_goal: "Build a cool SAAS site for my crypto project".to_string(),
//       initial_spec: Some(initial_spec),
//       urls: None,
//       db_schema: None,
//       cargo_imports: None,
//       yarn_imports: None,
//       backend_rest_api_urls: None
//     };

//     // Execute running agent
//     agent.execute(&mut factsheet).await.expect("Unable to execute running agent");
//     dbg!(&factsheet);
//     assert!(factsheet.db_schema != None);
//   }
// }

