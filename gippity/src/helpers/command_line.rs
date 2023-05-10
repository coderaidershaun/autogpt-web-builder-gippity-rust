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


// User response on safe code
pub fn confirm_safe_code() -> bool {
  let mut stdout: std::io::Stdout = stdout();
  loop {

    // Print the question in a specific color
    stdout
        .execute(SetForegroundColor(Color::Blue))
        .unwrap();
    println!("");
    print!("You are about to run code written entirely by AI. ");
    println!("Review the code and confirm your view:");

    // Reset color
    stdout
        .execute(ResetColor)
        .unwrap();

    // Present options with different colors
    stdout
        .execute(SetForegroundColor(Color::Green))
        .unwrap();
    println!("[1] All good, nothing alarming going on here ");
    stdout
        .execute(SetForegroundColor(Color::DarkRed))
        .unwrap();
    println!("[2] Uh oh, better stop this project ");

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
    let human_response: String = human_response.trim().to_lowercase();

    // Match response
    match human_response.as_str() {
      "1" | "ok" | "y" => return true,
      "2" | "no" | "n" => return false,
      _ => {
          println!("Invalid input. Please enter '1' or '2'.");
      }
    }
  }
}
