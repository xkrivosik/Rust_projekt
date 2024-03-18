use std::io::{self, Write};

struct TodoList {
    tasks: Vec<String>,
}

impl TodoList {
    fn new() -> Self {
        TodoList { tasks: Vec::new() }
    }

    fn add_task(&mut self, task: String) {
        self.tasks.push(task);
    }

    fn remove_task(&mut self, index: usize) {
        if index < self.tasks.len() {
            self.tasks.remove(index);
        } else {
            println!("Index out of range");
        }
    }

    fn list_tasks(&self) {
        println!("Tasks:");
        for (index, task) in self.tasks.iter().enumerate() {
            println!("{}: {}", index + 1, task);
        }
    }
}

fn main() {
    let mut todo_list = TodoList::new();

    loop {
        println!("Choose an option:");
        println!("1. Add task");
        println!("2. Remove task");
        println!("3. List tasks");
        println!("4. Exit");

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read input");

        match input.trim().parse() {
            Ok(1) => {
                println!("Enter task:");
                let mut task = String::new();
                io::stdin().read_line(&mut task).expect("Failed to read input");
                todo_list.add_task(task.trim().to_string());
            }
            Ok(2) => {
                println!("Enter index of task to remove:");
                let mut index_input = String::new();
                io::stdin().read_line(&mut index_input).expect("Failed to read input");
                let index = index_input.trim().parse::<usize>().unwrap_or(0);
                todo_list.remove_task(index - 1);
            }
            Ok(3) => {
                todo_list.list_tasks();
            }
            Ok(4) => {
                println!("Exiting...");
                break;
            }
            _ => {
                println!("Invalid option");
            }
        }
    }
}
