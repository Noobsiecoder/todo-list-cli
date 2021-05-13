// Function which displays the user manual
pub fn show_manual() {
    let task_commands = "📝 TASK COMMANDS:
    \t-a, add     \tTo add a new task
    \t-d, delete  \tTo delete an existing task
    \t-r, read    \tTo read all tasks available
    \t-u, update   \tTo update a task
    ";

    let flag_commands = "⛳ FLAG COMMANDS:
    \t--help      \tOutputs all the commands available
    \t--uninstall \tUninstall todo.exe
    \t--version   \tOutputs current CLI version
    ";

    println!("\x1b[1;36mHello World! This your assistant AI Taskio 🤖\x1b[0m");
    println!("\n💻 USAGE: \n\t\x1b[1;3;35mtodo [command | flag]\x1b[0m\n");
    println!("{}\n{}", task_commands, flag_commands);
}
