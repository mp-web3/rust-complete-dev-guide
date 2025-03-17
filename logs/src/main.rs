use std::fs;
use std::io::Error;

fn extract_errors(text: &str) -> Vec<&str> {
    let splitted_text = text.split("\n");
    let mut errors = Vec::new();
    for line in splitted_text {
        if line.starts_with("ERROR") {
            errors.push(line);
        }
    }
    errors
}

fn main() -> Result<(), Error> {
    let text = fs::read_to_string("logs.txt")?;
    let error_logs = extract_errors(&text);
    fs::write("error_logs.txt", error_logs.join("\n"))?;
    Ok(())

    // match fs::read_to_string("logs.txt") {
    //     Ok(text) => {
    //         let error_logs = extract_errors(text.as_str());
    //         println!("Error logs: {:#?}", error_logs);

    //         match fs::write("error_logs.txt", error_logs.join("\n")) {
    //             Ok(_) => println!("Error logs written to file"),
    //             Err(e) => println!("Error: {}", e),
    //         }
    //     }
    //     Err(e) => println!("Error: {}", e),
    // }

    // let text = fs::read_to_string("logs.txt").expect("Failed to read logs.txt");
    // let error_logs = extract_errors(text.as_str());
    // fs::write("error_logs.txt", error_logs.join("\n")).expect("Failed to write error logs to file");
    // let error_logs_file = fs::read_to_string("error_logs.txt").expect("Failed to read error_logs.txt");
    // let error_logs_file_lines = error_logs_file.split("\n");
    // for line in error_logs_file_lines {
    //     println!("{}", line);
    // }
}
