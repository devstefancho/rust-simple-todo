// use serde::{Deserialize, Serialize};
use serde_json;
use std::fs::File;
use std::io::{self};
use std::io::{stdout, Write};

fn main() {
    let file: File = File::open("todos.json").expect("todos.json file not found");
    let mut todos: Vec<String> = match serde_json::from_reader(file) {
        Ok(todos) => todos,
        Err(_) => Vec::new(),
    };

    if todos.is_empty() {
        todos = Vec::new();
    };

    loop {
        println!("\nTodo List CLI");
        println!("1. Add Todo");
        println!("2. View Todos");
        println!("3. Remove Todo");
        println!("4. Exit");
        print!("Enter your choice: ");
        stdout().flush().expect("Failed to flush stdout");

        let mut choice = String::new();
        match io::stdin().read_line(&mut choice) {
            Ok(_) => {}
            Err(e) => {
                println!("{e} Error..")
            }
        };
        let choice = choice.trim();

        match choice {
            "1" => add_todo(&mut todos),
            "2" => view_todos(),
            "3" => remove_todo(&mut todos),
            "4" => exit_app(),
            _ => println!("Invalid"),
        }
    }
}

fn save_todo(todos: &Vec<String>) {
    println!("Saving todos...");
    println!("{:?}", todos);
    let serialized = serde_json::to_string(todos).expect("Error serializing todos");
    let mut file = File::create("todos.json").expect("Error creating file");
    file.write_all(serialized.as_bytes())
        .expect("Error writing to file");
}

fn add_todo(todos: &mut Vec<String>) {
    print!("Enter a new todo: ");
    stdout().flush().expect("Invalid new todo...");

    let mut todo = String::new();
    match io::stdin().read_line(&mut todo) {
        Ok(_) => {
            println!("success to add a new todo...");
        }
        Err(e) => {
            println!("{e} Error..")
        }
    }
    todos.push(todo);
    save_todo(&todos);
}

fn remove_todo(todos: &mut Vec<String>) {
    view_todos();

    print!("Which todo do you want to remove?");
    stdout().flush().expect("Error flushing stdout");
    let mut index_str = String::new();

    match io::stdin().read_line(&mut index_str) {
        Ok(_) => match index_str.trim().parse::<usize>() {
            Ok(index) => {
                if index > 0 && index <= todos.len() {
                    println!("Removing todo: {}", todos[index - 1]);
                    todos.remove(index - 1);
                    save_todo(&todos);
                }
            }
            _ => println!("Invalid index"),
        },
        Err(e) => println!("{e} Error.."),
    }

    save_todo(&todos);
}

fn view_todos() {
    let file: File = File::open("todos.json").expect("No File exists");
    let todos: Vec<String> = match serde_json::from_reader(file) {
        Ok(todos) => todos,
        Err(_) => {
            println!("Error.....");
            Vec::new()
        }
    };

    if todos.is_empty() {
        println!("No todos yet!");
        return;
    }

    println!("--------------");
    println!("--Your Todos--");
    println!("--------------");
    for (i, todo) in todos.iter().enumerate() {
        print!("{}: {}", i + 1, todo);
    }
}

fn exit_app() {
    println!("Exiting...");
    std::process::exit(0);
}
