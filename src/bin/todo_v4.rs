use std::io::Write;

use serde::{Deserialize, Serialize};

fn main() {
    let mut tasks: Vec<Task> = Vec::new();
    loop {
        let mut input = String::new();

        std::io::stdin().read_line(&mut input).expect("エラー");

        if input.trim() == "quit" {
            break;
        }

        tasks.push(Task {
            name: (input.trim().to_string()),
            completed: (false),
        });
    }

    let json = serde_json::to_string_pretty(&tasks).expect("エラー");

    let mut file = std::fs::File::create("tasks.json").expect("エラー");
    file.write_all(json.as_bytes()).expect("エラー");
}

#[derive(Serialize, Deserialize)]
struct Task {
    name: String,
    completed: bool,
}
