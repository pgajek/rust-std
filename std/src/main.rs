use std::io;
fn main() {
    let mut input = String::new();

    println!("Please enter something:");

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    println!("You entered: {}", input.trim());
}
