#[path = "../../input/get_task_data.rs"]
// Task module
// Recieves inputted data and constructs as a struct Task
mod task_data;

#[path = "../db.rs"]
// Database module
// Contains database middlewares
mod database;

pub fn add_task() {
    println!("\x1b[1;36mHello World! This your assistant AI Taskio 🤖\x1b[0m");
    let (task_name, start_date, start_time, end_date, end_time, is_important, is_completed): (
        String,
        String,
        String,
        String,
        String,
        u8,
        u8,
    ) = (
        task_data::GetTaskData::get_task_data(&task_data::TaskType::Name),
        task_data::GetTaskData::get_task_data(&task_data::TaskType::StartDate),
        task_data::GetTaskData::get_task_data(&task_data::TaskType::StartTime),
        task_data::GetTaskData::get_task_data(&task_data::TaskType::EndDate),
        task_data::GetTaskData::get_task_data(&task_data::TaskType::EndTime),
        task_data::GetTaskData::get_task_data(&task_data::TaskType::Important),
        task_data::GetTaskData::get_task_data(&task_data::TaskType::Completed),
    );

    let task_data = database::Task {
        task_name,

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
    database::Task::_add_task_in_database(&task_data);
}
