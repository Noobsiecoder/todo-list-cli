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
            Warnings::NoArgsWarning => {
                let code = "todo --help";
                font_styles::_warning_message(
                    String::from("NoArgsWarning"),
                    String::from("No arguments were passed."),
                );
                println!("❓ Try this : \x1b[1;3;35m{}\x1b[0m", code);
            }
        }
    }
}
