use std::{fs::File, path::Path};

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
        println!(">");

        let mut input = String::new();
        std::io::stdin().read_line(&mut input)?;

        match parse_command(&input) {
            Command::Quit => break,
            Command::AllComplete => {
                tasks.iter_mut().for_each(|v| v.completed = true);
                save_tasks(FILE_PATH, &tasks)?;
                println!("全てのタスクを完了しました");
            }
            Command::Continue => (),
            Command::AddTask(name) => {
                tasks.push(Task {
                    name,
                    completed: (false),
                });
                save_tasks(FILE_PATH, &tasks)?
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

    let file = File::open(path)?;
    let tasks = serde_json::from_reader(file)?;

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
    let file = File::create(path)?;
    serde_json::to_writer_pretty(file, &tasks).expect("JSONの生成に失敗しました");

    Ok(())
}

fn display_tasks(tasks: &[Task]) {
    println!("--------最終タスクリスト-------");
    tasks.iter().for_each(|v| {
        let status = if v.completed { "[x]" } else { "[ ]" };
        println!("{} {}", status, v.name)
    });
}
