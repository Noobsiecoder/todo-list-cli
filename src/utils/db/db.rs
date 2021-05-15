#[path = "../error/errors.rs"]
mod error;

#[path = "../ui/color_texts.rs"]
mod text;

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
    // Private function
    // Creates connection with the database and returns connection struct
    fn _create_database_if_not_exists() -> sqlite::Connection {
        let connection = sqlite::open("task.db").unwrap();

        connection
            .execute("CREATE TABLE IF NOT EXISTS user_tasks (TASK_ID INTEGER PRIMARY KEY AUTOINCREMENT, TASK_NAME TEXT, TASK_NOTE TEXT, START_DATE TEXT, START_TIME TEXT, END_DATE TEXT, END_TIME TEXT, IMPORTANT INTEGER, COMPLETED INTEGER);")
            .unwrap();
        connection
    }

    // Private function
    // Checks if task_id == TASK_ID or id given exists in database and returns a boolean value
    fn _check_task_id_exists(task_id: &u8) -> bool {
        let connection = Task::_create_database_if_not_exists();
        let mut flag = false;
        connection
            .iterate(
                format!("SELECT * FROM user_tasks WHERE TASK_ID={}", task_id),
                |_pair| {
                    flag = true;
                    true
                },
            )
            .unwrap();
        flag
    }

    // Public function
    // Adds tasks to the database
    pub fn _add_data_to_db(&self) {
        let connection = Task::_create_database_if_not_exists();
        match connection
            .execute(format!(
                "INSERT INTO user_tasks (TASK_NAME, TASK_NOTE, START_DATE, START_TIME, END_DATE, END_TIME, IMPORTANT, COMPLETED) VALUES ('{}', '{}', '{}', '{}', '{}', '{}', {} , {});",
                self.task_name, self.task_note, self.start_date, self.start_time, self.end_date, self.end_time, self.is_important, self.is_completed
            )) {
                Ok(_) => {
                    let code = "todo -r";
                    text::_success_message(
                    &String::from("DatabaseSuccess"),
                    &String::from("Data has been added successfully!"),
                    );
                    println!("❓ Try this : \x1b[1;3;35m{}\x1b[0m", code);
                },
                Err(_err) => error::Errors::_throw_error(&error::Errors::DatabaseAddError),
            }
    }

    // Public function
    // Delete's task from the database using task_id
    pub fn _delete(task_id: &u8) {
        let connection = Task::_create_database_if_not_exists();
        let task_id_exists = Task::_check_task_id_exists(&task_id);
        if task_id_exists {
            match connection.execute(format!("DELETE FROM user_tasks WHERE TASK_ID={}", task_id)) {
                Ok(_) => {
                    let code = "todo -r";
                    text::_success_message(
                        &String::from("DatabaseSuccess"),
                        &String::from("Data has been deleted successfully!"),
                    );
                    println!("❓ Try this : \x1b[1;3;35m{}\x1b[0m", code);
                }
                Err(_err) => error::Errors::_throw_error(&error::Errors::DatabaseDeleteError),
            }
        } else {
            error::Errors::_throw_error(&error::Errors::TaskIdError);
        }
    }

    // Public function
    // Returns task_data in nested vector form which is being taken from database
    pub fn _read_tasks_from_database() -> Vec<Vec<String>> {
        let mut task_data: Vec<Vec<String>> = vec![];

        let connection = Task::_create_database_if_not_exists();
        match connection.iterate(
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
                            "1" => task_data
                                .last_mut()
                                .unwrap()
                                .push(String::from("Important")),
                            _ => error::Errors::_throw_error(&error::Errors::DatabaseBoolError),
                        }
                    } else if column == "COMPLETED" {
                        match value.unwrap() {
                            "0" => task_data
                                .last_mut()
                                .unwrap()
                                .push(String::from("Yet to completed")),
                            "1" => task_data
                                .last_mut()
                                .unwrap()
                                .push(String::from("Completed")),
                            _ => error::Errors::_throw_error(&error::Errors::DatabaseBoolError),
                        }
                    } else {
                        task_data
                            .last_mut()
                            .unwrap()
                            .push(value.unwrap().to_string());
                    }
                }
                true
            },
        ) {
            Ok(_) => (),
            Err(_err) => error::Errors::_throw_error(&error::Errors::DatabaseReadError),
        }
        task_data
    }

    // Public function
    // Updates data corresponding to the task id given
    pub fn _update_data_in_db(task_id: &u8, task_type: &'static str, data: &String) {
        let connection = Task::_create_database_if_not_exists();
        let task_id_exists = Task::_check_task_id_exists(&task_id);
        if task_id_exists {
            if task_type == "IMPORTANT" || task_type == "COMPLETED" {
                match connection.execute(format!(
                    "UPDATE user_tasks SET {}={} WHERE TASK_ID={}",
                    task_type,
                    data.parse::<i8>().unwrap(),
                    task_id
                )) {
                    Ok(_) => {
                        let code = "todo -r";
                        text::_success_message(
                            &String::from("DatabaseSuccess"),
                            &String::from("Data has been updated successfully!"),
                        );
                        println!("❓ Try this : \x1b[1;3;35m{}\x1b[0m", code);
                    }
                    Err(_err) => error::Errors::_throw_error(&error::Errors::DatabaseUpdateError),
                }
            } else {
                match connection.execute(format!(
                    "UPDATE user_tasks SET {}='{}' WHERE TASK_ID={}",
                    task_type, data, task_id
                )) {
                    Ok(_) => {
                        let code = "todo -r";
                        text::_success_message(
                            &String::from("DatabaseSuccess"),
                            &String::from("Data has been updated successfully!"),
                        );
                        println!("❓ Try this : \x1b[1;3;35m{}\x1b[0m", code);
                    }
                    Err(_err) => error::Errors::_throw_error(&error::Errors::DatabaseUpdateError),
                }
            }
        } else {
            error::Errors::_throw_error(&error::Errors::TaskIdError);
        }
    }
}
