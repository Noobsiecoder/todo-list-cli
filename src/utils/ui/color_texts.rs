// Public function
// Prints error message
pub fn _error_message(error: String, info: String) {
    println!("\x1b[1;36mHello World! This your assistant AI Taskio 🤖\x1b[0m\n");
    println!("Error      : \x1b[1;31m{}\x1b[0m", error);
    println!("Error info : \x1b[2;31m{}\x1b[0m\n", info);
}

// Public function
// Prints success message
pub fn _success_message(success: &String, info: &String) {
    println!("Success      : \x1b[1;32m{}\x1b[0m", success);
    println!("Success info : \x1b[2;32m{}\x1b[0m\n", info);
}

// Public function
// Prints warning message
pub fn _warning_message(warning: String, info: String) {
    println!("\x1b[1;36mHello World! This your assistant AI Taskio 🤖\x1b[0m\n");
    println!("Warning      : \x1b[1;33m{}\x1b[0m", warning);
    println!("Warning info : \x1b[2;33m{}\x1b[0m\n", info);
}
