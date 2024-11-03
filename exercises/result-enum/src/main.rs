use::std::io::Error;

// Todo: add in a return type
fn validate_ingredients(ingredients: &Vec<String>) -> Result<(), Error> {
    if ingredients.len() > 3 {
        // Todo: make it clear that this is an error!
        let ingredients_plus = ingredients.len() - 3;
        Err(Error::new(std::io::ErrorKind::Other, format!("Too many ingredients! Remove {}", ingredients_plus)))
    } else {
        // Todo: make that ingredients have passed validation
        // Note that we don't have a value to return so we return an empty tuple ()
        Ok(())
    }
}


fn main() {
    let ingredients = vec![
        String::from("Cheese"),
        String::from("Tomatoes"),
        // String::from("Peppers"),
        "Olives".to_string()
    ];


    // Todo: validation is an operation that might succeed or fail.
    // Print out a success or fail message based on whether it passes validation
    match validate_ingredients(&ingredients) {
        Ok(_) => println!("Ingredients are valid."),
        Err(e) => println!("Validation failed: {}", e),
    }
}
