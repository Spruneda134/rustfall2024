use std::process::Command;

fn executing_os_commands_linux() {
    let output = Command::new("sudo cat example.txt")
        .output()
        .expect("Failed to execute command");

    println!("Command output: {}", String::from_utf8_lossy(&output.stdout));
}
fn main() {
    executing_os_commands_linux();
}