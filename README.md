# rust-complete-dev-guidee

## Project description

## Definitions

- Vector: can grow and shrink in size and contains elements
- Array: it is a collection of elements fixed in size
- Trait: a set of functions

> ⚠️ In Rust "variables" are called "bindings"

## Useful

### Formatter

#### {:#}

```
#[derive(Debug)]

// #[derive(Debug)] it is going to be used whenever we try to print `deck` using the Debug Formatter {:?}
    println!("Here is your deck {:#?}", deck);
```

**TERMINAL (cargo run -q):**

```
Here is your deck Deck {
    cards: [
        "Ace of Hearts",
        "Two of Hearts",
        "Three of Hearts",
        "Ace of Spades",
        "Two of Spades",
        "Three of Spades",
        "Ace of Diamonds",
        "Two of Diamonds",
        "Three of Diamonds",
    ],
}
```

## Implementation

Make a list of suits:

- harts
- spades
- clubs
- diamonds

Make a list of values:

- ace
- two
- three
- ...

Create a double nested for looping all possible combinations
