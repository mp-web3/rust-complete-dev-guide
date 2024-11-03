use std::fs;
use::std::io::Error;

fn main() {
//    let text = fs::read_to_string("logs.txt");

//    println!("{:#?}", text);


    match divide(5.0, 0.0) {
        Ok(result ) => {println!("result: {}", result)},
        Err(e) => {println!("error: {}", e)}
    };

    match validate_email("@test.com".to_string()) {
        // To receive an empty value use ".." or "_"
        // you could use any variable name but the standard is ".."
        Ok(..) => {println!("email is valid")},
        Err(e) => {println!("error: {}", e)}
    }
}

fn validate_email(email: String) -> Result<(), Error> {
    if email.contains("@") {
        // When not returning a value you need to return and empty tuple ()
        Ok(())
    } else {
        Err(Error::other("email must have @"))
    }
}

fn divide(a: f64, b: f64) -> Result<f64, Error> {
    if b == 0.0 {
        Err(Error::other("can't divide by 0"))
    } else {
        Ok(a/b)
    }
}