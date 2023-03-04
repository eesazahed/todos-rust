use std::io;
use colored::Colorize;

fn main() {
  println!("\n{}\n{}\n{}\n", "Todo App".blue().bold(), "By Eesa Zahed".blue().italic(), "https://github.com/eesazahed".cyan().underline());

  let mut todos: Vec<String> = vec![String::new(); 0];

  loop {
    println!("\nWhat would you like to do? \n\n (1) Add a todo \n (2) List all todos \n (3) Update a todo \n (4) Delete a todo \n (5) Quit Application \n");
    
    let input : String = get_input("Type something: ");
    
    match input.trim() {
      "1" => add_todo(&mut todos),
      "2" => list_todos(&todos),
      "3" => update_todo(&mut todos),
      "4" => delete_todo(&mut todos),
      "5" => goodbye(),
       _ => println!("\n{}.", "Invalid input".red())
    }
  }    
}

fn get_input(prompt: &str) -> String {
  let mut buffer : String = String::new();
  println!("\n{}\n", prompt);
  io::stdin().read_line(&mut buffer).expect("There was an error in recieving user input.");
  buffer.trim().to_string()
}

fn add_todo(todos: &mut Vec<String>) {
  let input: String = get_input("Enter a new todo: ");
  todos.push(input);
  let index: usize = todos.len();
  println!("\n{} {}", "Added todo at index".green(), index.to_string().green());
}

fn list_todos(todos: &Vec<String>) {
  if check_if_empty(todos) {
    return;
  }
    
  println!("\n{}\n", "------------------".blue());

  for i in 0..todos.len() {
    println!(" {index}.) {todo}", index = i + 1, todo = todos[i])
  }

  println!("\n{}\n", "------------------".blue());
}

fn update_todo(todos: &mut Vec<String>) {
  if check_if_empty(todos) {
    return;
  }
    
  let index: String = get_input("Enter the index of the todo you would like to update: ");
  let int_index : i32 = check_index(index, todos);

  if int_index == -1 {
    return
  }

  let array_index : usize = (int_index - 1).try_into().unwrap();
    
  let ref old_text = &todos[array_index];
  println!("\nOld todo: {old_text}");
    
  let input : String = get_input("Update todo: ");
    
  todos[array_index] = input;
  println!("\n{} {}", "Updated todo at index", int_index.to_string().green());
}

fn delete_todo(todos: &mut Vec<String>) {
  if check_if_empty(todos) {
    return;
  }

  let index: String = get_input("Enter the index of the todo you would like to delete: ");
  let int_index : i32 = check_index(index, todos);

  if int_index == -1 {
    return;
  }
    
  let array_index : usize = (int_index - 1).try_into().unwrap();

  todos.remove(array_index);
    
  println!("\n{} {}", "Deleted todo at index", int_index.to_string().green());
}

fn check_if_empty(todos: &Vec<String>) -> bool {
  if todos.is_empty() {
    println!("\n{}", "Your todo list is empty.".red());
    return true;
  }

  false
}

fn check_index(index: String, todos: &Vec<String>) -> i32 {
  let int_index = match index.parse::<i32>() {
    Ok(i) => i,
    Err(_e) => {
      println!("\n{}", "Invalid integer.".red());
      return -1;
    },
  };
    
  if int_index < 1 || int_index > (todos.len()).try_into().unwrap() {
    println!("\n{}", "Invalid integer.".red());
    return -1;
  }

  int_index
}

fn goodbye () {
  println!("\n{}\n", "Goodbye 👋".yellow());
  process::exit(0);
}
