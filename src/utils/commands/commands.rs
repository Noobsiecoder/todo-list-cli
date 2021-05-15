#[path = "../error/errors.rs"]
mod error;

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

enum TaskType {
    Name,
    Note,
    StartDate,
    StartTime,
    EndDate,
    EndTime,
    Important,
    Completed,
}

trait GetTaskData {
    fn get_data(task_type: &TaskType) -> Self;
}

// Implementation of GetTaskData in String data type
impl GetTaskData for String {
    // Private function
    // Gets data in String type and returns in String type
    fn get_data(task_type: &TaskType) -> String {
        match task_type {
            TaskType::Name => Commands::trim_whitespaces(input::user_input(String::from(
                "\x1b[32m?\x1b[0m 🔖 Task Name                  : ",
            ))),
            TaskType::Note => Commands::trim_whitespaces(input::user_input(String::from(
                "\x1b[32m?\x1b[0m 📝 Task Note                  : ",
            ))),
            TaskType::StartDate => Commands::trim_whitespaces(input::user_input(String::from(
                "\x1b[32m?\x1b[0m 📅 Start Date [dd-mm-yy]      : ",
            ))),
            TaskType::StartTime => Commands::trim_whitespaces(input::user_input(String::from(
                "\x1b[32m?\x1b[0m ⌚ Start time [hh:mm (AM/PM)] : ",
            ))),
            TaskType::EndDate => Commands::trim_whitespaces(input::user_input(String::from(
                "\x1b[32m?\x1b[0m 📅 End Date   [dd-mm-yy]      : ",
            ))),
            TaskType::EndTime => Commands::trim_whitespaces(input::user_input(String::from(
                "\x1b[32m?\x1b[0m ⌚ End time   [hh:mm (AM/PM)] : ",
            ))),
            _ => panic!("[IGNORE MESSAGE] Shouldn't come here!"),
        }
    }
}

// Implementation of GetTaskData in i8 (Range: -128 to 127 || -2^8 to ((2^8) - 1)) data type
impl GetTaskData for i8 {
    // Private function
    // Gets data in String type and returns in i8 type
    fn get_data(task_type: &TaskType) -> i8 {
        match task_type {
            TaskType::Important => match input::user_input(String::from(
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
            },
            TaskType::Completed => match input::user_input(String::from(
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
            },
            _ => panic!("[IGNORE MESSAGE] Shouldn't come here!"),
        }
    }
}

impl Commands {
    // Private function
    // Displays version
    fn display_version(&self) -> String {
        String::from("1.0.0")
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
        \t--version   \tOutputs current CLI version
        ";

        println!("\x1b[1;36mHello World! This your assistant AI Taskio 🤖\x1b[0m");
        println!("\n💻 USAGE: \n\t\x1b[1;3;34mtodo [command | flag]\x1b[0m\n");
        println!("{}\n{}", task_commands, flag_commands);
    }

    // Private function
    // Trims whitespaces by replacing "/r/n" in a String data
    fn trim_whitespaces(value: String) -> String {
        value.replace("\r\n", "")
    }

    // Private function
    // Gets required user input and processes the data to add a new task_commands
    fn add_task() {
        println!("\x1b[1;36mHello World! This your assistant AI Taskio 🤖\x1b[0m");
        println!("\nPlease enter all the details properly:- \n");

        let task_name = GetTaskData::get_data(&TaskType::Name);
        let task_note = GetTaskData::get_data(&TaskType::Note);
        let start_date = GetTaskData::get_data(&TaskType::StartDate);
        let start_time = GetTaskData::get_data(&TaskType::StartTime);
        let end_date = GetTaskData::get_data(&TaskType::EndDate);
        let end_time = GetTaskData::get_data(&TaskType::EndTime);
        let is_important = GetTaskData::get_data(&TaskType::Important);
        let is_completed = GetTaskData::get_data(&TaskType::Completed);

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

    // Private function
    // Reads tasks present in database and displays in a table
    fn read_task() {
        println!("\x1b[1;36mHello World! This your assistant AI Taskio 🤖\x1b[0m");
        table::_diplay_data_in_table_cells();
    }

    // Private function
    // Deletes task corrsponding to the task id
    // If task id not found, return TaskIdError
    fn delete_task() {
        println!("\x1b[1;36mHello World! This your assistant AI Taskio 🤖\x1b[0m");
        let task_id = Commands::trim_whitespaces(input::user_input(String::from(
            "\x1b[32m?\x1b[0m 🆔 Task ID : ",
        )));
        match task_id.parse::<u8>() {
            Ok(task_id) => db::Task::_delete(&task_id),
            Err(_err) => error::Errors::_throw_error(&error::Errors::TaskIdError),
        };
    }

    // Private function
    // Updates task corrsponding to the task id, task type and data given
    // If task type not valid, return TaskTypeError
    // If task id not found, return TaskIdError
    fn update_task() {
        println!("\x1b[1;36mHello World! This your assistant AI Taskio 🤖\x1b[0m");
        let task_id = Commands::trim_whitespaces(input::user_input(String::from(
            "\x1b[32m?\x1b[0m 🆔 Task ID : ",
        )));
        match task_id.parse::<u8>() {
            Ok(task_id) => {
                println!("\nThese are the task names :-");
                println!("tnm\tTask Name");
                println!("tne\tTask Note");
                println!("sdt\tStart Date");
                println!("stm\tStart Time");
                println!("edt\tEnd Date");
                println!("etm\tEnd Time");
                println!("imp\tImportant");
                println!("com\tImportant");
                let task_type = Commands::trim_whitespaces(input::user_input(String::from(
                    "\x1b[32m?\x1b[0m 💾 Enter Task type : ",
                )));
                match task_type.as_str() {
                    "tnm" => {
                        let task_name: String = GetTaskData::get_data(&TaskType::Name);
                        db::Task::_update_data_in_db(&task_id, "TASK_NAME", &task_name);
                    }
                    "tne" => {
                        let task_note: String = GetTaskData::get_data(&TaskType::Note);
                        db::Task::_update_data_in_db(&task_id, "TASK_NOTE", &task_note);
                    }
                    "sdt" => {
                        let start_date: String = GetTaskData::get_data(&TaskType::StartDate);
                        db::Task::_update_data_in_db(&task_id, "START_DATE", &start_date);
                    }
                    "stm" => {
                        let start_time: String = GetTaskData::get_data(&TaskType::StartDate);
                        db::Task::_update_data_in_db(&task_id, "START_TIME", &start_time);
                    }
                    "edt" => {
                        let end_date: String = GetTaskData::get_data(&TaskType::EndDate);
                        db::Task::_update_data_in_db(&task_id, "END_DATE", &end_date);
                    }
                    "etm" => {
                        let end_time: String = GetTaskData::get_data(&TaskType::EndTime);
                        db::Task::_update_data_in_db(&task_id, "END_TIME", &end_time);
                    }
                    "imp" => {
                        let important: i8 = GetTaskData::get_data(&TaskType::Important);
                        db::Task::_update_data_in_db(&task_id, "IMPORTANT", &important.to_string());
                    }
                    "com" => {
                        let completed: i8 = GetTaskData::get_data(&TaskType::Completed);
                        db::Task::_update_data_in_db(&task_id, "COMPLETED", &completed.to_string());
                    }
                    _ => error::Errors::_throw_error(&error::Errors::TaskTypeError),
                }
            }
            Err(_err) => error::Errors::_throw_error(&error::Errors::TaskIdError),
        };
    }

    // Public function
    // Matches with argument given and CLI Commands present
    pub fn cli_command(argument: &String) -> bool {
        match argument.as_str() {
            "-a" | "add" => {
                Commands::add_task();
                true
            }

            "-r" | "read" => {
                Commands::read_task();
                true
            }

            "-d" | "delete" => {
                Commands::delete_task();
                true
            }

            "-u" | "update" => {
                Commands::update_task();
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
