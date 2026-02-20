// cargo run --bin todo_v2.rs

use std::io;

fn main() {
    let mut tasks: Vec<Task> = Vec::new();

    let mut input = String::new();

    io::stdin().read_line(&mut input).expect("エラー");

    tasks.push(Task {
        name: (input),
        completed: (false),
    });

    println!("現在のタスク数 : {}", tasks.len());
    println!("最初のタスク : {}", tasks[0].name)
}

struct Task {
    name: String,
    completed: bool,
}
