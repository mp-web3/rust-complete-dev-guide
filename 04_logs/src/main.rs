use std::fs;

fn extract_errors(text: &str) -> Vec<&str> {
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

    match fs::read_to_string("logs.txt") {
        Ok(text) => {

            let error_logs = extract_errors(text.as_str());

            match fs::write("errors.txt", error_logs.join("\n")) {
                Ok(..) => {
                    print!("Errors written to file")
                }
                Err(e) => {
                    println!("Error writing file: {:?}", e)
                }
            }
        }
        Err(e) => {
            println!("Error reading file: {:?}", e)
        }
    }

}