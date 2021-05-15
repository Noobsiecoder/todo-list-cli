#[path = "../ui/color_texts.rs"]
mod font_styles;

#[allow(dead_code)]
pub enum Errors {
    SyntaxError,
    InputError,
    DatabaseAddError,
    DatabaseReadError,
    DatabaseUpdateError,
    DatabaseDeleteError,
    DatabaseBoolError,
    TaskIdError,
    TaskTypeError,
}

impl Errors {
    // Public function
    // Throws an error
    pub fn _throw_error(&self) {
        match *self {
            Errors::SyntaxError => {
                let code = "todo --help";
                font_styles::_error_message(
                    String::from("SyntaxError"),
                    String::from("An error occured since command given doesn't exist"),
                );
                println!("❓ Try this : \x1b[1;3;35m{}\x1b[0m", code);
            }

            Errors::InputError => font_styles::_error_message(
                String::from("InputError"),
                String::from("An error occured since no input was given, please try again!"),
            ),

            Errors::DatabaseAddError => font_styles::_error_message(
                String::from("DatabaseAddError"),
                String::from("An error occured while trying to add data in database!"),
            ),

            Errors::DatabaseReadError => font_styles::_error_message(
                String::from("DatabaseReadError"),
                String::from("An error occured while trying to read data from database!"),
            ),

            Errors::DatabaseUpdateError => font_styles::_error_message(
                String::from("DatabaseUpdateError"),
                String::from("An error occured while trying to update data from database!"),
            ),

            Errors::DatabaseDeleteError => font_styles::_error_message(
                String::from("DatabaseDeleteError"),
                String::from("An error occured while trying to delete data from database!"),
            ),

            Errors::DatabaseBoolError => font_styles::_error_message(
                String::from("DatabaseBoolError"),
                String::from("An error occured since no boolean value in database was corrupted!"),
            ),

            Errors::TaskIdError => font_styles::_error_message(
                String::from("TaskIdError"),
                String::from("An error occured since the task id entered is either incorrect or doesn't exist in database, please try again later!"),
            ),

            Errors::TaskTypeError => font_styles::_error_message(
                String::from("TaskTypeError"),
                String::from("An error occured since the task type entered is incorrect, please try again later!"),
            ),
        }
    }
}
