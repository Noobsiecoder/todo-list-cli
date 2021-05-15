#[path = "../error/errors.rs"]
mod error;

use std::io::{self, Write};

//Public function
// Gets input from user and returns the value
pub fn user_input(input_message: String) -> String {
    let mut input = String::new();
    loop {
        print!("{}", input_message);
        io::stdout().flush().expect("Unable to flush stdout");
        io::stdin()
            .read_line(&mut input)
            .expect("Error reading input, please try later");
        if input.trim().is_empty() {
            error::Errors::_throw_error(&error::Errors::InputError);
        } else {
            break;
        }
    }
    input
}
