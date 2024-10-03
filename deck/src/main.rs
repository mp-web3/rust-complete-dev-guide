struct Deck {
    cards: Vec<String>,
}

fn main() {
    // Creates an instance of Deck struct, and inside that creates an empty vector `cards`
    let deck: Deck = Deck { cards: vec![] };
    // These 2 ways of creating a vector are equivalent
    let deck1: Deck = Deck { cards: Vec::new() };

    println!("Here is your deck {}", deck);
}
