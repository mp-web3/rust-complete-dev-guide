# rust-complete-dev-guidee

## Project description

## Definitions

> ⚠️ In Rust "variables" are called "bindings"

- Vector: can grow and shrink in size and contains elements
- Array: it is a collection of elements fixed in size
- Trait: a set of functions
  > In other languages _Associated function_ is commonly referred to as _class_
- Associated Function: is a function tied to a struct definition
  - To call an associated function e.g. `impl-name::fun-name`
  - Use when you have a functionality not tied to a specific intance
  - E.g. you want to make a brand new copy of a deck that have different initialization points
- Method: a function that operates on a specific instance of a type "struct" or "enum"
  - The argument must be `&self`
  - Use when you want to read or change fields on a specific instance
- Implicit return: rust is going to return the last executed expression inside the function, as long as it doesn't end eith a semicolomn (;)

## Useful

### Installing external Crates

```
cargo add <crate-name>
```

e.g. to install the [rand package](https://crates.io/crates/rand) which has functionalities to generate random numbers.

```
cargo add rand
```

[See rand docs](https://docs.rs/rand/0.8.5/rand/)

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
