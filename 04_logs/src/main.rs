use std::fs;
mod weird_strings;

fn extract_errors(text: &str) -> Vec<&str> {
 // Split the text into separate lines
 let split_text = text.split("\n");
 let mut results = vec![];

 for line in split_text {
     if line.starts_with("ERROR") {
         results.push(line);
     }
 }

    results
}

fn main() {
    // let text = fs::read_to_string("logs.txt");
    // println!("{:?}", text);

    // Let's refactor this to use a match statement
    match fs::read_to_string("logs.txt") {
        Ok(text) => {
            let error_logs = extract_errors(text.as_str());
            println!("{:#?}", error_logs);
        }
        Err(e) => println!("Error: {}", e),
    }

    // weird_strings::main();

}