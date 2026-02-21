use std::{
    fs::File,
    io::{Read, Write},
};

use serde::{Deserialize, Serialize};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let content = load_file("tasks.json").unwrap_or_else(|_| "[]".to_string());
    let mut tasks: Vec<Task> = serde_json::from_str(&content).unwrap_or_else(|_| Vec::new());

    loop {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input)?;

        match parse_command(&input) {
            Command::Quit => break,
            Command::AllComplete => tasks.iter_mut().for_each(|v| v.completed = true),
            Command::AddTask(name) => {
                tasks.push(Task {
                    name,
                    completed: (false),
                });
            }
        }
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

fn load_file(file_name: &str) -> Result<String, Box<dyn std::error::Error>> {
    let mut content = String::new();
    let mut file = File::open(file_name)?;
    file.read_to_string(&mut content)?;
    Ok(content)
}

enum Command {
    AddTask(String),
    AllComplete,
    Quit,
}

fn parse_command(input: &str) -> Command {
    match input.trim() {
        "quit" => Command::Quit,
        "all complete" => Command::AllComplete,
        name => Command::AddTask(name.to_string()),
    }
}
