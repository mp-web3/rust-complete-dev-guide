use std::{fs, string};

fn string_test(
    a: String,
    b: String,
    c: &String,
    d: &str,
    e: &str
) {
    println!("{}", a);
    println!("{}", b);
    println!("{}", c);
    println!("{}", d);
    println!("{}", e);
}

pub fn main() {

    string_test(
        String::from("red"),
        "red".to_string(),
        &String::from("blue"),
        "green",
        String::from("green").as_str()
    );

}