mod task;

use chrono::{DateTime, Utc};
use clap::{Arg, Command};
use std::collections::VecDeque;
use task::Task;

struct TaskManager {
    tasks: VecDeque<Task>,
}

impl TaskManager {
    fn new() -> TaskManager {
        TaskManager {
            tasks: VecDeque::new(),
        }
    }

    fn add_task(&mut self, task: Task) {
        self.tasks.push_back(task);
    }
}

fn main() {
    let matches = Command::new("To Do List")
        .version("1.0")
        .author("Ibrahim Ibrahim (Devwraithe) <folahanmi0001@gmail.com>")
        .about("Manages your tasks")
        .subcommand(
            Command::new("add")
                .about("Adds a new task")
                .arg(
                    Arg::new("title")
                        .help("The title of the task")
                        .required(true)
                        .index(1),
                )
                .arg(
                    Arg::new("description")
                        .help("The description of the task")
                        .required(false)
                        .index(2),
                )
                .arg(
                    Arg::new("due_date")
                        .help("The due date of the task (RFC3339 format")
                        .required(false)
                        .index(3),
                ),
        )
        .get_matches();

    let mut manager = TaskManager::new();

    if let Some(matches) = matches.subcommand_matches("add") {
        let title = matches.get_one::<String>("title").unwrap().to_string();
        let description = matches
            .get_one::<String>("description")
            .map(|d| d.to_string());
        let due_date = matches.get_one::<String>("due_date").map(|d| {
            DateTime::parse_from_rfc3339(d)
                .expect("Invalid date format")
                .with_timezone(&Utc)
        });

        let task = Task::new(title, description, due_date);
        manager.add_task(task);

        println!("Task added successfully");
    }
}
