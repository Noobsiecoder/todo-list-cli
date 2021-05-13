use std::env;

// Function gets arguments from user
pub fn get_args() -> Vec<String> {
    let args: Vec<String> = env::args().skip(1).collect();
    args
}
