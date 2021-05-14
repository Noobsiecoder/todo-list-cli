#[path = "../ui/color_texts.rs"]
mod font_styles;

pub enum Errors {
    SyntaxError,
}

impl Errors {
    // Public function
    // Throws an error
    pub fn throw_error(&self) {
        match *self {
            Errors::SyntaxError => font_styles::FontStyle::_error_message(
                &font_styles::FontStyle::Red,
                String::from("SyntaxError"),
                String::from("An error occured since command given doesn't exist"),
            ),
        }
    }
}
