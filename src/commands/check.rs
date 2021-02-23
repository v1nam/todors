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

    let check_mark = "✔ ";
    let cross_mark = "✗ ";
    
    let todo_arr = todos.as_array_mut().unwrap();

    if todo_arr.len() < 1 {
       println!("{}No todos found{}", styles.red, styles.reset);
       process::exit(1);
    }

    for todo in todo_arr.iter() {
        items.push(
            button(
                match todo["done"].as_bool().unwrap() {
                    true => format!("{}{}", check_mark, todo["title"].as_str().unwrap()),
                    false => format!("{}{}", cross_mark, todo["title"].as_str().unwrap()),
                }
            )
        );
    }

    items.push(button("Exit"));
    let menu = terminal_menu::menu(items);
    terminal_menu::activate(&menu);
    terminal_menu::wait_for_exit(&menu);
    let selected_name = terminal_menu::selected_item_name(&menu);

    let mut todo_arr: Vec<_> = todo_arr.iter_mut().map(|x| x.as_object_mut().unwrap()).collect();

    
    if selected_name != "Exit" {
        let name = &selected_name[selected_name.find(" ").unwrap()+1..];
        for todo in todo_arr.iter_mut() {
            if todo.get("title").unwrap().to_string() == format!("\"{}\"", name) {
                let done = todo.get_mut("done").unwrap();
                if done == &json!(true) {
                    *done = json!(false);
                } else {
                    *done = json!(true);
                }
            }
        }
        creds.insert("todos".to_string(), json!(todo_arr));
        base::write(creds);
        println!("{}Toggled {}!{}", styles.green, name, styles.reset);
    }
}
