use crate::ai_functions::aifunc_frontend::{
  print_svg_logo,
  print_completed_logo_with_brand_name_react_component,
  print_header_navigation_react_component,
  print_footer_navigation_react_component,
  print_react_typescript_hook_component
};
use crate::helpers::general::{
  save_frontend_code, 
  ai_task_request, 
  read_frontend_code_contents,
  BACKEND_CODE_DIR, 
  FRONTEND_CODE_DIR
};
use crate::models::agents::agent_frontend::AgentFrontendDeveloper;
use serde::{Serialize, Deserialize};
use std::fs;
use strum_macros::EnumIter;

// Used for decoding page names and suggested content
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, EnumIter)]
pub enum BuildComponent {
  Logo,
  NavHeader,
  NavFooter,
  ReactHook,
  PageContent1,
  PageContent2
}

impl BuildComponent {

  pub fn name(&self) -> &'static str {
    match *self {
      BuildComponent::Logo => "Logo",
      BuildComponent::NavHeader => "NavHeader",
      BuildComponent::NavFooter => "NavFooter",
      BuildComponent::ReactHook => "ReactHook",
      BuildComponent::PageContent1 => "PageContent1",
      BuildComponent::PageContent2 => "PageContent2"
    }
  }

  pub fn filepath(&self) -> String {
    match *self {
      BuildComponent::Logo => "/src/components/shared/Logo.tsx".to_string(),
      BuildComponent::NavHeader => "/src/components/shared/Navigation.tsx".to_string(),
      BuildComponent::NavFooter => "/src/components/shared/Footer.tsx".to_string(),
      BuildComponent::ReactHook => "/src/hooks/useCall.tsx".to_string(),
      BuildComponent::PageContent1 => "/src/components/pages/PageOne.tsx".to_string(),
      BuildComponent::PageContent2 => "/src/components/pages/PageTwo.tsx".to_string()
    }
  }

  // Create component
  async fn create_and_save(&self, msg_context: String,  ai_function: fn(&str) -> &'static str) {

    // Retrieve AI Reponse
    let ai_response: String = ai_task_request(
      msg_context, 
      "Component Writer",
      &self.name(), 
      ai_function).await;

    // Save Component
    save_frontend_code(&self.filepath(), &ai_response);
  }

  // Prepare and create component
  pub async fn create_component(&self, agent: &AgentFrontendDeveloper, project_description: &String) {

    // Extract pages
    let pages: &Vec<String> = agent.buildsheet.pages.as_ref().expect("Missing pages");

    match self {

      Self::Logo => {

        // Create SVG: Structure message
        let msg_context: String = format!("PROJECT_DESCRIPTION: {}, BRAND_COLOURS: {:?}", 
          project_description, agent.buildsheet.brand_colours);

        // Create SVG: Retrieve AI Reponse
        let ai_response_svg_logo: String = ai_task_request(
          msg_context, 
          "Component Writer", 
          get_function_string!(print_svg_logo), 
          print_svg_logo).await;

        // Create SVG: Structure message for logo creation
        let msg_context: String = format!("WEBSITE SPECIFICATION: {{
          SVG_LOGO: {},
          PAGES: {:?},
        }}", project_description, ai_response_svg_logo);

        // Create Component
        self.create_and_save(
          msg_context,
          print_completed_logo_with_brand_name_react_component
        ).await;
      },

      Self::NavHeader | Self::NavFooter => {

        // Structure message
        let msg_context: String = format!("WEBSITE_SPECIFICATION: {{
            PROJECT_DESCRIPTION: {},
            PAGES_WHICH_NEED_LINKS: {:?},
            COLOUR_SCHEME: {:?}
          }}", project_description, pages, agent.buildsheet.brand_colours);

        // Create and Save
        if self.name() == "NavHeader" {
          self.create_and_save(
            msg_context,
            print_header_navigation_react_component).await;
        } else {
          self.create_and_save(
            msg_context,
            print_footer_navigation_react_component).await;
        }
      },

      Self::ReactHook => {

        // Initialize
        let path: String = format!("{}/api_endpoints.json", BACKEND_CODE_DIR);
        let api_endpoints: String = fs::read_to_string(path).expect("Something went wrong reading the file");
        
        // Create and Save
        self.create_and_save(
          format!("API_ENDPOINTS_JSON_SCHEMA: {}", api_endpoints),
          print_react_typescript_hook_component
        ).await;
      },

      Self::PageContent1 | Self::PageContent2 => {

        // Extract page name
        let page_name: &String = match self.name(){
          "PageContent1" => &pages[0],
          "PageContent2" => &pages[1],
          _ => panic!("Page not recognised")
        };

        // Extract file path
        let file_path: String = self.filepath();
        let react_hook_contents: String = read_frontend_code_contents(&file_path);
        

        // Get Content Wireframe


        // Add Styling


        // Add API Endpoints




        dbg!(page_name);
      },

    };
  

  }


}
