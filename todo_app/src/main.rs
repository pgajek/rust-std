use colored::*;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{self, Read, Write};
#[derive(Serialize, Deserialize)]
struct Task {
    description: String,
    completed: bool,
}

impl Task {
    // fn new(description: &str) -> Task {

    // }
    fn new(description: String) -> Task {
        Task {
            description,
            completed: false,
        }
    }

    fn complete(&mut self) {
        self.completed = true;
    }
}

struct TodoList {
    tasks: Vec<Task>,
}

impl TodoList {
    fn new() -> TodoList {
        TodoList { tasks: Vec::new() }
    }

    fn add_task(&mut self, description: String) {
        let task = Task::new(description);
        self.tasks.push(task);
    }

    fn list_tasks(&self) {
        for (index, task) in self.tasks.iter().enumerate() {
            let status = if task.completed {
                "✔".green()
            } else {
                "✘".red()
            };
            println!(
                "{}: [{}] - {}",
                index + 1,
                status,
                task.description.magenta()
            );
        }
    }

    fn complete_task(&mut self, index: usize) {
        if index > 0 && index <= self.tasks.len() {
            self.tasks[index - 1].complete();
            println!("Task {} completed", index);
        } else {
            println!("invalid task number!")
        }
    }

    fn save_to_file(&self, filename: &str) {
        let json_data = serde_json::to_string_pretty(&self.tasks).unwrap(); //understand this
        let mut file = File::create(filename).expect("Unable to create file");
        file.write_all(json_data.as_bytes())
            .expect("Unable to write data");
    }

    fn load_from_file(&mut self, filename: &str) {
        if let Ok(mut file) = File::open(filename) {
            let mut contents = String::new();
            file.read_to_string(&mut contents)
                .expect("Unable to read file");
            if let Ok(tasks) = serde_json::from_str::<Vec<Task>>(&contents) {
                self.tasks = tasks;
                println!("Tasks loaded from file successfully!");
            } else {
                println!(
                    "{}",
                    "Could not parse JSON. Starting with empty list.".yellow()
                );
            }
        } else {
            println!(
                "{}",
                "No saved tasks found. Starting with empty list.".yellow()
            );
        }
    }
}

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
