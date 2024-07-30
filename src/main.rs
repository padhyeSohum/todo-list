use std::io;

#[derive(Debug)]
struct TodoItem {
    name: String,
    completed: bool,
}

fn main() {
    let mut todo_list: Vec<TodoItem> = Vec::new();

    let mut input: String = String::new();
    println!("Enter a command word: add, remove, list, complete, or quit");
    input = ask_for_input(input);

    while input != "quit" {
        if input == "list" {
            todo_list = list_todo_list(todo_list);
        }
        else if input == "complete" {
            println!("Which task do you want to mark as complete?");
            input = ask_for_input(input);
            let num: usize = input.parse::<usize>().unwrap();
            todo_list = complete_task(num, todo_list);
        }
        else if input == "remove" {
            input = ask_for_input(input);
            let num: usize = input.parse::<usize>().unwrap();
            todo_list = remove_item(num, todo_list);
        }
        else if input == "add" {
            input = ask_for_input(input);
            (input, todo_list) = add_item(input, todo_list);
        }
        else {
            println!("Please input one of the command words.");
        }

        println!("Enter a command word: add, remove, list, complete, or quit");
        input = ask_for_input(input);
    }
}

fn ask_for_input(mut input: String) -> String {
    input.clear();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    input = input.trim().to_string();
    return input;
}

fn add_item(name: String, mut todo_list: Vec<TodoItem>) -> (String, Vec<TodoItem>) {
    todo_list.push(TodoItem{name: name.clone(), completed: false});
    return (name, todo_list);
}

fn remove_item(number: usize, mut todo_list: Vec<TodoItem>) -> Vec<TodoItem> {
    if todo_list.len() == 0 {
        println!("Add items to your todo list first!");
    }
    else if number < 1 || number > todo_list.len() {
        println!("Please enter a valid number!");
    }
    else {
        println!("You have removed the item: {}", todo_list[number - 1].name);
        todo_list.remove(number - 1);
    }
    return todo_list;
}

fn list_todo_list(todo_list: Vec<TodoItem>) -> Vec<TodoItem> {
    if todo_list.len() == 0 {
        println!("Add items to your todo list!");
        return todo_list;
    }

    println!("YOUR TODO LIST:");
    for i in 0..todo_list.len() {
        println!("#-------------#");
        println!("{}. {}", i + 1, todo_list[i].name);
        if todo_list[i].completed {
            println!("COMPLETED");
        } else {
            println!("IN PROGRESS");
        }
        println!("#-------------#\n");
    }
    return todo_list;
}

fn complete_task(number: usize, mut todo_list: Vec<TodoItem>) -> Vec<TodoItem> {
    todo_list[number - 1].completed = true;
    return todo_list;
}