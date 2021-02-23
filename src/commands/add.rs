use std::io;

use crate::commands::base;
use crate::utils::styles::Styles;
use serde_json::json;

pub fn run(arg: &String) {
    let styles = Styles::new();
    let mut creds = base::credentials();

    let mut todo = String::new();

    if arg == "" {
        let mut t = String::new();
        println!("Enter todo name: ");
        io::stdin()
            .read_line(&mut t)
            .expect("Failed to read input");
    
        todo = t.trim().to_string();
    } else {
        todo = arg.to_string();
    }
    let new_todo = json!({"done".to_string(): false,
                          "title".to_string(): todo});

    let mut todos = match creds.remove(&"todos".to_string()) {
        Some(s) => s,
        None => {
            creds.insert("todos".to_string(), json!([new_todo]));
            base::write(creds);
            return ();
        }
    };

    let todo_arr = todos.as_array_mut().unwrap();
    for t in todo_arr.iter() {
        if todo == t["title"].to_string().replace("\"", "") {
            println!("{}Todo with the same name already exists!{}", styles.yellow, styles.reset);
            return ();
        }
    }
    todo_arr.push(json!(new_todo));

    creds.insert("todos".to_string(), json!(todo_arr));

    base::write(creds);
}
