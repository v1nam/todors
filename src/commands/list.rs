use crate::commands::base;
use crate::utils::styles::Styles;

pub fn run() -> Result<(), String> {
    let styles = Styles::new();
    let creds = base::credentials();
    println!("\t{}{}{}{}", styles.bold, styles.blue, creds["name"].as_str().unwrap(), styles.reset);

    let todos = match creds.get(&"todos".to_string()) {
        Some(s) => s,
        None => return Err("You have not added any todos yet 😕".to_string()),
    };

    let todos = todos.as_array().unwrap();
    if todos.len() < 1 {
        return Err("🎊 Epic, there are no todos left!🎉".to_string());
    }

    let mut completed = 0;

    for todo in todos {
        let mark = match todo["done"].as_bool().unwrap() {
            true => {
                completed += 1;
                "✔"
            },
            false => "✗",
        };
        
        println!("{}{} {}{}", styles.green, mark, todo["title"].to_string().replace("\"", ""), styles.reset);
    }
    println!("{} Done, {} Undone.", completed, todos.len() - completed);
    Ok(())
}
