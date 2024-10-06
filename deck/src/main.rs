#[derive(Debug)]

struct Deck {
    cards: Vec<String>,
}

fn main() {
    // Possible Suits
    let suits = ["Hearts", "Spades", "Diamonds"];
    // Possible Values
    let values = ["Ace", "Two", "Three"];

    // let card: String;
    let mut cards: Vec<String> = vec![];

    // Double nested for loop to get all possible combinations
    for suit in suits {
        for value in values {
            let card: String = format!("{} of {}", value, suit);
            cards.push(card);
        }
    }

    // Creates an instance of Deck struct, and inside that creates an empty vector `cards`
    // let deck: Deck = Deck { cards: vec![] };
    // Let's create the actual deck
    let deck: Deck = Deck { cards: cards }; // Also let deck = Deck {cards}

    // These 2 ways of creating a vector are equivalent
    // let deck1: Deck = Deck { cards: Vec::new() };

    // #[derive(Debug)] it is going to be used whenever we try to print `deck` using the Debug Formatter {:?}
    println!("Here is your deck {:#?}", deck);
}
