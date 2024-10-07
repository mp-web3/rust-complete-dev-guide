#[derive(Debug)]

struct Deck {
    cards: Vec<String>,
}

// Inherent Implementation of Deck
// Self is a reference to whatever is the parent implementation
impl Deck {
    fn new() -> Self {
        let suits = ["Hearts", "Spades", "Diamonds"];
        let values = ["Ace", "Two", "Three"];
        let mut cards: Vec<String> = vec![];

        // Double nested for loop to get all possible combinations
        for suit in suits {
            for value in values {
                let card: String = format!("{} of {}", value, suit);
                cards.push(card);
            }
        }

        let deck: Deck = Deck { cards };
        return deck;
    }
}

fn main() {
    let deck = Deck::new();

    println!("Here is your deck {:#?}", deck);
}
