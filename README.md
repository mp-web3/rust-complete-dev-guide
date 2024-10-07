# rust-complete-dev-guide

## Projects

1. deck
2. [bank](#bank)

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

## Create a project in a directory name different from package valid name

To create a Rust project in a directory named `01_deck` while giving the package a valid name, you can use the `--name` flag with `cargo new`. This flag allows you to set the package name separately from the directory name. Here’s how you can do it:

```bash
cargo new "01_deck" --name deck
```

### Explanation:

- `cargo new "01_deck"` creates a new Rust project in the directory named `01_deck`.
- `--name deck` sets the package name to `deck` instead of `01_deck`, which resolves the issue of having an invalid character in the package name.

### Alternative Approach:

If you want the binary to have the name `01_deck`, you can modify the `Cargo.toml` file after creating the project:

1. Create the project using:

   ```bash
   cargo new "01_deck" --name deck
   ```

2. Open the `Cargo.toml` file in the `01_deck` directory and add the following section to specify a custom binary name:

   ```toml
   [[bin]]
   name = "01_deck"
   path = "src/main.rs"
   ```

This way, the package name is still `deck`, but the binary will be named `01_deck` when compiled.

### Summary

The `--name` flag allows you to separate the directory name from the package name. Using this command:

```bash
cargo new "01_deck" --name deck
```

- Directory name: `01_deck`
- Package name: `deck`

And if you want a custom binary name, you can modify the `Cargo.toml` file to set `[[bin]] name = "01_deck"`.

## Installing external Crates

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

## bank

### bank overview

We want to create a project that simulates a bank

Each account will have 3 fields:

- id (unsigned integer)
- balance (integer)
- holder (String)

We will have a struct type representing a bank and another struct type representing an account.

The bank will have tied n number of accounts.
