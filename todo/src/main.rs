use core::task;
use std::{io, path};
use serde::{Deserialize, Serialize};
use serde_json::Result;
fn main() {
    
    let mut tasks: Vec<Task> = if path::Path::new("tasks.json").exists() {
        load_tasks()
    } else {
        Vec::new()
    };
    
    loop{  
        clear_screen();   
        let mut input = String::new();
        println!("Add a todo (1)");
        println!("List all todos (2)");
        println!("Check off a todo (3");
        println!("Quit (4)");
        io::stdin().read_line(&mut input).expect("Failed to read"); 
        let choice: i32 = input.trim().parse().expect("Please enter a number");
        if choice == 1 {
            add_to_do(&mut tasks);
        }else if choice == 2{
            view_to_do_list(&mut tasks);
        }else if choice == 3 {
            check_off_task(&mut tasks);
        }else if choice >=4{
            break;
        }
    }
}

fn add_to_do(tasks: &mut Vec<Task>){
    let mut input: String = String::new();
    println!("Please Enter the description for your task: ");
    io::stdin().read_line(&mut input).expect("Failed to read"); 
    let x = Task{
        description: input.trim().to_string(),
        done: false,
    };
    tasks.push(x);
    save_tasks(tasks);

}

fn check_off_task(tasks: &mut Vec<Task>){
    let mut input: String = String::new();
        for (i, task) in tasks.iter().enumerate() {
        println!("{}: {}: {}", i,task.done, task.description);
    }
    println!("Enter the index for the task you want to remove: ");
    io::stdin().read_line(&mut input).expect("Failed to read"); 
    let choice: usize = input.trim().parse().expect("Please enter a number");
    if choice >= tasks.len() {
    println!("Invalid index!");
    return;
    }
    tasks[choice].done = true;
    save_tasks(tasks);
}

fn view_to_do_list(tasks: &mut Vec<Task>) {
    for (i, task) in tasks.iter().enumerate() {
        println!("{}: {}: {}", i,task.done, task.description);
    }
    println!("\nPress enter to go back...");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read");
}
fn clear_screen() {
    print!("\x1B[2J\x1B[1;1H");
}

#[derive(Serialize, Deserialize)]
struct Task {
    description: String,
    done: bool,
}


fn save_tasks(tasks: &Vec<Task>)
{
    let j = serde_json::to_string(tasks).expect("Failed to serialize");
    std::fs::write("tasks.json", j).expect("Failed to write to file");
}

fn load_tasks() -> Vec<Task> 
{
    let data = std::fs::read_to_string("tasks.json").expect("Failed to read file");
    let tasks: Vec<Task> = serde_json::from_str(&data).expect("Failed to deserialize");
    tasks
}
