#[path = "../../input/get_task_data.rs"]
// Database module
// Recieves inputted data and constructs as a struct Task
mod task_data;

#[path = "../db.rs"]
// Database module
// Contains database middlewares
mod database;

// Private function
// Gets task id from database
// Doesn't stop until correct id is recieved
fn get_task_id() -> u8 {
    loop {
        let id = task_data::GetTaskData::get_task_data(&task_data::TaskType::TaskId);
        let does_id_exists = database::Task::_check_id_exists(&id);
        if does_id_exists {
            return id;
        } else {
            eprintln!("ID doesn't exist! Please try again\n");
        }
    }
}

// Private function
// Displays the task name in abbreviated form and full form
fn display_task_type() {
    println!("\nThese are the task names :-");
    println!("tnm\tTask Name");
    println!("sdt\tStart Date");
    println!("stm\tStart Time");
    println!("edt\tEnd Date");
    println!("etm\tEnd Time");
    println!("imp\tImportant");
    println!("com\tImportant");
}

// Public function
// Middleware for updating task
// Takes task id, task type and corresponding data
// Sends it to database middleware
pub fn update_task() {
    let task_id = get_task_id();
    loop {
        display_task_type();
        let task_type: String =
            task_data::GetTaskData::get_task_data(&task_data::TaskType::TaskType);
        match task_type.as_str() {
            "tnm" => {
                let data: String =
                    task_data::GetTaskData::get_task_data(&task_data::TaskType::Name);
                database::Task::_update_task_data_in_database(
                    &task_id,
                    &String::from("TASK_NAME"),
                    &data,
                );
                break;
            }
            "sdt" => {
                let data: String =
                    task_data::GetTaskData::get_task_data(&task_data::TaskType::StartDate);
                database::Task::_update_task_data_in_database(
                    &task_id,
                    &String::from("START_DATE"),
                    &data,
                );
                break;
            }
            "stm" => {
                let data: String =
                    task_data::GetTaskData::get_task_data(&task_data::TaskType::StartTime);
                database::Task::_update_task_data_in_database(
                    &task_id,
                    &String::from("START_TIME"),
                    &data,
                );
                break;
            }
            "edt" => {
                let data: String =
                    task_data::GetTaskData::get_task_data(&task_data::TaskType::EndDate);
                database::Task::_update_task_data_in_database(
                    &task_id,
                    &String::from("END_DATE"),
                    &data,
                );
                break;
            }
            "etm" => {
                let data: String =
                    task_data::GetTaskData::get_task_data(&task_data::TaskType::EndTime);
                database::Task::_update_task_data_in_database(
                    &task_id,
                    &String::from("END_TIME"),
                    &data,
                );
                break;
            }
            "imp" => {
                let data: u8 =
                    task_data::GetTaskData::get_task_data(&task_data::TaskType::Important);
                database::Task::_update_task_data_in_database(
                    &task_id,
                    &String::from("IMPORTANT"),
                    &data.to_string(),
                );
                break;
            }
            "com" => {
                let data: u8 =
                    task_data::GetTaskData::get_task_data(&task_data::TaskType::Completed);
                database::Task::_update_task_data_in_database(
                    &task_id,
                    &String::from("COMPLETED"),
                    &data.to_string(),
                );
                break;
            }
            err => {
                eprintln!("{}, please try again\n", err);
            }
        }
    }
}
