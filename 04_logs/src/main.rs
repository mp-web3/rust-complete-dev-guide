use std::fs;

fn extract_errors(text: &str) -> Vec<String> {
 let split_text = text.split("\n");
 let mut results = vec![];

 for line in split_text {
     if line.starts_with("ERROR") {
         results.push(line.to_string());
     }
 }

    results
}

fn main() {
    let mut error_logs = vec![];

    match fs::read_to_string("logs.txt") {
        Ok(text) => {
            error_logs = extract_errors(text.as_str());
        }
        Err(e) => println!("Error: {}", e),
    }
    println!("{:#?}", error_logs);

}