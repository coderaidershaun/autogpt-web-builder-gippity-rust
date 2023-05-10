mod ai_functions;
mod apis;
mod helpers;
mod models;

use helpers::command_line::get_user_response;
use models::agents_manager::managing_agent::ManagingAgent;


struct TestStruct {
    name: String,
    age: u8
}


#[tokio::main]
async fn main() {

    // Obtain user goal
    let usr_req: String = get_user_response("What are we building today?");

    // Create Gippity Managing Agent
    let managing_agent: ManagingAgent = ManagingAgent::new(usr_req).await.expect("Error creating agent");

    dbg!(managing_agent);

}