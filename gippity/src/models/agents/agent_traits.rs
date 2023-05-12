use crate::models::agent_basic::basic_agent::BasicAgent;
use crate::models::agents_manager::managing_agent::ManagingAgent;
use async_trait::async_trait;
use serde::{Serialize, Deserialize};
use std::fmt::Debug;
use std::any::Any;


#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct RouteObject {
  pub is_route_dynamic: String,
  pub method: String,
  pub request_body: serde_json::Value,
  pub response: serde_json::Value,
  pub route: String,
}


#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq)]
pub struct ProjectScope {
  pub is_crud_required: bool,
  pub is_user_login_and_logout: bool,
  pub is_external_urls_required: bool,
}


#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct FactSheet {
  pub project_description: String,
  pub project_scope: Option<ProjectScope>,
  pub external_urls: Option<Vec<String>>,
  pub backend_code: Option<String>,
  pub api_endpoint_schema: Option<Vec<RouteObject>>,
}


// Trait functionality
// This will be applied to each agent uniquely
#[async_trait]
pub trait SpecialFunctions: Debug {
    
    // Used so that manager can get attributes info from Agents
    fn get_attributes_from_agent(&self) -> &BasicAgent;
    
    // The function in which all agents will execute their logic in
    async fn execute(&mut self, factsheet: &mut FactSheet) -> Result<(), Box<dyn std::error::Error>>;
}
