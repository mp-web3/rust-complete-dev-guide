use std::fs;
mod weird_strings;

fn main() {
    // let text = fs::read_to_string("logs.txt");
    // println!("{:?}", text);

    // Let's refactor this to use a match statement
    match fs::read_to_string("logs.txt") {
        Ok(text) => println!("{}", text.len()),
        Err(e) => println!("Error: {}", e),
    }

    weird_strings::main();

}