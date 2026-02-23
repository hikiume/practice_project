use std::{fs::File, path::Path, usize};

use serde::{Deserialize, Serialize};

const FILE_PATH: &str = "tasks.json";

enum Command {
    AddTask(String),
    AllComplete,
    Quit,
    Continue,
    DeleteTask(usize),
}

#[derive(Serialize, Deserialize)]
struct Task {
    id: usize,
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
                let new_id = tasks.iter().map(|v| v.id).max().unwrap_or(0) + 1;
                tasks.push(Task {
                    id: (new_id),
                    name,
                    completed: (false),
                });
                save_tasks(FILE_PATH, &tasks)?
            }
            Command::DeleteTask(id) => {
                let original_len = tasks.len();
                tasks.retain(|v| v.id != id);

                if tasks.len() < original_len {
                    save_tasks(FILE_PATH, &tasks)?;
                    println!("ID {} は削除しました", id)
                } else {
                    println!("ID {} は見つかりませんでした", id)
                }
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
    let parts: Vec<&str> = input.trim().split_whitespace().collect();

    match parts.as_slice() {
        ["quit"] => Command::Quit,
        ["all complete"] => Command::AllComplete,
        [] => Command::Continue,
        ["delete", id_str] => {
            if let Ok(id) = id_str.parse::<usize>() {
                Command::DeleteTask(id)
            } else {
                Command::Continue
            }
        }
        _ => Command::AddTask(input.trim().to_string()),
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
