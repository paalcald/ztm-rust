// Topic: User input
//
// Requirements:
// * Verify user input against pre-defined keywords
// * The keywords represent possible power options for a computer:
//   * Off
//   * Sleep
//   * Reboot
//   * Shutdown
//   * Hibernate
// * If the user enters one of the keywords, a message should be printed to
//   the console indicating which action will be taken
//   * Example: If the user types in "shutdown" a message should display such
//     as "shutting down"
// * If the keyword entered does not exist, an appropriate error message
//   should be displayed
//
// Notes:
// * Use an enum to store the possible power states
// * Use a function with a match expression to print out the power messages
//   * The function should accept the enum as an input
// * Use a match expression to convert the user input into the power state enum
// * The program should be case-insensitive (the user should be able to type
//   Reboot, reboot, REBOOT, etc.)
#[derive(Debug)]
#[allow(dead_code)]
enum Command {
    Off,
    Sleep,
    Reboot,
    Shutdown,
    Hibernate,
}
impl Command {
    fn from_str(keyword: String) -> Option<Self> {
        match keyword.to_lowercase().as_str() {
            "off" => Some(Command::Off),
            "sleep" => Some(Command::Sleep),
            "reboot" => Some(Command::Reboot),
            "shutdown" => Some(Command::Shutdown),
            "hibernate" => Some(Command::Hibernate),
            _ => None,
        }
    }
}
use std::io::{self, Write};
fn main() {
    print!("Input system command: ");
    io::stdout().flush().expect("could not flush.");
    let mut user_input = String::new();
    io::stdin().read_line(&mut user_input).expect("Could not read input.");
    user_input = user_input.trim().to_owned();
    Command::from_str(user_input)
        .map_or( println!("invalid power state."),|x| println!("{:?} initiated.", x));
}
