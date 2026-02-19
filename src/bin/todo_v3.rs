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
            compoleted: (false),
        });
    }

    for task in &tasks {
        println!("name : {} completed : {}", task.name, task.compoleted)
    }
}

struct Task {
    name: String,
    compoleted: bool,
}