use std::{
    fs::File,
    io::{Read, Write},
};

use serde::{Deserialize, Serialize};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let content = load_file("tasks.json").unwrap_or_else(|_|"[]".to_string());
    let mut tasks: Vec<Task> = serde_json::from_str(&content).unwrap_or_else(|_| Vec::new());

    loop {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input)?;

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

    let json = serde_json::to_string_pretty(&tasks)?;

    let mut write_file = File::create("tasks.json")?;
    write_file.write_all(json.as_bytes())?;

    Ok(())
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
            LoopStatus::Exit
        }
        _ => LoopStatus::Continue,
    }
}

fn load_file(file_name:&str) -> Result<String, Box<dyn std::error::Error>> {
    let mut content = String::new();
    let mut file = File::open(file_name)?;
    file.read_to_string(&mut content)?;
    Ok(content)
}
