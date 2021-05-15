use ansi_term;

#[path = "./utils/io/args.rs"]
mod args;

#[path = "./utils/error/errors.rs"]
mod error;

#[path = "./utils/warnings/warnings.rs"]
mod warning;

#[path = "./utils/commands/commands.rs"]
mod command;

#[path = "./utils/ui/table.rs"]
mod table;

fn main() {
    // Enable ansi support for windows 10
    #[cfg(target_os = "windows")]
    let enabled = ansi_term::enable_ansi_support();
    match enabled {
        Ok(_) => (),
        Err(err) => println!("{}", err),
    }
    let user_args = args::get_args();
    match user_args.len() {
        0 => warning::Warnings::throw_warning(&warning::Warnings::NoArgsWarning), // throw warning
        _ => {
            let command_exists: bool = command::Commands::cli_command(&user_args[0]);
            if !command_exists {
                error::Errors::_throw_error(&error::Errors::SyntaxError); // throw syntax error
            }
        }
    }
}
