fn next_language<'a>(languages: &'a [String], current_language: &str) -> &'a str {
    let mut found = false;

    for lang in languages {
        if found {
            return lang;
        }

        if lang == current_language {
            found = true;
        }
    }

    languages.last().unwrap()
}

fn longest_language<'a>(lang_a: &'a str, lang_b: &'a str) -> &'a str {
    if lang_a.len() > lang_b.len() {
        return lang_a;
    } else {
        return lang_b;
    }
}

fn main() {
    let languages = vec![String::from("rust"), String::from("go"), String::from("typescript")];

    // let result = next_language(&languages, "go");
    let result = longest_language("rust", "typescript");

    println!("{}", result);


}
