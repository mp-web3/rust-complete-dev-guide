use rand::{thread_rng, seq::SliceRandom};

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

        // let deck: Deck = Deck { cards };
        // return deck;
        Deck { cards } // This is equal to `return Deck { cards }`
    }

    // Implement a shuffle function which returns cards in random order
    // We will use a crate "package" called `rand`
    // we need to add also `&` to self, meaning that when fn receive the pointer 
    // that we want to modify it has to be mutable
    fn shuffle(&mut self) {
        let mut rng = thread_rng();
        self.cards.shuffle(&mut rng);
    }

    // we are going to use `split_off()` from the rust standard library (std)
    fn deal(&mut self, num_cards: usize) -> Vec<String> {
        self.cards.split_off(
            self.cards.len() - num_cards
        )
    }
}

fn main() {
    let mut deck = Deck::new();

    deck.shuffle();
    let cards = deck.deal(3);
    

    println!("Here is your deck {:#?}", deck);
    println!("Here is your cards (hand) {:#?}", cards);
}
