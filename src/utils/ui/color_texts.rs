// Public function
// Prints error message
pub fn _error_message(error: String, info: String) {
    println!("\x1b[1;36mHello World! This your assistant AI Taskio 🤖\x1b[0m\n");
    println!("Error      : \x1b[1;31m{}\x1b[0m", error);
    println!("Error info : \x1b[2;31m{}\x1b[0m\n", info);
}

// Function yet to be implemented
#[allow(dead_code)]
pub fn success_message(_success: &String, _info: &String) {
    println!("\x1b[1;32m{}\x1b[0m", "Success!");
}

// Public function
// Prints warning message
pub fn _warning_message(warning: String, info: String) {
    println!("\x1b[1;36mHello World! This your assistant AI Taskio 🤖\x1b[0m\n");
    println!("Warning      : \x1b[1;33m{}\x1b[0m", warning);
    println!("Warning info : \x1b[2;33m{}\x1b[0m\n", info);
}
