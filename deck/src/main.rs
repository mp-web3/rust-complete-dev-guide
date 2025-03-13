use std::fmt::format;
use rand::{rng, prelude::SliceRandom};
#[derive(Debug)]

struct Deck {
    cards: Vec<String>,
}

// Inherent Implementation of Deck
impl Deck {
    fn new() -> Self {
    
    let suits = ["Hearts", "Spades", "Diamonds"];
    let values = ["Ace", "Two", "Three"];
    let mut cards: Vec<String> = vec![];
    
    for suit in suits {
        for value in values {
            let card = format!("{} of {}", value, suit);
            cards.push(card);
        }
    }
    
    let deck = Deck { cards: cards };
    return deck;
    }

    fn shuffle(&mut self) {
        let mut rng = rng();
        self.cards.shuffle(&mut rng);
    }

    fn deal(&mut self, num_cards:usize) -> Vec<String> {
        self.cards.split_off(self.cards.len() - num_cards)
    }
}
fn main() {
    
    let mut deck = Deck::new();
    // deck.shuffle();
    let hand = deck.deal(3);

    println!("Here is your deck {:#?}", deck);
    println!("Here is your hand {:#?}", hand);
    println!("The deck has {:#?} cards", deck.cards.len());
}
