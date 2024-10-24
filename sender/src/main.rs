use std::io::Write;
use std::process::{Command, Stdio};

fn main() {
    let mut child = Command::new("/home/redooo25/working/rust_playground/std/std/target/debug/std")
        .stdin(Stdio::piped())
        .spawn()
        .expect("Failed to start program");

    if let Some(stdin) = child.stdin.as_mut() {
        stdin
            .write_all(b"143242")
            // .write_all(b"Hello from another program!\n")
            .expect("Failed to write to stdin");
    }

    child.wait().expect("Failed to wait on child");
}
