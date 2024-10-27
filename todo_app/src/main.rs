mod task;
mod todo_list;

use crate::todo_list::TodoList;
use colored::*;
use std::io::{self, Write};

fn main() {
    let mut todo_list = TodoList::new();
    println!(
        "{} {} {}{}{}{} {}",
        "WELCOME".bright_blue(),
        "to our",
        "T".bright_purple(),
        "O".white(),
        "D".bright_purple(),
        "O".white(),
        "LIST".bright_blue(),
    );
    let filename = "tasks.json";

    todo_list.load_from_file(filename);
    loop {
        println!("\n1. Add Task");
        println!("2. List Tasks");
        println!("3. Complete Task");
        println!("4. Exit");

        let mut choice = String::new();
        print!("{}", "\nenter your choice: ".blue());
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut choice).unwrap();

        match choice.trim() {
            "1" => {
                let mut description = String::new();
                print!("Enter task description: ");

                io::stdout().flush().unwrap();
                io::stdin().read_line(&mut description).unwrap();

                todo_list.add_task(description.trim().to_string());
                println!("{}", "Task added!".green());
            }
            "2" => {
                todo_list.list_tasks();
            }
            "3" => {
                let mut task_number = String::new();
                print!("Enter task number to complete: ");

                io::stdout().flush().unwrap();
                io::stdin().read_line(&mut task_number).unwrap();

                if let Ok(num) = task_number.trim().parse::<usize>() {
                    todo_list.complete_task(num);
                } else {
                    println!("Invalid task number!")
                }
            }
            "4" => {
                todo_list.save_to_file(filename);
                println!("{}", "Tasks saved. Goodbye!".yellow());
                break;
            }
            _ => {
                println!("Invalid choice!")
            }
        }
    }
}
