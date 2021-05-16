#[path = "../get_task_data.rs"]
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

// Public function
//
pub fn delete_task() {
    let id = get_task_id();
    database::Task::_delete_task_from_database(&id);
}
