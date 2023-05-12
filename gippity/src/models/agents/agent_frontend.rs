use crate::ai_functions::aifunc_frontend::{
  print_code_bugs_resolution,
  print_recommended_site_pages,
  print_recommended_site_pages_with_apis, 
  print_recommended_site_main_colours,
};
use crate::helpers::general::{
  save_frontend_code,
  ai_task_request_decoded,
  ai_task_request,
  read_frontend_code_contents,
  BACKEND_CODE_DIR,
  FRONTEND_CODE_DIR
};
use crate::models::agents::agent_frontend_comp::BuildComponent;
use crate::models::agents::agent_traits::{SpecialFunctions, FactSheet};
use crate::models::agent_basic::basic_agent::{BasicAgent, AgentState};
use crate::helpers::command_line::PrintCommand;
use async_trait::async_trait;
use serde::{Serialize, Deserialize};
use std::fs;
use std::process::{Command, Stdio};
use std::collections::HashMap;
use strum::IntoEnumIterator;


// To define what stage the frontend developer is at
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum FrontendBuildMode {
  Infrastructure,
  PageComponents,
  Completion
}


// For decoding the serde_json api routes for a given page
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct APIAssignment {
  pub api_route: String,
  pub method: String,
  pub route_type: String,
}


// Used for creating a type to be used for decoding shorthand
type PageRoutes = HashMap<String, Vec<APIAssignment>>;


// Used for decoding page names and suggested content
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PageAPIAssign {
  pub page: Vec<APIAssignment>
}


// Used for decoding page names and suggested content
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SitePages {
  pub page_name: String,
  pub suggested_content_sections: serde_json::Value
}


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DesignBuildSheet {
  pub pages: Option<Vec<String>>,
  pub pages_descriptons: Option<Vec<SitePages>>,
  pub api_assignments: Option<PageRoutes>,
  pub brand_colours: Option<Vec<String>>,
  pub build_mode: FrontendBuildMode
}


// Solution Architect
#[derive(Debug)]
pub struct AgentFrontendDeveloper {
  pub attributes: BasicAgent,
  pub buildsheet: DesignBuildSheet,
  pub bug_count: u8,
  pub operation_focus: BuildComponent
}

impl AgentFrontendDeveloper {
  pub fn new() -> Self {

    // Define attributes
    let attributes: BasicAgent = BasicAgent {
      objective: "Develops frontned code for website".to_string(),
      position: "Frontend Developer".to_string(),
      state: AgentState::Discovery,
      memory: vec![]
    };

    // Define Buildsheet
    let buildsheet: DesignBuildSheet = DesignBuildSheet {
      pages: None,
      pages_descriptons: None,
      api_assignments: None,
      brand_colours: None,
      build_mode: FrontendBuildMode::Infrastructure
    };

    // Return Self
    Self {
      attributes,
      buildsheet,
      bug_count: 0,
      operation_focus: BuildComponent::Logo
    }
  }


  // Confirms what stage the Frontend Agent is in
  fn confirm_stage(&self) {
    match self.buildsheet.build_mode {
      FrontendBuildMode::Infrastructure => println!("[Working on Frontend Infrastructure]"),
      FrontendBuildMode::PageComponents => println!("[Working on Frontend Page Components]"),
      FrontendBuildMode::Completion => println!("[Working on Frontend Completion Items]"),
    }
  }

  // Get pages and page context from description and backend code
  async fn get_page_context(&mut self, project_description: &String) {

    // Extract backend code
    let path: String = format!("{}/src/main.rs", BACKEND_CODE_DIR);
    let backend_code: String = fs::read_to_string(path).expect("Something went wrong reading the file");

    // Structure Message
    let msg_context: String = format!("PROJECT_DESCRIPTION: {:?}, CODE_LOGIC: {:?}", project_description, backend_code);

    // Call AI
    let ai_response: Vec<SitePages> = ai_task_request_decoded::<Vec<SitePages>>(
      msg_context, 
      &self.attributes.position, 
      get_function_string!(print_recommended_site_pages), 
      print_recommended_site_pages).await;

    // Extract pages
    let pages: Vec<String> = ai_response
      .iter().filter_map(|item| Some(item.page_name.clone())).collect();

    // Assign pages to buildsheet
    self.buildsheet.pages = Some(pages.clone());
    self.buildsheet.pages_descriptons = Some(ai_response);
  }


  // Assign API Routes to pages
  async fn assign_api_routes(&mut self, project_description: &String, external_api_urls: &Option<Vec<String>>) {

    // Extract internal API schema
    let path: String = format!("{}/api_endpoints.json", BACKEND_CODE_DIR);
    let internal_api_endpoints: String = fs::read_to_string(path).expect("Something went wrong reading the file");

    // Extract external API endpoints
    let external_api_endpoints: String = match external_api_urls {
      Some(endpoints) => format!("{:?}", endpoints),
      None => String::from("")
    };

    // Structure message for api route assignment
    let msg_context: String = format!("WEBSITE SPECIFICATION: {{
      PROJECT_DESCRIPTION: {},
      PAGES: {:?},
      INTERNAL_API_ROUTES: {},
      EXTERNAL_API_ROUTES: {} 
    }}", project_description, self.buildsheet.pages, internal_api_endpoints, external_api_endpoints);

    // Call AI
    let ai_response: PageRoutes = ai_task_request_decoded::<PageRoutes>(
      msg_context, 
      &self.attributes.position, 
      get_function_string!(print_recommended_site_pages_with_apis), 
      print_recommended_site_pages_with_apis).await;

    // Add API assignments to buildsheet
    self.buildsheet.api_assignments = Some(ai_response);
  }


  // Define Brand Colours
  async fn define_brand_colours(&mut self, project_description: &String) {

    // Structure message
    let msg_context: String = format!("PROJECT_DESCRIPTION: {}, WEBSITE_CONTENT: {:?}", 
      project_description, self.buildsheet.pages_descriptons);

    // Call AI
    let ai_response: Vec<String> = ai_task_request_decoded::<Vec<String>>(
      msg_context, 
      &self.attributes.position, 
      get_function_string!(print_recommended_site_main_colours), 
      print_recommended_site_main_colours).await;

    // Add decoded brand colours
    self.buildsheet.brand_colours = Some(ai_response);
  }


  // Fix buggy component code
  async fn run_code_correction(&self, file_path: String, error_code: String) {

    // Initialize
    PrintCommand::UnitTest.print_agent_message(self.attributes.position.as_str(), 
      "Fixing component bugs");
    let buggy_code: String = read_frontend_code_contents(&file_path);

    // Structure message
    let msg_context: String = format!("ORIGINAL_CODE: {}, ERROR_MESSAGE: {:?}", buggy_code, error_code);

    // Retrieve AI Reponse
    let ai_response: String = ai_task_request(
      msg_context, 
      &self.attributes.position, 
      get_function_string!(print_code_bugs_resolution), 
      print_code_bugs_resolution).await;

    // Save corrected code
    save_frontend_code(&file_path, &ai_response);
  }


  // Frontend component test
  async fn perform_component_test(&mut self) -> Result<(), String> {
    let test_statement = format!("Testing Component: {}", self.operation_focus.name());
    PrintCommand::UnitTest.print_agent_message(self.attributes.position.as_str(), test_statement.as_str());
    let build_frontend_server: std::process::Output = Command::new("yarn")
      .arg("build")
      .current_dir(FRONTEND_CODE_DIR)
      .stdout(Stdio::piped())
      .stderr(Stdio::piped())
      .output()
      .expect("Failed to run component test");

    // Determine if build errors
    if build_frontend_server.status.success() {
      PrintCommand::UnitTest.print_agent_message(self.attributes.position.as_str(), "Component build test successful");
      self.bug_count = 0;
      return Ok(());

    // Handle Build error
    } else {
      let error_arr: Vec<u8> = build_frontend_server.stderr;
      let error_str: String = String::from_utf8(error_arr).unwrap();

      // Check and return error
      self.bug_count += 1;
      if self.bug_count >= 2 {
        PrintCommand::Issue.print_agent_message(self.attributes.position.as_str(), "Too many code failures");
        PrintCommand::Issue.print_agent_message(self.attributes.position.as_str(), "Remember: check frontend builds before retrying");
        panic!("Too many code failed attempts for {}", self.operation_focus.name());
      } else {
        return Err(error_str)
      }
    }
  }

}


#[async_trait]
impl SpecialFunctions for AgentFrontendDeveloper {

  fn get_attributes_from_agent(&self) -> &BasicAgent {
    &self.attributes
  }

  async fn execute(&mut self, factsheet: &mut FactSheet) -> Result<(), Box<dyn std::error::Error>> {

    // Extract required project factsheet items
    let project_description: &String = &factsheet.project_description;
    let external_api_urls: &Option<Vec<String>> = &factsheet.external_urls;

    // Continue until finished
    // !!! WARNING !!!
    while self.attributes.state != AgentState::Finished {

      // Execute logic based on Agent State
      match &self.attributes.state {

        // Get pages, api assignments and branding
        AgentState::Discovery => {

          // Confirm Stage
          self.confirm_stage();

          // Get pages and page context
          self.get_page_context(&project_description).await;

          // Assign API routes to pages
          self.assign_api_routes(&project_description, &external_api_urls).await;

          // Define Brand Colours
          self.define_brand_colours(&project_description).await;

          // Proceed to Working status
          self.attributes.state = AgentState::Working;
          continue;
        },

        // Get pages, api assignments and branding
        AgentState::Working => {
          
          // Loop through components
          for component in BuildComponent::iter() {
            
            // !!!! REMOVE ONLY FOR TESTING !!!
            if component != BuildComponent::PageContent1 {
              continue;
            }

            // Update current operation focus to component
            self.operation_focus = component.clone();
            component.create_component(&self, &project_description).await;

            // Unit test component
            let test_res: Result<(), String> = self.perform_component_test().await;
            match test_res {

              // Continue to next component
              Ok(()) => continue,

              // Fix bugs for current component
              Err(err_str) => {
                let file_path: String = self.operation_focus.filepath();
                self.run_code_correction(file_path, err_str).await;

                // Perform one more test
                let _ = self.perform_component_test().await;
                continue;
              }
            }
          }

          // Complete
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


  #[tokio::test]
  async fn develops_context_and_branding() {

    // Create agent instance and site purpose
    let mut agent: AgentFrontendDeveloper = AgentFrontendDeveloper::new();

    // Initialze Factsheet
    let mut factsheet: FactSheet = serde_json::from_str("{\"project_description\":\"Build a todo app for a fitness tracking goal\",\"project_scope\":{\"is_crud_required\":true,\"is_user_login_and_logout\":true,\"is_external_urls_required\":true},\"external_urls\":[\"https://api.exchangeratesapi.io/latest\"],\"backend_code\":null,\"frontend_code\":null,\"json_db_schema\":null}").unwrap();

    // Execute running agent
    agent.execute(&mut factsheet).await.expect("Unable to execute running agent");
    dbg!(agent);
  }

  #[tokio::test]
  async fn works_on_shared_components() {

    // Create agent instance and site purpose
    let mut agent: AgentFrontendDeveloper = AgentFrontendDeveloper::new();
    agent.attributes.state = AgentState::Working;
    agent.buildsheet.pages = Some(vec!["home_page".to_string(), "about_page".to_string()]);

    // Initialze Factsheet
    let mut factsheet: FactSheet = serde_json::from_str("{\"project_description\":\"Build a todo app for a fitness tracking goal\",\"project_scope\":{\"is_crud_required\":true,\"is_user_login_and_logout\":true,\"is_external_urls_required\":true},\"external_urls\":[\"https://api.exchangeratesapi.io/latest\"],\"backend_code\":null,\"frontend_code\":null,\"json_db_schema\":null}").unwrap();

    // Execute running agent
    agent.execute(&mut factsheet).await.expect("Unable to execute running agent");
    dbg!(agent);
  }

}
