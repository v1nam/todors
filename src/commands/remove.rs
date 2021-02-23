use std::process;

use crate::commands::base;
use crate::utils::styles::Styles;
use terminal_menu::{TerminalMenuItem, button};
use serde_json::json;

pub fn run() {
    let styles = Styles::new();
    let mut items: Vec<TerminalMenuItem> = Vec::new();
    let mut creds = base::credentials();

    let mut todos = match creds.remove(&"todos".to_string()) {
        Some(s) => s,
        None => {
            println!("{}No todos found{}", styles.red, styles.reset);
            process::exit(1);
        }
    };

    let todo_arr = todos.as_array_mut().unwrap();

    if todo_arr.len() < 1 {
        println!("{}No todos found{}", styles.red, styles.reset);
        process::exit(1);
    }

    for todo in todo_arr.iter() {
        items.push(
            button(todo["title"].to_string())
        );
    }
    
    items.push(button("Exit"));
    let menu = terminal_menu::menu(items);
    terminal_menu::activate(&menu);
    terminal_menu::wait_for_exit(&menu);
    let selected_name = terminal_menu::selected_item_name(&menu);

    if selected_name != "Exit" {
        println!("{}Removed {}{}", styles.yellow, selected_name, styles.reset);
    }
    todo_arr.retain(|x| x["title"].to_string() != selected_name.to_string());

    creds.insert("todos".to_string(), json!(todo_arr));
    base::write(creds);
}
