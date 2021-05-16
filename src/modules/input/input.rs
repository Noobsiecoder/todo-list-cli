use std::env;
use std::io::{self, Write};

// Public function
// Returns in String
pub fn _get_data(input_message: String) -> String {
    let mut input = String::new();
    print!("{}", input_message);
    io::stdout().flush().expect("Unable to flush stdout");
    io::stdin()
        .read_line(&mut input)
        .expect("Error reading input, please try later");
    input.replace("\r\n", "")
}

// Public function
// Gets arguments from user
pub fn _get_args() -> Vec<String> {
    env::args().skip(1).collect()
}
