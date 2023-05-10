use crate::models::agent_basic::basic_agent::BasicAgent;
use crate::models::agents_manager::managing_agent::ManagingAgent;
use async_trait::async_trait;
use serde::{Serialize, Deserialize};
use std::fmt::Debug;
use std::any::Any;


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UrlItem {
  pub rest_api_endpoint: String,
  pub rest_api_purpose: String,
}


#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ProjectScope {
  pub is_crud_required: bool,
  pub is_user_login_and_logout: bool,
  pub is_external_urls_required: bool,
}


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FactSheet {
  pub project_description: String,
  pub project_scope: Option<ProjectScope>,
  pub external_urls: Option<Vec<String>>,
  pub backend_code: Option<String>,
  pub frontend_code: Option<String>,
  pub json_db_schema: Option<String>
}


// Trait functionality
// This will be applied to each agent uniquely
#[async_trait]
pub trait SpecialFunctions: Debug + Any { // 'Any' so that we can downcast the Structs in the managing agent
    
    // Used so that manager can get attributes info from Agents
    fn get_attributes_from_agent(&self) -> &BasicAgent;
    
    // The function in which all agents will execute their logic in
    async fn execute(&mut self, factsheet: &mut FactSheet) -> Result<(), Box<dyn std::error::Error>>;
}
