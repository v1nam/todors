use std::env;
mod commands;
mod utils;

use commands::list;
use commands::add;
use commands::remove;
use commands::check;

use utils::styles::Styles;

pub fn help() {
    let help_message = "
Commands:\n
    list -> List all the todos\n
    add [todo name] -> Add a new todo\n
    remove -> select a todo from a menu to remove it\n
    check -> select a todo from a menu to mark it as done, mark it as undone if already done\n
    help -> show this message";
    println!("{}", help_message);
}

pub struct Parser {
    pub arg: String,
    pub value: String,
}

impl Parser {
    pub fn new(mut args: env::Args) -> Parser {
        args.next();

        let arg = match args.next() {
            Some(arg) => arg.to_string(),
            None => "list".to_string(),
        };

        let value = args.collect::<Vec<String>>().join(" ").to_string();

        Parser {
            arg,
            value
        }
    }

    pub fn parse(&self) -> Result<(), String> {
        let styles = Styles::new();
        match &self.arg[..] {
            "list" => Ok(match list::run() {
                Ok(s) => s,
                Err(e) => println!("{}", e),
            }),
            "add" => Ok({
                add::run(&self.value);
            }),
            "remove" => Ok({
                remove::run();
            }),
            "check" => Ok({
                check::run();
            }),
            "help" => Ok({
                help();
            }),
            _ => return Err(format!("{}{}Invalid Argument!{}",
                styles.bold,
                styles.red,
                styles.reset
            )),
        }
    }

}
