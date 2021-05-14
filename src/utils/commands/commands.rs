#[path = "../io/input.rs"]
mod input;

#[path = "../db/db.rs"]
mod db;

#[path = "../ui/table.rs"]
mod table;

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
        println!("\n💻 USAGE: \n\t\x1b[1;3;34mtodo [command | flag]\x1b[0m\n");
        println!("{}\n{}", task_commands, flag_commands);
    }

    fn trim_whitespaces(value: String) -> String {
        value.replace("\r\n", "")
    }

    // Private function
    // Gets required user input and processes the data to add a new task_commands
    fn add_task() {
        println!("\x1b[1;36mHello World! This your assistant AI Taskio 🤖\x1b[0m");
        println!("\nPlease enter all the details properly:- \n");
        let task_name = Commands::trim_whitespaces(input::user_input(String::from(
            "\x1b[32m?\x1b[0m 🔖 Task Name                  : ",
        )));
        let task_note = Commands::trim_whitespaces(input::user_input(String::from(
            "\x1b[32m?\x1b[0m 📝 Task Note                  : ",
        )));
        let start_date = Commands::trim_whitespaces(input::user_input(String::from(
            "\x1b[32m?\x1b[0m 📅 Start Date [dd-mm-yy]      : ",
        )));
        let start_time = Commands::trim_whitespaces(input::user_input(String::from(
            "\x1b[32m?\x1b[0m ⌚ Start time [hh:mm (AM/PM)] : ",
        )));
        let end_date = Commands::trim_whitespaces(input::user_input(String::from(
            "\x1b[32m?\x1b[0m 📅 End Date   [dd-mm-yy]      : ",
        )));
        let end_time = Commands::trim_whitespaces(input::user_input(String::from(
            "\x1b[32m?\x1b[0m ⌚ End time   [hh:mm (AM/PM)] : ",
        )));

        let is_important: i8 = match input::user_input(String::from(
            "\x1b[32m?\x1b[0m 🌟 Important  [true/false]    : ",
        ))
        .trim()
        .parse::<bool>()
        {
            Ok(is_important) => {
                if is_important {
                    1
                } else {
                    0
                }
            }
            Err(err) => {
                println!("{}, hence taking value as `false`", err);
                0
            }
        };

        let is_completed: i8 = match input::user_input(String::from(
            "\x1b[32m?\x1b[0m ✅ Completed  [true/false]    : ",
        ))
        .trim()
        .parse::<bool>()
        {
            Ok(is_completed) => {
                if is_completed {
                    1
                } else {
                    0
                }
            }
            Err(err) => {
                println!("{}, hence taking value as `false`", err);
                0
            }
        };

        let task = db::Task {
            // Task name and note
            task_name,
            task_note,

            // Date
            start_date,
            end_date,

            // Time
            start_time,
            end_time,

            // Task priority and status
            is_important,
            is_completed,
        };

        db::Task::_add_data_to_db(&task);
    }

    // Function reads tasks present in database and displays in a table
    fn read_task() {
        table::_diplay_data_in_table_cells();
    }

    // Public function
    // Matches with argument given and CLI Commands present
    pub fn cli_command(argument: &String) -> bool {
        match argument.as_str() {
            "-d" | "delete" | "-u" | "update" | "--uninstall" => true,

            "-a" | "add" => {
                Commands::add_task();
                true
            }

            "-r" | "read" => {
                Commands::read_task();
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
