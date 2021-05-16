#[allow(unused_imports)]
use ansi_term;

#[path = "./modules/input/input.rs"]
// Input module
// Gets user arguments inputted in cli
mod input;

#[path = "./modules/middlewares/args.rs"]
// Arguments module
// Checks if arguments exists
// Takes care of calling respective middlewares
mod arguments;

#[path = "./modules/output/main_errors.rs"]
// Main error module
// Contains = SyntaxError and ArgumentError
mod main_errors;

// Handles Task argument middlewares
fn task_middleware(argument: &String) {
    match argument.as_str() {
        "-a" | "add" => arguments::TaskCommands::task_command(&arguments::TaskCommands::Add),
        "-d" | "delete" => arguments::TaskCommands::task_command(&arguments::TaskCommands::Delete),
        "-r" | "read" => arguments::TaskCommands::task_command(&arguments::TaskCommands::Read),
        "-u" | "update" => arguments::TaskCommands::task_command(&arguments::TaskCommands::Update),
        _ => (),
    }
}

// Handles Miscellaneous argument middlewares
fn misc_middleware(argument: &String) {
    match argument.replace("--", "").as_str() {
        "-h" | "help" => arguments::MiscCommands::show_manual(),
        "-v" | "version" => arguments::MiscCommands::display_version(),
        _ => (),
    }
}

fn main() {
    // Enable ansi support for windows 10
    #[cfg(target_os = "windows")]
    {
        let enabled = ansi_term::enable_ansi_support();
        match enabled {
            Ok(_) => (),
            Err(err) => eprintln!("{}", err),
        }
    }

    let cli_args = input::_get_args();

    let is_args_given = arguments::check_args_len(&cli_args);
    if is_args_given {
        let argument = &cli_args[0];
        let does_args_exists = arguments::check_args_exists(argument);

        match does_args_exists {
            (true, 1) => task_middleware(argument), // 1 -> Task middleware
            (true, 2) => misc_middleware(argument), // 2 -> Miscellaneous middleware
            (false, 0) => main_errors::MainErrors::errors(&main_errors::MainErrors::SyntaxError), // 0 -> Syntax error
            _ => (), // No other meaningful value that could be returned
        }
    } else {
        main_errors::MainErrors::errors(&main_errors::MainErrors::ArgumentError);
    }
}
