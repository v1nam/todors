use std::{fs, env, io};
use std::path::Path;
use std::collections::HashMap;
use std::process;
use serde_json::{json, Value};

pub fn credentials() -> HashMap<String, Value> {
    let mut path = match env::var("HOME") {
        Ok(val) => val,
        Err(_) => ".".to_string(),
    };

    path = format!("{}/todos.json", path);
 
    if Path::new(&path).exists() {
        let file = fs::File::open(&path).unwrap();
        let content: HashMap<String, Value> = serde_json::from_reader(file).unwrap();
        return content;
    }
    
    let mut project_name = String::new();
    let mut project = String::new();


    println!("No project detected, create new one? (y/n)");
    io::stdin()
        .read_line(&mut project_name)
        .expect("Failed to read input");
    match project_name[..].to_lowercase().as_str() {
        "n\n" => process::exit(0),
        _ => {
            println!("Enter name: ");
            io::stdin()
                .read_line(&mut project)
                .expect("Failed to read input");
            if project == "\n".to_string() {
                println!("No name provided");
                process::exit(1);
            }
        },
    }

    let file = fs::File::create(path).unwrap();
    let mut value: HashMap<String, Value> = HashMap::new();
    value.insert("name".to_string(), json!(project.trim()));

    serde_json::to_writer(file, &value).unwrap();
    process::exit(0);
}

pub fn write(creds: HashMap<String, Value>) {
     let mut path = match env::var("HOME") {
        Ok(val) => val,
        Err(_) => ".".to_string(),
    };

    path = format!("{}/todos.json", path);
    let file = fs::File::create(path).unwrap();
    serde_json::to_writer(file, &json!(creds)).unwrap();
}
