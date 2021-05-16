pub enum MainErrors {
    ArgumentError,
    SyntaxError,
}

// Implementation of MainError enum
// Used for ArgumentError and SyntaxError messages
impl MainErrors {
    // Private Function
    // Displays the required error message
    fn display_error(error: String, info: String, can_suggest: bool) {
        println!("\x1b[1;36mOops! I tried my best, seems something went wrong 🤖\x1b[0m\n");
        println!("Error      : \x1b[1;31m{}\x1b[0m", error);
        println!("Error info : \x1b[2;31m{}\x1b[0m\n", info);
        if can_suggest {
            let code = "todo --help";
            println!("❓ Try this : \x1b[1;3;35m{}\x1b[0m", code);
        }
    }

    // Public Function
    // Matches with itself (Enum object)
    pub fn errors(&self) {
        match *self {
            MainErrors::ArgumentError => MainErrors::display_error(
                String::from("ArgumentError"),
                String::from("An error occured since no argument were given"),
                false,
            ),
            MainErrors::SyntaxError => MainErrors::display_error(
                String::from("SyntaxError"),
                String::from("An error occured since the syntax is incorrect"),
                true,
            ),
        }
    }
}
