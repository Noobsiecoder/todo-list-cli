#[derive(Debug)]
pub struct Task {
    // Task name and note
    pub task_name: String,

    // Date
    pub start_date: String,
    pub end_date: String,

    // Time
    pub start_time: String,
    pub end_time: String,

    // Task priority and status
    pub is_important: u8,
    pub is_completed: u8,
}

impl Task {
    // Private function
    // Creates connection with the database and returns connection struct
    fn _create_database_if_not_exists() -> sqlite::Connection {
        let connection = sqlite::open("./task.db").unwrap();

        connection
            .execute("CREATE TABLE IF NOT EXISTS user_tasks (TASK_ID INTEGER PRIMARY KEY AUTOINCREMENT, TASK_NAME TEXT, START_DATE TEXT, START_TIME TEXT, END_DATE TEXT, END_TIME TEXT, IMPORTANT INTEGER, COMPLETED INTEGER);").unwrap();
        connection
    }

    // Private function
    // Unwraps string into meaning data to output data which has been read from the database
    // Used for IMPORTANT and COMPLPETED task type
    fn _unwrap_boolstring_from_database(
        value: &str,
        task_data: &mut Vec<Vec<String>>,
        data: (String, String),
    ) {
        match value {
            "0" => task_data.last_mut().unwrap().push(data.0),
            "1" => task_data.last_mut().unwrap().push(data.1),
            _ => eprintln!("Error unwrapping!"),
        }
    }

    // Public function
    // Checks if id exists in database
    // Returns boolean value corresponding to the fetched data
    pub fn _check_id_exists(task_id: &u8) -> bool {
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
    // Database middleware which add's data in the database
    pub fn _add_task_in_database(&self) {
        let sqlite_connection = Task::_create_database_if_not_exists();

        match sqlite_connection
            .execute(format!(
                "INSERT INTO user_tasks (TASK_NAME, START_DATE, START_TIME, END_DATE, END_TIME, IMPORTANT, COMPLETED) VALUES ('{}', '{}', '{}', '{}', '{}', {} , {});",
                self.task_name, self.start_date, self.start_time, self.end_date, self.end_time, self.is_important, self.is_completed
            )) {
                Ok(_) => println!("\x1b[1;36mTask added successfully 🤖\x1b[0m"),
                Err(err) => eprintln!("{}", err)
            };
    }

    // Public function
    // Database middleware which delete's data from the database
    pub fn _delete_task_from_database(task_id: &u8) {
        let sqlite_connection = Task::_create_database_if_not_exists();
        match sqlite_connection.execute(format!("DELETE FROM user_tasks WHERE TASK_ID={}", task_id))
        {
            Ok(_) => println!("\x1b[1;36mTask deleted successfully 🤖\x1b[0m"),
            Err(err) => eprintln!("{}", err),
        };
    }

    // Public function
    // Database middleware which read's data from the database
    // Returns a nested array of tasks to represent different types of tasks from the database
    pub fn _read_tasks_from_database() -> Vec<Vec<String>> {
        let mut user_data: Vec<Vec<String>> = vec![];
        let sqlite_connection = Task::_create_database_if_not_exists();
        sqlite_connection
            .iterate("SELECT * FROM user_tasks", |pairs| {
                for &(column_name, pair) in pairs.iter() {
                    if column_name == "IMPORTANT" {
                        Task::_unwrap_boolstring_from_database(
                            pair.unwrap(),
                            &mut user_data,
                            (String::from("Not Important"), String::from("Important")),
                        );
                    } else if column_name == "COMPLETED" {
                        Task::_unwrap_boolstring_from_database(
                            pair.unwrap(),
                            &mut user_data,
                            (String::from("Not Completed"), String::from("Completed")),
                        );
                    } else if column_name == "TASK_ID" {
                        user_data.push(vec![pair.unwrap().to_string()]);
                    } else {
                        user_data
                            .last_mut()
                            .unwrap()
                            .push(pair.unwrap().to_string());
                    }
                }
                true
            })
            .unwrap();
        user_data
    }

    // Public function
    // Database middleware which update's data in the database
    pub fn _update_task_data_in_database(task_id: &u8, task_type: &String, data: &String) {
        let sqlite_connection = Task::_create_database_if_not_exists();

        if task_type == "IMPORTANT" || task_type == "COMPLETED" {
            match sqlite_connection.execute(format!(
                "UPDATE user_tasks SET {}={} WHERE TASK_ID={}",
                task_type,
                data.parse::<u8>().unwrap(),
                task_id
            )) {
                Ok(_) => println!("\x1b[1;36mTask updated successfully 🤖\x1b[0m"),
                Err(err) => eprintln!("{}", err),
            };
        } else {
            match sqlite_connection.execute(format!(
                "UPDATE user_tasks SET {}='{}' WHERE TASK_ID={}",
                task_type, data, task_id
            )) {
                Ok(_) => println!("\x1b[1;36mTask updated successfully 🤖\x1b[0m"),
                Err(err) => eprintln!("{}", err),
            };
        }
    }
}
