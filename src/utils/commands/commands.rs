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
    // Private function
    // Display version
    fn display_version(&self) -> String {
        String::from("0.0.3")
    }

    // Private function
    // Displays the user manual
    fn show_manual() {
        let task_commands = "📝 TASK COMMANDS:
        \t-a, add     \tTo add a new task
        \t-d, delete  \tTo delete an existing task
        \t-r, read    \tTo read all tasks available
        \t-u, update   \tTo update a task
        ";

        let flag_commands = "⛳ FLAG COMMANDS:
        \t--help      \tOutputs all the commands available
        \t--uninstall \tUninstall todo.exe
        \t--version   \tOutputs current CLI version
        ";

        println!("\x1b[1;36mHello World! This your assistant AI Taskio 🤖\x1b[0m");
        println!("\n💻 USAGE: \n\t\x1b[1;3;35mtodo [command | flag]\x1b[0m\n");
        println!("{}\n{}", task_commands, flag_commands);
    }

    // Public function
    // Matches with argument given and CLI Commands present
    pub fn cli_command(argument: &String) -> bool {
        match argument.as_str() {
            "-a" | "add" | "-d" | "delete" | "-r" | "read" | "-u" | "update" | "--uninstall" => {
                true
            }

            "--help" => {
                Commands::show_manual();
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
