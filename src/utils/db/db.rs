#[path = "../error/errors.rs"]
mod error;

#[derive(Debug)]
pub struct Task {
    // Task name and note
    pub task_name: String,
    pub task_note: String,

    // Date
    pub start_date: String,
    pub end_date: String,

    // Time
    pub start_time: String,
    pub end_time: String,

    // Task priority and status
    pub is_important: i8,
    pub is_completed: i8,
}

impl Task {
    // Function creates connection with the database
    fn _create_database_if_not_exists() -> sqlite::Connection {
        let connection = sqlite::open("task.db").unwrap();

        connection
            .execute("CREATE TABLE IF NOT EXISTS user_tasks (TASK_ID INTEGER PRIMARY KEY AUTOINCREMENT, TASK_NAME TEXT, TASK_NOTE TEXT, START_DATE TEXT, START_TIME TEXT, END_DATE TEXT, END_TIME TEXT, IMPORTANT INTEGER, COMPLETED INTEGER);")
            .unwrap();
        connection
    }

    // Function adds tasks to the database
    pub fn _add_data_to_db(&self) {
        let connection = Task::_create_database_if_not_exists();
        connection
            .execute(format!(
                "INSERT INTO user_tasks (TASK_NAME, TASK_NOTE, START_DATE, START_TIME, END_DATE, END_TIME, IMPORTANT, COMPLETED) VALUES ('{}', '{}', '{}', '{}', '{}', '{}', {} , {});",
                self.task_name, self.task_note, self.start_date, self.start_time, self.end_date, self.end_time, self.is_important, self.is_completed
            ))
            .unwrap();
    }

    // Yet to implement
    pub fn delete(task_id: &String) {}

    // Function returns task_data in nested vector form which is being taken from database
    pub fn _read_tasks_from_database() -> Vec<Vec<String>> {
        let mut task_data: Vec<Vec<String>> = vec![];

        let connection = Task::_create_database_if_not_exists();
        connection
            .iterate(
                "SELECT * FROM user_tasks",
                |pairs: &[(&str, Option<&str>)]| {
                    for &(column, value) in pairs.iter() {
                        if column == "TASK_ID" {
                            task_data.push(vec![value.unwrap().to_string()]);
                        } else if column == "IMPORTANT" {
                            match value.unwrap() {
                                "0" => task_data
                                    .last_mut()
                                    .unwrap()
                                    .push(String::from("Not Important")),
                                "1" => task_data.last_mut().unwrap().push(String::from("Important")),
                                _ => {
                                    error::Errors::_throw_error(&error::Errors::_DatabaseBoolError)
                                }
                            }
                        } else if column == "COMPLETED" {
                            match value.unwrap() {
                                "0" => task_data
                                    .last_mut()
                                    .unwrap()
                                    .push(String::from("Yet to completed")),
                                "1" => task_data.last_mut().unwrap().push(String::from("Completed")),
                                _ => {
                                    error::Errors::_throw_error(&error::Errors::_DatabaseBoolError)
                                }
                            }
                        } else {
                            task_data.last_mut().unwrap().push(value.unwrap().to_string());
                        }
                    }
                    true
                },
            )
            .unwrap();
        task_data
    }

    // Yet to implement
    pub fn update(task_type: &String, task_id: &String) {}
}
