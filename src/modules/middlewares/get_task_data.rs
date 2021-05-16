#[path = "../input/input.rs"]
// Input module
// Gets user input in String format
// Is used while handling CRUD operation
mod input;

#[allow(dead_code)]
pub enum TaskType {
    // Task related enum
    TaskType,
    TaskId,

    // Name
    Name,

    // Date
    StartDate,
    EndDate,

    // Time
    StartTime,
    EndTime,

    // Status
    Important,
    Completed,
}

pub trait GetTaskData {
    // Implements String and u8
    // Function gets task data
    fn get_task_data(task_type: &TaskType) -> Self;
}

// Implementation of GetTaskData in String data type
impl GetTaskData for String {
    // Private function
    // Gets data in String type and returns in String type
    fn get_task_data(task_type: &TaskType) -> String {
        match task_type {
            TaskType::TaskType => input::_get_data(String::from(
                "\x1b[32m?\x1b[0m 💾 Enter Task type            : ",
            )),
            TaskType::Name => input::_get_data(String::from(
                "\x1b[32m?\x1b[0m 🔖 Task Name                  : ",
            )),
            TaskType::StartDate => input::_get_data(String::from(
                "\x1b[32m?\x1b[0m 📅 Start Date [dd-mm-yy]      : ",
            )),
            TaskType::StartTime => input::_get_data(String::from(
                "\x1b[32m?\x1b[0m ⌚ Start time [hh:mm (AM/PM)] : ",
            )),
            TaskType::EndDate => input::_get_data(String::from(
                "\x1b[32m?\x1b[0m 📅 End Date   [dd-mm-yy]      : ",
            )),
            TaskType::EndTime => input::_get_data(String::from(
                "\x1b[32m?\x1b[0m ⌚ End time   [hh:mm (AM/PM)] : ",
            )),
            _ => String::from("[IGNORE MESSAGE] Shouldn't come here!"),
        }
    }
}

// Implementation of GetTaskData in i8 (Range: -128 to 127 || -2^8 to ((2^8) - 1)) data type
impl GetTaskData for u8 {
    // Private function
    // Gets data in String type and returns in i8 type
    fn get_task_data(task_type: &TaskType) -> u8 {
        match task_type {
            TaskType::TaskId => match input::_get_data(String::from(
                "\x1b[32m?\x1b[0m 🆔 Task Id                    : ",
            ))
            .trim()
            .parse::<u8>()
            {
                Ok(task_id) => task_id,
                Err(err) => {
                    eprintln!("{}, hence taking value as `0`", err);
                    0
                }
            },
            TaskType::Important => match input::_get_data(String::from(
                "\x1b[32m?\x1b[0m 🌟 Important  [true/false]    : ",
            ))
            .trim()
            .parse::<bool>()
            {
                Ok(is_important) => is_important.into(),
                Err(err) => {
                    eprintln!("{}, hence taking value as `false`", err);
                    0
                }
            },
            TaskType::Completed => match input::_get_data(String::from(
                "\x1b[32m?\x1b[0m ✅ Completed  [true/false]    : ",
            ))
            .trim()
            .parse::<bool>()
            {
                Ok(is_completed) => is_completed.into(),
                Err(err) => {
                    eprintln!("{}, hence taking value as `false`", err);
                    0 // returns 0
                }
            },
            _ => 0,
        }
    }
}
