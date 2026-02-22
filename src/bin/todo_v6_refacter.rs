use std::{
    fs::File,
    io::{Read, Write},
    path::{self, Path},
};

use serde::{Deserialize, Serialize};

const FILE_PATH: &str = "tasks.json";

enum Command {
    AddTask(String),
    AllComplete,
    Quit,
    Continue,
}

#[derive(Serialize, Deserialize)]
struct Task {
    name: String,
    completed: bool,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut tasks: Vec<Task> = load_file(FILE_PATH)?;

    loop {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input)?;

        match parse_command(&input) {
            Command::Quit => break,
            Command::AllComplete => tasks.iter_mut().for_each(|v| v.completed = true),
            Command::Continue => (),
            Command::AddTask(name) => {
                tasks.push(Task {
                    name,
                    completed: (false),
                });
            }
        }
    }

    display_tasks(&tasks);
    save_tasks(FILE_PATH, &tasks)?;

    Ok(())
}

fn load_file<P: AsRef<Path>>(path: P) -> Result<Vec<Task>, Box<dyn std::error::Error>> {
    if !path.as_ref().exists() {
        return Ok(Vec::new());
    }
    let mut content = String::new();
    let mut file = File::open(path.as_ref())?;
    file.read_to_string(&mut content)?;
    let tasks = serde_json::from_str(&content).unwrap_or_else(|_| {
        println!("ファイルが読み込めないので、新しいリストを作成します。");
        Vec::new()
    });

    Ok(tasks)
}

fn parse_command(input: &str) -> Command {
    match input.trim() {
        "quit" => Command::Quit,
        "all complete" => Command::AllComplete,
        "" => Command::Continue,
        name => Command::AddTask(name.to_string()),
    }
}

fn save_tasks<P: AsRef<Path>>(path: P, tasks: &[Task]) -> std::io::Result<()> {
    let json = serde_json::to_string_pretty(&tasks).expect("JSONの生成に失敗しました");
    let mut write_file = File::create(path)?;
    write_file.write_all(json.as_bytes())?;

    Ok(())
}

fn display_tasks(tasks: &[Task]) {
    println!("--------最終タスクリスト-------");
    tasks.iter().for_each(|v| {
        let status = if v.completed { "[x]" } else { "[ ]" };
        println!("{} {}", status, v.name)
    });
}
