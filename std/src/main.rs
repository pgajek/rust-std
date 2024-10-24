use regex::Regex;
use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let re = Regex::new(r"\d+").unwrap();

    println!("Please enter a string with numbers(or type exit to quit):");

    for line in stdin.lock().lines() {
        let input = line.expect("Failed to read line");

        if input.trim().eq_ignore_ascii_case("exit") {
            println!("Exiting...");
        }

        let mut found = false;
        for caps in re.captures_iter(&input) {
            println!("You enteres: {}", &caps[0]);
            found = true;
        }

        if !found {
            println!("No numbers found in the input");
        }

        println!("\nEnter another string or type 'exit' to quit:")
    }
}

#[cfg(test)]
mod tests {
    // use super::*;

    #[test]
    fn test_trim_input() {
        let input = String::from(" hello \n");
        assert_eq!(input.trim(), "hello");
    }
}
