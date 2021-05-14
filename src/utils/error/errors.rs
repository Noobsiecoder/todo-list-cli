#[path = "../ui/color_texts.rs"]
mod font_styles;

#[allow(dead_code)]
pub enum Errors {
    _SyntaxError,
    _InputError,
    _DatabaseBoolError,
}

impl Errors {
    // Public function
    // Throws an error
    pub fn _throw_error(&self) {
        match *self {
            Errors::_SyntaxError => {
                let code = "todo --help";
                font_styles::_error_message(
                    String::from("SyntaxError"),
                    String::from("An error occured since command given doesn't exist"),
                );
                println!("❓ Try this : \x1b[1;3;35m{}\x1b[0m", code);
            }

            Errors::_InputError => font_styles::_error_message(
                String::from("InputError"),
                String::from("An error occured since no input was given, please try again!"),
            ),

            Errors::_DatabaseBoolError => font_styles::_error_message(
                String::from("DatabaseBoolError"),
                String::from("An error occured since no boolean value in database was corrupted!"),
            ),
        }
    }
}
