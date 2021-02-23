use std::env;

use todors::Parser;

fn main() {
    let parsed = Parser::new(env::args());
    match parsed.parse() {
        Ok(s) => s,
        Err(e) => println!("{}", e),
    };
}
