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


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct InitialSpec {
  pub user_login_required: bool,
  pub website_purpose: Option<String>,
  pub preferred_data_storage: Option<String>,
  pub main_colors: Option<Vec<String>>,
  pub other_info_database: Option<String>,
  pub other_info_backend: Option<String>,
  pub other_info_frontend: Option<String>
}


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CargoImports {
  pub cargo_package_name: String,
  pub cargo_package_features: Option<Vec<String>>
}


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct YarnImports {
  pub yarn_package_name: String,
  pub yarn_package_version: Option<Vec<String>>
}


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FactSheet {
  pub project_goal: String,
  pub initial_spec: Option<InitialSpec>,
  pub urls: Option<Vec<UrlItem>>,
  pub db_schema: Option<String>,
  pub cargo_imports: Option<Vec<CargoImports>>,
  pub yarn_imports: Option<Vec<YarnImports>>,
  pub backend_rest_api_urls: Option<Vec<String>>
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
