use crossterm::{
  style::{Color, ResetColor, SetForegroundColor},
  ExecutableCommand,
};
use std::io::{stdin, stdout};


// Get user request
pub fn get_user_response(question: &str) -> String {
  let mut stdout: std::io::Stdout = stdout();

  // Print the question in a specific color
  stdout
      .execute(SetForegroundColor(Color::Blue))
      .unwrap();
  println!("");
  println!("{}", question);

  // Reset color
  stdout
      .execute(ResetColor)
      .unwrap();

  // Read user input
  let mut user_response: String = String::new();
  stdin()
      .read_line(&mut user_response)
      .expect("Failed to read response");

  // Trim whitespace and return
  return user_response.trim().to_string();
}


// Get user request
pub fn request_database_preference() -> Option<String> {
  let mut stdout: std::io::Stdout = stdout();

  loop {

    // Print the question in a specific color
    stdout
        .execute(SetForegroundColor(Color::Blue))
        .unwrap();
    println!("");
    println!("Do you have a preferred database solution?");

    // Reset color
    stdout
        .execute(ResetColor)
        .unwrap();

    // Present options with different colors
    stdout
        .execute(SetForegroundColor(Color::DarkMagenta))
        .unwrap();
    println!("[1] JSON ");
    stdout
        .execute(SetForegroundColor(Color::Cyan))
        .unwrap();
    println!("[2] No ");

    // Reset color
    stdout
        .execute(ResetColor)
        .unwrap();

    // Read user input
    let mut human_response: String = String::new();
    stdin()
        .read_line(&mut human_response)
        .expect("Failed to read response");

    // Trim whitespace and convert to lowercase for case-insensitive comparison
    let human_response = human_response.trim().to_lowercase();

    // Match response
    match human_response.as_str() {
      "1" | "json" | "1\n" => {
        return Some("JSON".to_string());
      }
      "2" | "no" | "2\n" => {
        return None;
      }
      _ => {
          println!("Invalid input. Please enter '1' or '2'.");
      }
    }
  }
}
