use crate::ai_functions::frontend_developer::{
  print_code_bugs_resolution,
  print_recommended_site_pages,
  print_recommended_site_pages_with_apis, 
  print_recommended_site_main_colours,
  prints_svg_logo,
  prints_completed_logo_with_brand_name_react_component,
  print_header_navigation_react_component,
  print_footer_navigation_react_component,
  print_react_typescript_hook_component
};
use crate::models::agents::agent_traits::{
  SpecialFunctions, 
  FactSheet
};

use crate::helpers::general::{
  extend_ai_function, 
  save_frontend_code,
  check_status_code, 
  read_code_template_contents, 
  save_api_endpoints,
  read_frontend_code_contents,
  BACKEND_CODE_DIR,
  FRONTEND_CODE_DIR
};
use crate::helpers::command_line::confirm_safe_code;
use crate::models::agent_basic::basic_agent::{BasicAgent, AgentState};
use crate::models::agent_basic::basic_traits::BasicTraits;
use crate::models::general::llm::Message;
use crate::apis::call_request::call_gpt;
use async_trait::async_trait;
use serde::{Serialize, Deserialize};
use std::fs;
use std::process::{Command, Stdio};
use std::time::Duration;
use tokio::time;
use reqwest::Client;
use std::collections::HashMap;


// To define what stage the frontend developer is at
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum FrontendBuildMode {
  Infrastructure = 1,
  PageComponents = 2,
  Completion = 3
}


// To define what stage the component development for each page is at
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum FrontendPageStage {
  Content,
  Wireframing,
  Developing,
  APIIntegration,
  Styling,
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
  pub page_stage: Option<FrontendPageStage>,
  pub build_mode: FrontendBuildMode
}


// Solution Architect
#[derive(Debug)]
pub struct AgentFrontendDeveloper {
  attributes: BasicAgent,
  buildsheet: DesignBuildSheet,
  bug_count: u8,
  error_code: Option<String>,
  operation_focus: String
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
      page_stage: None,
      build_mode: FrontendBuildMode::Infrastructure
    };

    // Return Self
    Self {
      attributes,
      buildsheet,
      bug_count: 0,
      error_code: None,
      operation_focus: "logo".to_string()
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
    let msg_context: String = format!("PROJECT_DESCRIPTION: {:?}, CODE_LOGIC: {:?}", 
      project_description, backend_code);
    let func_message: Message = extend_ai_function(print_recommended_site_pages, &msg_context);

    // Call GPT - Obtain website pages
    println!("{} Agent: Reviewing page architecture...", {self.attributes.get_position()});
    let frontend_pages_schema: String = call_gpt(vec!(func_message)).await
      .expect("Failed to get response from LLM");

    // Decode pages schema
    let decoded_pages_schema: Vec<SitePages> = serde_json::from_str(frontend_pages_schema.as_str())
      .expect("Failed to decode JSON Schema");

    // Extract pages
    let pages: Vec<String> = decoded_pages_schema
      .iter().filter_map(|item| Some(item.page_name.clone())).collect();

    // Assign pages to buildsheet
    self.buildsheet.pages = Some(pages.clone());
    self.buildsheet.pages_descriptons = Some(decoded_pages_schema);
  }


  // Assign API Routes to pages
  async fn assign_api_routes(&mut self, project_description: &String, external_api_urls: &Option<Vec<String>>) {

    // Extract internal API schema and external api urls
    let path: String = format!("{}/api_endpoints.json", BACKEND_CODE_DIR);
    let internal_api_endpoints: String = fs::read_to_string(path).expect("Something went wrong reading the file");
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
    let func_message: Message = extend_ai_function(print_recommended_site_pages_with_apis, &msg_context);

    // Call GPT - Assign endpoints to website pages
    println!("{} Agent: Assigning endpoints to pages...", {self.attributes.get_position()});
    let pages_apis_schema: String = call_gpt(vec!(func_message)).await
      .expect("Failed to get response from LLM");

    // Decode pages api assignment schema
    let decoded_api_assign_schema: PageRoutes = serde_json::from_str(pages_apis_schema.as_str())
      .expect("Failed to decode JSON Schema");

    // Add API assignments to buildsheet
    self.buildsheet.api_assignments = Some(decoded_api_assign_schema);
  }


  // Define Brand Colours
  async fn define_brand_colours(&mut self, project_description: &String) {

    // Structure message
    let msg_context: String = format!("PROJECT_DESCRIPTION: {}, WEBSITE_CONTENT: {:?}", 
      project_description, self.buildsheet.pages_descriptons);
    let func_message: Message = extend_ai_function(print_recommended_site_main_colours, &msg_context);

    // Call GPT - Assign endpoints to website pages
    println!("{} Agent: Defining brand colours...", {self.attributes.get_position()});
    let brand_colours_list: String = call_gpt(vec!(func_message)).await
      .expect("Failed to get response from LLM");

    // Decode pages api assignment schema
    let decoded_brand_colours: Vec<String> = serde_json::from_str(brand_colours_list.as_str())
      .expect("Failed to decode JSON Schema");

    // Add API assignments to buildsheet
    self.buildsheet.brand_colours = Some(decoded_brand_colours);
  }


  // Build logo component
  async fn create_logo_component(&mut self, project_description: &String, file_path: &String) {

    // Structure message
    let msg_context: String = format!("PROJECT_DESCRIPTION: {}, BRAND_COLOURS: {:?}", 
      project_description, self.buildsheet.brand_colours);
    let func_message: Message = extend_ai_function(prints_svg_logo, &msg_context);

    // Call GPT - Build SVG Logo
    println!("{} Agent: Building SVG Logo...", {self.attributes.get_position()});
    let svg_logo_code: String = call_gpt(vec!(func_message)).await
      .expect("Failed to get response from LLM");

    // Structure message for logo creation
    let msg_context: String = format!("WEBSITE SPECIFICATION: {{
      SVG_LOGO: {},
      PAGES: {:?},
    }}", project_description, svg_logo_code);
    let func_message: Message = extend_ai_function(
      prints_completed_logo_with_brand_name_react_component, 
      &msg_context);

    // Call GPT - Build complete Logo
    println!("{} Agent: Building Logo Component...", {self.attributes.get_position()});
    let logo_component: String = call_gpt(vec!(func_message)).await
      .expect("Failed to get response from LLM");

    // Save Component
    save_frontend_code(file_path, &logo_component);
  }


  // Build navigation header component
  async fn create_navigation_header_component(&mut self, project_description: &String, file_path: &String) {

    // Structure message
    let pages: &Vec<String> = self.buildsheet.pages.as_ref().expect("Missing pages");
    let msg_context: String = format!("WEBSITE_SPECIFICATION: {{
      PROJECT_DESCRIPTION: {},
      PAGES_WHICH_NEED_LINKS: {:?},
      COLOUR_SCHEME: {:?}
    }}", 
      project_description, pages, self.buildsheet.brand_colours);
    let func_message: Message = extend_ai_function(
      print_header_navigation_react_component, &msg_context);

    // Call GPT - Build SVG Logo
    println!("{} Agent: Building Navigation component...", {self.attributes.get_position()});
    let navigation_component: String = call_gpt(vec!(func_message)).await
      .expect("Failed to get response from LLM");

    // Save Component
    save_frontend_code(file_path, &navigation_component);
  }


  // Build navigation footer component
  async fn create_navigation_footer_component(&mut self, project_description: &String, file_path: &String) {

    // Structure message
    let pages: &Vec<String> = self.buildsheet.pages.as_ref().expect("Missing pages");
    let msg_context: String = format!("WEBSITE_SPECIFICATION: {{
      PROJECT_DESCRIPTION: {},
      PAGES_WHICH_NEED_LINKS: {:?},
      COLOUR_SCHEME: {:?}
    }}", 
      project_description, pages, self.buildsheet.brand_colours);
    let func_message: Message = extend_ai_function(
      print_footer_navigation_react_component, &msg_context);

    // Call GPT - Build SVG Logo
    println!("{} Agent: Building Footer component...", {self.attributes.get_position()});
    let navigation_component: String = call_gpt(vec!(func_message)).await
      .expect("Failed to get response from LLM");

    // Save Component
    save_frontend_code(file_path, &navigation_component);
  }


  // Fix buggy component code
  async fn run_code_correction(&self, file_path: String) {

    // Initialize
    println!("Fixing component bugs...");
    let buggy_code: String = read_frontend_code_contents(&file_path);

    // Structure message
    let msg_context: String = format!("ORIGINAL_CODE: {}, ERROR_MESSAGE: {:?}", 
      buggy_code, self.error_code);
    let func_message: Message = extend_ai_function(print_code_bugs_resolution, &msg_context);

    // Call GPT - Correcting component code
    println!("{} Agent: Working on component bug fixes...", {self.attributes.get_position()});
    let updated_code: String = call_gpt(vec!(func_message)).await
      .expect("Failed to get response from LLM");

    // Save Component
    save_frontend_code(&file_path, &updated_code);
  }


  // Frontend component test
  async fn perform_component_test(&mut self) -> Result<(), String> {
    println!("Testing component for {}...", self.operation_focus);
    let build_frontend_server: std::process::Output = Command::new("yarn")
      .arg("build")
      .current_dir(FRONTEND_CODE_DIR)
      .stdout(Stdio::piped())
      .stderr(Stdio::piped())
      .output()
      .expect("Failed to run component test");

    // Determine if build errors
    if build_frontend_server.status.success() {
      println!("Frontend component build successful...");
      self.bug_count = 0;
      return Ok(());

    // Handle Build error
    } else {
      let error_arr: Vec<u8> = build_frontend_server.stderr;
      let error_str: String = String::from_utf8(error_arr).unwrap();

      // Check and return error
      self.bug_count += 1;
      self.error_code = Some(error_str);
      if self.bug_count >= 2 {
        panic!("Too many code failed attempts for {}", self.operation_focus);
      } else {
        return Err("Build failed".to_string())
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
        },

        // Get pages, api assignments and branding
        AgentState::Working => {
          
          // Complete on building tasks around Infrastructure
          if self.buildsheet.page_stage == None {

            // Create logo
            if self.operation_focus == "logo" {
              
              // Create or correct code
              let file_path: String = "/src/components/shared/Logo.tsx".to_string();
              if self.bug_count == 0 {
                self.create_logo_component(&project_description, &file_path).await;
              } else {
                self.run_code_correction(file_path).await;
              }

              // Test Component
              match self.perform_component_test().await {
                Ok(_) => {
                  self.operation_focus = "navigation_header".to_string();
                },
                Err(_) => {
                  continue; // Loop back and perform code correction
                }
              }
            }

            // Create navigation header
            if self.operation_focus == "navigation_header" {

              // Create component
              let file_path: String = "/src/components/shared/Navigation.tsx".to_string();
              if self.bug_count == 0 {
                self.create_navigation_header_component(&project_description, &file_path).await;
              } else {
                self.run_code_correction(file_path).await;
              }

              // Test Component
              match self.perform_component_test().await {
                Ok(_) => {
                  self.operation_focus = "navigation_footer".to_string();
                },
                Err(_) => {
                  continue; // Loop back and perform code correction
                }
              }
            }

            // Create navigation header
            if self.operation_focus == "navigation_footer" {

              // Create component
              let file_path: String = "/src/components/shared/Footer.tsx".to_string();
              if self.bug_count == 0 {
                self.create_navigation_footer_component(&project_description, &file_path).await;
              } else {
                self.run_code_correction(file_path).await;
              }

              // Test Component
              match self.perform_component_test().await {
                Ok(_) => {
                  self.operation_focus = "pages".to_string();
                },
                Err(_) => {
                  continue; // Loop back and perform code correction
                }
              }
            }
            
          }

          // Complete
          self.attributes.state = AgentState::Finished;
        },

        // Check Code Builds
        AgentState::UnitTesting => {



        }

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
  async fn develops_and_saves_initial_schema() {

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
