use std::{
    fs::File,
    io::{Read, Write},
};

use serde::{Deserialize, Serialize};

fn main() {
    let mut content = String::new();
    let mut file = File::open("tasks.json").expect("エラー");
    file.read_to_string(&mut content).expect("エラー");

    let mut tasks: Vec<Task> = serde_json::from_str(&content).expect("エラー");

    loop {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).expect("エラー");

        if handle_command(&input, &mut tasks) == LoopStatus::Exit {
            break;
        }

        tasks.push(Task {
            name: (input.trim().to_string()),
            completed: (false),
        });
    }

    tasks
        .iter()
        .for_each(|v| println!("v.name :{} v.conpleted :{}", v.name, v.completed));

    let json = serde_json::to_string_pretty(&tasks).expect("エラー");

    let mut write_file = File::create("tasks.json").expect("エラー");
    write_file.write_all(json.as_bytes()).expect("エラー");
}

#[derive(Serialize, Deserialize)]
struct Task {
    name: String,
    completed: bool,
}

#[derive(PartialEq)]
enum LoopStatus {
    Continue,
    Exit,
}

fn handle_command(input: &String, tasks: &mut Vec<Task>) -> LoopStatus {
    match input.trim() {
        "quit" => LoopStatus::Exit,
        "all complete" => {
            tasks.iter_mut().for_each(|v| v.completed = true);
            return LoopStatus::Exit;
        }
        _ => LoopStatus::Continue,
    }
}
