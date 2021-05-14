#[path = "../ui/color_texts.rs"]
mod font_styles;

pub enum Warnings {
    NoArgsWarning,
}

impl Warnings {
    // Public function 
    // Throws a warning
    pub fn throw_warning(&self) {
        match *self {
            Warnings::NoArgsWarning => font_styles::FontStyle::_warning_message(
                &font_styles::FontStyle::Orange,
                String::from("NoArgsWarning"),
                String::from("No arguments were passed."),
            ),
        }
    }
}
