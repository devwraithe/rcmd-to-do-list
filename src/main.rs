mod task;

use chrono::{DateTime, Utc};
use clap::{Arg, Command};
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;
use std::fs;
use std::path::Path;
use task::Task;

const TASKS_FILE: &str = "tasks.json";

#[derive(Serialize, Deserialize)]
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

    fn get_all_tasks(&self) -> &VecDeque<Task> {
        &self.tasks
    }

    fn get_pending_tasks(&self) -> Vec<&Task> {
        self.tasks.iter().filter(|task| !task.completed).collect()
    }

    fn get_completed_tasks(&self) -> Vec<&Task> {
        self.tasks.iter().filter(|task| task.completed).collect()
    }

    fn get_tasks_by_due_date(&self) -> Vec<&Task> {
        let mut tasks: Vec<&Task> = self.tasks.iter().collect();
        tasks.sort_by_key(|task| task.due_date);
        tasks
    }

    fn load_tasks() -> TaskManager {
        if Path::new(TASKS_FILE).exists() {
            let data = fs::read_to_string(TASKS_FILE).expect("Unable to read file");
            serde_json::from_str(&data).expect("Unable to parse JSON")
        } else {
            TaskManager::new()
        }
    }

    fn save_tasks(&self) {
        let data = serde_json::to_string(&self).expect("Unable to serialize tasks");
        fs::write(TASKS_FILE, data).expect("Unable to write file");
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
                        .help("The due date of the task (RFC3339 format)")
                        .required(false)
                        .index(3),
                ),
        )
        .subcommand(
            Command::new("view")
                .about("View tasks")
                .arg(
                    Arg::new("filter")
                        .help("Filter tasks by status or due date")
                        .long("filter")
                        .short('f')
                        .value_name("FILTER")
                        .value_parser(["all", "pending", "completed", "due_date"])
                        .default_value("all"),
                ),
        )
        .get_matches();

    let mut manager = TaskManager::load_tasks();

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

        manager.save_tasks();

        println!("Task added successfully!");
    }

    if let Some(matches) = matches.subcommand_matches("view") {
        let filter = matches.get_one::<String>("filter").unwrap().as_str();
        match filter {
            "all" => display_tasks(manager.get_all_tasks().iter().collect()),
            "pending" => display_tasks(manager.get_pending_tasks()),
            "completed" => display_tasks(manager.get_completed_tasks()),
            "due_date" => display_tasks(manager.get_tasks_by_due_date()),
            _ => unreachable!(),
        }
    }
}

fn display_tasks(tasks: Vec<&Task>) {
    for task in tasks {
        println!(
            "Title: {}\nDescription: {}\nDue Date: {}\nCompleted: {}\n",
            task.title,
            task.description.as_deref().unwrap_or("No description"),
            task.due_date
                .map(|d| d.to_string())
                .unwrap_or_else(|| "No due date".to_string()),
            task.completed
        );
    }
}
