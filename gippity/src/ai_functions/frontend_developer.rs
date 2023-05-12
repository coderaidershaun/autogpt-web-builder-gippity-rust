use proc_macro::function_to_string;


// Fix buggy component code
#[function_to_string]
pub fn print_code_bugs_resolution(_existing_code_and_error: &str) {
  /// INPUT: Takes in existing code causing build failres along with errors caused by the code
  /// FUNCTION: Writes the new and imporved React typescript component code with bugs fixed
  /// NOTE: Functions considers that the code is made of React Typescript.
  /// Therefore, it is allowed to use //@ts-ignore if that is the appropriate solution rather than guessing the solution
  /// IMPORTANT: This function only prints a full react component with completed typescript code, nothing else.
  println!(OUTPUT)
}


// Page Architecture
#[function_to_string]
pub fn print_recommended_site_pages(_project_description_and_backend_code_logic: &str) {
  /// INPUT: Takes in a PROJECT_DESCRIPTION and CODE_LOGIC for a websites backend. This function interprets a page structure solution for the frontend
  /// FUNCTION: Outputs up to 2 recommended pages for an SPA application that would BEST suit the PROJECT_DESCRIPTION and CODE_LOGIC
  /// IMPORTANT: 
  ///   1. The "suggested_content_sections" do not mention headers or footers as these are already covered
  ///   2. The keys within the "suggested_content_sections" object can vary based on the functions imagination
  ///   3. The "page_name" and "suggested_content_sections" keys are fixed and cannot change. Every object must have one
  /// FORMAT: Prints a JSON object array
  /// OUTPUT EXAMPLE 1 of a Health information related site: [
  ///   {
  ///     "page_name": "home_page",
  ///     "suggested_content_sections": {
  ///       "banner_section": "A glorious and colourful title and subtitle capturing the page visitors attention in bold writing",
  ///       "ratings_section": "A sliding carousell of customer ratings and feedback",
  ///       "facts_section": "3 short paragraphs talking about the benefits of looking after your health",
  ///       ...
  ///     },
  ///      "page_name": "about_page",
  ///      "suggested_content_sections": {
  ///       "title_section": "About us title with a catchy tagline underneath",
  ///       "about_body_section": "A medium sized article about who we are and what we do",
  ///       ...
  ///     },
  ///   ...
  ///   }
  /// ]
  /// OUTPUT EXAMPLE 2 of a Task Tracking site related site: [
  ///   {
  ///     "page_name": "dashboard",
  ///     "suggested_content_sections": {
  ///       "tasks_tracked_section": "Listing of all the users todo tasks completed if logged in",
  ///       "tasks_tracked_section": "Signup Hero section if NOT logged in",
  ///       ...
  ///     },
  ///      "page_name": "create_tasks",
  ///      "suggested_content_sections": {
  ///       "input_section": "Input section with a button for writing a task and submitting it
  ///       A delete icon should also be next to the task for removing tasks",
  ///       ...
  ///     },
  ///     ...
  ///   }
  /// ]
  /// IMPORTANT: This function only prints the JSON output, nothing else. This function makes sure the JSON is valid.
  println!(OUTPUT)
}


// Page Architecture
#[function_to_string]
pub fn print_recommended_site_pages_with_apis(_website_specification: &str) {
  /// INPUT: Takes in a PROJECT_DESCRIPTION, and WEBSITE_SPECIFICATION with API_ENDPOINTS and PAGES for a websites frontend build.
  /// FUNCTION: Assignes endpoints to each page
  /// OUTPUT FORMAT: The function outputs in a JSON format as follows:
  ///   [
  ///     "page_name": [
  ///       {
  ///         "api_route": String, // the route with a /route format
  ///         "method": String, // get, post, put, delete
  ///         "route_type": String // internal or external
  ///       },
  ///       ...
  ///     ],
  ///     ...
  ///   ]
  /// EXAMPLE:
  ///   [
  ///     "todo_dashboard": [
  ///       { 
  ///         "api_route": "/task",
  ///         "method": "get",
  ///         "route_type": "internal"
  ///       },
  ///       {
  ///         "api_route": "https://myforexprices.com/prices?symbol=ABC",
  ///         "method": "get",
  ///         "route_type": "external"
  ///       },
  ///     "user": [
  ///       {
  ///         "api_route": "/signin",
  ///         "method": "post",
  ///         "route_type": "internal"
  ///       },
  ///       {
  ///         "api_route": "/register",
  ///         "method": "post",
  ///         "route_type": "internal"
  ///       },
  ///      "contact": [] // notice how contact is blank. No APIs need to be assigned here
  ///     ],
  ///     ...
  ///   ]
  /// REMEMBER: Not all pages need routes. Also, ALL API ROUTES MUST BE ACCOUNTED FOR
  /// IMPORTANT: This function only prints a JSON response, nothing else. Just JSON.
  println!(OUTPUT)
}


// Branding - Colour
#[function_to_string]
pub fn print_recommended_site_main_colours(_website_content: &str) {
  /// INPUT: Takes in a PROJECT_DESCRIPTION and WEBSITE_CONTENT for a frontend website project
  /// FUNCTION: Outputs up to 3 recommended colours that would BEST suit the PROJECT_DESCRIPTION and nature of the website
  /// FORMAT: Prints just a list of strings based on Tailwind css colours, here are some examples:
  /// OUTPUT EXAMPLE: ["##32a852", "#0fa0d1", "#d10fcb"]
  /// NOTE: This function is excellent at design and prints professionally considered colours only in the above format
  /// IMPORTANT: This function only prints a list of strings, nothing else
  println!(OUTPUT)
}


// Branding - Logo inner
#[function_to_string]
pub fn print_svg_logo(_project_description_and_colours: &str) {
  /// INPUT: Takes in a PROJECT_DESCRIPTION and BRAND_COLOURS for a websites frontend
  /// FUNCTION: Creates a basic simple and flat looking LOGO via SVG JSX to best suit the website. Just an icon with no writing
  /// THIS IS AN EXAMPLE OF THE OUTPUT:
  ///   <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" strokeWidth={1.5} stroke="currentColor" className="w-6 h-6 text-[#0fa0d1]">
  ///     <path strokeLinecap="round" strokeLinejoin="round" d="M4.26 10.147a60.436 60.436 0 00-.491 6.347A48.627 48.627 0 0112 20.904a48.627 48.627 0 018.232-4.41 60.46 60.46 0 00-.491-6.347m-15.482 0a50.57 50.57 0 00-2.658-.813A59.905 59.905 0 0112 3.493a59.902 59.902 0 0110.399 5.84c-.896.248-1.783.52-2.658.814m-15.482 0A50.697 50.697 0 0112 13.489a50.702 50.702 0 017.74-3.342M6.75 15a.75.75 0 100-1.5.75.75 0 000 1.5zm0 0v-3.675A55.378 55.378 0 0112 8.443m-7.007 11.55A5.981 5.981 0 006.75 15.75v-1.5" />
  ///   </svg>
  /// IMPORTANT: Tailwind CSS is used for styling. You will not this in the example above where #0fa0d1 as an example colour. Any colour used should be selected from the brand colours provided.
  /// REMEMBER: This function only prints the <svg... code in the format shown above. Nothing else. Just the code.
  println!(OUTPUT)
}


// Branding - Logo component
#[function_to_string]
pub fn print_completed_logo_with_brand_name_react_component(_website_specification: &str) {
  /// INPUT: Takes in a WEBSITE_SPECIFICATION for a websites frontend.
  /// FUNCTION: Takes the logo from the SVG provided and adds a catchy PascalCase app name.
  /// NOTE: The Logo is based on the WEBSITE_SPECIFICATION information.
  /// FORMAT: It is important that the company name and logo are centered well with flex tailwind css containers
  ///   1.  A logo icon can be to the left and company name vertically centered to its right
  ///   2.  A logo icon can be centered in the middle and company name centered horizontally beneath of overlaying it
  ///   3.  The function chooses whichever option it decides will be clearer for website visitors
  /// TEMPLATE:
  /// function Logo() {
  ///   return (
  ///     <div>
  ///       YOUR COMPONENT CODE GOES HERE
  ///     </div>
  ///   )
  /// }
  /// export default Logo
  /// IMPORTANT: Tailwind CSS is used for styling. Does NOT use any extrernal libraries not included in this list: [axios, @mui/icons-material", react, tailwind]
  /// REMEMBER: This function only prints React Typescript component code. Nothing else. Just the code.
  println!(OUTPUT)
}


// Navigation - Header nav bar
#[function_to_string]
pub fn print_header_navigation_react_component(_website_specification: &str) {
  /// INPUT: Takes in a WEBSITE_SPECIFICATION for a websites frontend.
  /// FUNCTION: Writes the code for a REACT TYPESCRIPT navigation header bar for a frontend website
  /// OUTPOUT: The navigation header bar includes the following
  ///   1 - The existing logo which shows as <Logo />, the logo should have an 'import Logo from "./Logo"' at the top of the component
  ///   2 - Page links based on the pages provided in the specification
  ///   3 - This should be responsive based upon the size of the screen using tailwind css. A small screen should have a burger menu with slider
  ///   4 - Takes in a getter and setter prop called currentPage and setCurrentPage respectively
  ///   5 - Depending on the page, a different color will show on the navigation links
  /// TEMPLATE:
  /// import Logo from "./Logo"
  /// type Props = {
  ///   currentPage: string;
  ///   setCurrentPage: any; // Leave this as any as an unknown setter function will be passed here
  /// }
  /// function Navigation({setCurrentPage, currentPage}: Props) {
  ///   return (
  ///     <div>
  ///       YOUR COMPONENT CODE GOES HERE
  ///     </div>
  ///   )
  /// }
  /// export default Navigation
  /// DO NOT LEAVE ANY CODE UNFINISHED FOR LATER. CODE EVERYTHING INCLUDING THE SLIDER MENU NOW.
  /// IMPORTANT: This function only prints a full react component with completed typescript code, nothing else.
  /// IMPORTANT: Tailwind CSS is used for styling. Does NOT use any extrernal libraries not included in this list: [axios, @mui/icons-material", react, tailwind]
  println!(OUTPUT)
}


// Navigation - Footer nav bar
#[function_to_string]
pub fn print_footer_navigation_react_component(_website_specification: &str) {
  /// INPUT: Takes in a WEBSITE_SPECIFICATION for a websites frontend.
  /// FUNCTION: Writes only the HTML code for a REACT TYPESCRIPT footer for website
  /// OUTPOUT: The navigation footer bar includes the following
  ///   1 - Page links based on the pages provided in the specification
  ///   2 - Must be responsive and be a small fixed bar to the bottom of the screen if in mobile view
  ///   3 - Takes in a getter and setter prop called currentPage and setCurrentPage respectively
  ///   4 - Depending on the page, a different color will show on the navigation links
  /// TEMPLATE:
  /// type Props = {
  ///   currentPage: string;
  ///   setCurrentPage: any; // Leave this as any as an unknown setter function will be passed here
  /// }
  /// function Footer({setCurrentPage, currentPage}: Props) {
  ///   return (
  ///     <div>
  ///       YOUR COMPONENT CODE GOES HERE
  ///     </div>
  ///   )
  /// }
  /// export default Footer
  /// IMPORTANT: This function only prints a full react component with completed typescript code, nothing else.
  /// IMPORTANT: Tailwind CSS is used for styling. Does NOT use any extrernal libraries not included in this list: [axios, @mui/icons-material", react, tailwind]
  println!(OUTPUT)
}


// Integration - React Hooks
#[function_to_string]
pub fn print_react_typescript_hook_component(_api_endpoints: &str) {
  /// INPUT: Takes in a list of API_ENDPOINTS_JSON_SCHEMA and with their request and response schema. All these endpoints are called from http://localhost:8080
  /// OUTPUT: A full REACT "useCall" TYPESCRIPT CUSTOM REACT HOOK component connecting to and returning data for ALL of the endpoints. No endpoints are left out
  /// NOTE: All code is fully written and interfaces made available for decoding any returned data
  /// COMPONENT TITLE: The components title is "useCall"
  /// IMPORTANT: The component is fully working with typescript annotations types declared for everything or //@ts-ignore if unsure
  /// IMPORTANT: Does NOT use any extrernal libraries not included in this list: [axios, @mui/icons-material", react, tailwind]
  /// FORMAT: Just prints the react typescript component, Nothing else. All API endpoints are called from http://localhost:8080
  /// ERROR HANDLING: All error handling includes "as any" to prevent build errors on type issues:
  ///  catch (e) {
  ///   setError(e as any);
  ///   ...
  ///  }
  println!(OUTPUT)
}


// Integration - Content and wireframing
#[function_to_string]
pub fn create_frontend_website_page_content(_page_content_spec: &str) {
  /// INPUT: Receives page description and high level spec along with api data information that the page will receive
  /// OUTPUT: Writes HTML code only with content based on spec
  /// RULES: 
  ///   1. Provides a lot of content, is not afraid to provide expert level wireframing
  ///   2. This is JUST the html with each tag having an id, no classes
  ///   3. Again: No classes will be added to tags. This is just the html structure with ids
  ///   4. The id will describe what html tag does
  /// 
  println!(OUTPUT)
}


