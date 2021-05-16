#[path = "./get_task_data.rs"]
// Task module
// Recieves inputted data and constructs as a struct Task
mod task_data;

#[path = "./crud/add.rs"]
// Add data middleware as a module
mod add;

#[path = "./crud/delete.rs"]
// Delete data middleware as a module
mod delete;

#[path = "./crud/read.rs"]
// Read data middleware as a module
mod read;

#[path = "./crud/update.rs"]
// Update data middleware as a module
mod update;

#[path = "./db.rs"]
// Database module
// Handles database's CRUD operation
mod database;

#[allow(dead_code)]
pub enum TaskCommands {
    Add,
    Delete,
    Read,
    Update,
}

#[allow(dead_code)]
pub enum MiscCommands {
    Help,
    Version,
}

impl TaskCommands {
    // Private function
    // Returns a boolean value indicating if task argument exists
    fn check_task_args(args: &String) -> bool {
        match args.as_str() {
            "-a" | "add" | "-d" | "delete" | "-r" | "read" | "-u" | "update" => true,
            _ => false,
        }
    }

    // Public function
    // Task command middleware
    pub fn task_command(&self) {
        match *self {
            TaskCommands::Add => add::add_task(),
            TaskCommands::Delete => delete::delete_task(),
            TaskCommands::Read => read::read_task(),
            TaskCommands::Update => update::update_task(),
        }
    }
}

impl MiscCommands {
    // Private function
    // Returns a boolean value indicating if miscellaneous argument exists
    fn check_misc_args(args: &String) -> bool {
        match args.replace("--", "").as_str() {
            "-h" | "help" | "-v" | "version" => true,
            _ => false,
        }
    }

    // Public function
    // Displays the version of the executable
    pub fn display_version() {
        println!("\x1b[1;36mHello World! This your assistant AI Taskio 🤖\x1b[0m");
        println!("Current version: 1.0.1");
    }

    // Public function
    // Displays the user manual
    pub fn show_manual() {
        let task_commands = "📝 TASK COMMANDS:
    \t-a, add     \tTo add a new task
    \t-d, delete  \tTo delete an existing task
    \t-r, read    \tTo read all tasks available
    \t-u, update   \tTo update a task
    ";

        let flag_commands = "⛳ FLAG COMMANDS:
    \t--help      \tOutputs all the commands available
    \t--version   \tOutputs current CLI version
    ";

        let code = "todo --help";

        println!("\x1b[1;36mHello World! This your assistant AI Taskio 🤖\x1b[0m");
        println!("\n💻 USAGE: \n\t\x1b[1;3;34mtodo [command | flag]\x1b[0m\n");

        println!("{}\n{}", task_commands, flag_commands);
        println!("❓ Try this : \x1b[1;3;35m{}\x1b[0m", code);
    }
}

// Public function
// Returns a boolean value indicating if argument exists
pub fn check_args_exists(argument: &String) -> (bool, u8) {
    if TaskCommands::check_task_args(argument) {
        return (true, 1);
    } else if MiscCommands::check_misc_args(argument) {
        return (true, 2);
    } else {
        return (false, 0);
    }
}

// Public function
// Returns a boolean value indicating if miscellaneous argument exists
pub fn check_args_len(args: &Vec<String>) -> bool {
    let vector_length = args.len();
    match vector_length {
        0 => false,
        _ => true,
    }
}
