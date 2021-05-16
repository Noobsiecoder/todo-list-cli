#[path = "../../input/get_task_data.rs"]
// Task module
// Recieves inputted data and constructs as a struct Task
mod task_data;

#[path = "../db.rs"]
// Database module
// Contains database middlewares
mod database;

#[path = "../../output/display_in_table.rs"]
// Table module
// Used for diplaying user data in a table
mod display_in_table;

// Public function
// Middleware for reading task
// Takes all the data from the database
// Sends it display_in_table module to display in table
pub fn read_task() {
    let user_task_data = database::Task::_read_tasks_from_database();
    display_in_table::display_data(&user_task_data);
}
