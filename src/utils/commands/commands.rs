#[path = "./manual.rs"]
mod manual;

#[allow(dead_code)]
pub enum Commands {
    // Task commands
    Add,
    Delete,
    Edit,
    Read,

    // Miscellaneous commands
    Help,
    Version,
    Uninstall,
}

impl Commands {
    // Function display version
    fn display_version(&self) -> String {
        String::from("0.0.3")
    }

    // Function for CLI Commands
    pub fn cli_command(argument: &String) -> bool {
        match argument.as_str() {
            "-a" | "add" | "-d" | "delete" | "-r" | "read" | "-u" | "update" | "--uninstall" => {
                true
            }

            "--help" => {
                manual::show_manual();
                true
            }

            "--version" => {
                let exe_version = Commands::display_version(&Commands::Version);
                println!("\x1b[1;36mHello World! This your assistant AI Taskio 🤖\x1b[0m");
                println!("\nTodo version: {}", exe_version);
                true
            }
            _ => false, // If doesn't exists, return false
        }
    }
}
