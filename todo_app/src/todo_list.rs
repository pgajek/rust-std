use crate::task::Task; // Import Task struct
use colored::*;
use std::fs::File;
use std::io::{Read, Write};

pub struct TodoList {
    pub tasks: Vec<Task>,
}

impl TodoList {
    pub fn new() -> TodoList {
        TodoList { tasks: Vec::new() }
    }

    pub fn add_task(&mut self, description: String) {
        let task = Task::new(description);
        self.tasks.push(task);
    }

    pub fn list_tasks(&self) {
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

    pub fn complete_task(&mut self, index: usize) {
        if index > 0 && index <= self.tasks.len() {
            self.tasks[index - 1].complete();
            println!("Task {} completed", index);
        } else {
            println!("Invalid task number!")
        }
    }

    pub fn save_to_file(&self, filename: &str) {
        let json_data = serde_json::to_string_pretty(&self.tasks).unwrap();
        let mut file = File::create(filename).expect("Unable to create file");
        file.write_all(json_data.as_bytes())
            .expect("Unable to write data");
    }

    pub fn load_from_file(&mut self, filename: &str) {
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
