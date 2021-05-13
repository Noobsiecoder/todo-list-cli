use std::io::{self, Write};

// Function which gets input from user and returns the value
pub fn user_input() -> String {
    let mut input = String::new();
    io::stdout().flush().expect("Unable to flush stdout");
    io::stdin()
        .read_line(&mut input)
        .expect("Error reading input, please try later");
    input
}
