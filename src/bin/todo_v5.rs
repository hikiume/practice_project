use std::{fs::File, io::Read};

use serde::{Deserialize, Serialize};

fn main() {
    let mut content = String::new();

    let mut file = File::open("tasks.json").expect("エラー");
    file.read_to_string(&mut content).expect("エラー");

    let tasks: Vec<Task> = serde_json::from_str(&content).expect("エラー");

    for task in &tasks {
        println!("{}{}", task.name, task.completed)
    }
}

#[derive(Serialize, Deserialize)]
struct Task {
    name: String,
    completed: bool,
}
