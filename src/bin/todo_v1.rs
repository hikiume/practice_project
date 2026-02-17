// cargo run --bin todo_v1
use std::io;

fn main() {
    let mut gus = String::new();

    io::stdin().read_line(&mut gus).expect("エラー");

    println!("タスク : {}", gus)
}
