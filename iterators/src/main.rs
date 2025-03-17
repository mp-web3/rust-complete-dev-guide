fn print_elements(elements: &[String]) {
    // for element in elements {
    //     println!("{}", element);
    // }
    elements
        .iter()
        .map(|element| format!("{} {}", element, element))
        .for_each(|element| println!("{}", element));
}

fn shorten_string(strings: &mut [String]) {
    strings.iter_mut().for_each(|string| string.truncate(1))
}

fn strings_to_uppercase(strings: &mut [String]) -> Vec<String> {
    strings.iter().map(|string| string.to_uppercase()).collect::<Vec<String>>()
}

fn move_strings(vec_a: Vec<String>, vec_b: &mut Vec<String>) {
    vec_a.into_iter().for_each(|string| vec_b.push(string))
}

fn explode(strings_slices: &[String]) -> Vec<Vec<String>> {
    strings_slices.iter()
        .map(|string | string.chars().map(|char| char.to_string()).collect::<Vec<String>>())
        .collect::<Vec<Vec<String>>>()
}

fn find_color_or(colors_slices: &[String], search_string: &str, fallback_color: &str) -> String {
    colors_slices.iter()
        .find(|color | color.contains(search_string))
        .map_or(String::from(fallback_color), |color| color.to_string())
}
fn main() {
    let mut colors = vec![
        String::from("red"),
        String::from("green"),
        String::from("blue"),
    ];

    // let dish = String::from("ramen");
    // let mut reversed_dish: Vec<&str> = vec![];
    // let mut count = 0;

    // for _char in dish.chars() {
    //     reversed_dish.push(&dish[dish.len() - count - 1 .. dish.len() - count]);
    //     count += 1;
    // }

    // println!("{:?}", reversed_dish.concat());
    // print_elements(&colors[..2]);
    // shorten_string(&mut colors[..2]);
    // println!("{:?}", colors);

    let uppercase_colors = strings_to_uppercase(&mut colors);
    println!("{:?}", uppercase_colors);
    let mut moved_colors: Vec<String> = vec![];
    move_strings(colors, &mut moved_colors);
    println!("{:?}", moved_colors);

    let exploded_colors = explode(&moved_colors);
    println!("{:?}", exploded_colors);

    let found_color = find_color_or(&moved_colors, "blue", "grey");
    println!("{}", found_color);
}