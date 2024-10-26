use colored::*;
use std::io::{self, Write};
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
}

fn main() {
    let mut todo_list = TodoList::new();

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
                println!("Exiting...");
                break;
            }
            _ => {
                println!("Invalid choice!")
            }
        }
    }
}
