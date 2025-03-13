use rand::{prelude::SliceRandom, rng};
use std::fmt::format;
#[derive(Debug)]
struct Deck {
    cards: Vec<String>,
}

#[derive(Debug)]
struct Hand {
    cards: Vec<String>,
}

impl Hand {
    fn new(cards: Vec<String>) -> Self {
        Hand { cards: cards }
    }

    fn count(&self) -> usize {
        self.cards.len()
    }
}

// Inherent Implementation of Deck
impl Deck {
    fn new() -> Self {
        let suits = ["Hearts", "Spades", "Diamonds", "Clubs"];
        let values = [
            "Ace", "Two", "Three", "Four", "Five", "Six", "Seven", "Eight", "Nine", "Ten", "Jack",
            "Queen", "King",
        ];
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

    fn deal(&mut self, num_cards: usize) -> Hand {
        let dealt_cards = self.cards.split_off(self.cards.len() - num_cards);
        Hand::new(dealt_cards)
    }
}

fn main() {
    let mut deck = Deck::new();
    let mut hands: Vec<Hand> = vec![];
    let hand_size = 7;
    let num_players = 3;

    
    for _ in 0..num_players {
        deck.shuffle();
        let hand = deck.deal(hand_size);
        hands.push(hand);
        println!("The deck has {:#?} cards", deck.cards.len());
    }


    for (i, hand) in hands.iter().enumerate() {
        println!("Player {} hand: {:#?}", i + 1, hand);
        println!("Player {} has {} cards", i + 1, hand.count());
    }
}
