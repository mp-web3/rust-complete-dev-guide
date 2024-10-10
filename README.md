# rust-complete-dev-guide

## Important Links

- Full [video explanation](https://www.udemy.com/course/rust-the-complete-developers-guide/learn/lecture/44784613) with examples of Ownership and Moves

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

## 10 Rules of Ownership, Borrowing and Lifetimes

1. Every value is owned by a single variable, struct, vector, argument, etc. at a time
2. Reassigning the value to another variable, passing it to a function, putting it into a vector, etc. _moves_ the value. The old variable can't be used anymore!
3. You can create many read-only references to a value that exist at the same time
4. You can't move a value while a ref to the value exists
5. You can make a writeable (mutable) reference to a value only if there are no read-only references currently in use. One mutable ref to a value can exist at a time
6. You can't mutate a value through the owner when any ref (mutable or immutable) to the value exists
7. Some types of values are copied instead of moved (numbers, bools, chars, arrays/tuples with copyable elements)
8. When a variable goes out of scope, the value owned by it is dropped (cleaned up in memory)
9. Values can't be dropped if there are still active references to it
10. References to a value can't outlive the value they refer to

## Ownership

## Borrowing

> References allow us to look at a value without moving it

There are different type of references we can create:

- read-only/immutable ref
- writeable/mutable refs

### Read-only/Immutable references

3. You can create many read-only references to a value that exist at the same time
4. You can't move a value while a ref to the value exists

```
#[derive(Debug)]
struct Account {
    id: u32,
    balance: i32,
    holder: String
}

impl Account {
    fn new(id: u32, holder: String) -> Self {
        Account {
            id,
            holder,
            balance: 0
        }
    }
}

fn main() {
  let account = Account::new(1, String::from("Mattia"));

  let account_ref = &account;

  // If I try to change the value account or account fields it will fail
  // The following will vot compile!!!

  account_ref.id = 2; // This does not compile
}

```

### Writeable/Mutable references

5. You can make a writeable (mutable) reference to a value only if there are no read-only references currently in use. One mutable ref to a value can exist at a time
6. You can't mutate a value through the owner when any ref (mutable or immutable) to the value exists

> Mutable refs allow us to read or change a value without moving it

The following will compile correctly because we are using a mutable variable for account and reassigning the modified value to account, BUT is very manual and tedious.

```
// modify the account balance and returns account
fn change_account(mut account: Account) -> Account {
  account.balance = 10;
  account
}

fn main() {
  // 1. Instantiate a mutable account struct
  let mut account = Account::new(
    1,
    String::from("me")
  );

  // 2. Pass the instance of account to the change_account() function
  // and assign the returned value to account variable
  account = change_account(account);
  println!("{:#?}, account");

}
```

Since _mutable refs_ allow us to read or change a value withouut moving it, a better way to modify the value of account.balance is the following

```
// Takes in as an argument a mutable reference of type Account
fn change_account(account: &mut Account) {
    account.balance = 10;
}

fn main() {

  let mut account = Account::new(1, String::from("me"));

  change_account(&mut account);

  println!("{:#?}", account);
}
```

### Copiable values

7. Some types of values are copied instead of moved (numbers, bools, chars, arrays/tuples with copyable elements)

Copyable values:

- numbers (All)
- bools
- chars
- arrays
- tuples

```
fn main() {
  let num = 5:
  let other_num = num;

  println!("{} {}", num, other_num)
}
```

## Lifetimes

The word "Lifetimes" by itself, refers to how long an owner or reference to a value exists
"Generic Lifetimes"/"Lifetimes Annotations" is Extra syntax added in to clarify reletionships between different lifetimes

8. When a variable goes out of scope, the value owned by it is dropped (cleaned up in memory)
9. Values can't be dropped if there are still active references to it
10. References to a value can't outlive the value they refer to

## bank

## Implementation

> With every function we write, we need to think about whether we are receiving values or refs
> With every data structure we define, we need to think about whether we are storing values or refs

### Bank Methods

<!-- prettier-ignore-start -->

| Description                                            | Method or Assoc. Func? | Name     | Args                  | Returns |
|--------------------------------------------------------|-----------------------|----------|----------------------|---------|
| Create a 'Bank' instance                                | Assoc. Func           | new()    | -                    | Bank    |
| Add an account to the list of accounts                  | Method                | add_account() | account: Account    |         |
| Calculate the total balance of all accounts             | Method                | total_balance() | -                  | i32     |
| Create a `Vec` containing the summaries of all accounts | Method                | summary() | -                   | Vec<String> |

### Account Methods

| Description                                            | Method or Assoc. Func? | Name      | Args                      | Returns  |
|--------------------------------------------------------|-----------------------|-----------|--------------------------|----------|
| Create an 'Account' instance                            | Assoc. Func           | new()     | id: u32, holder: String   | Account  |
| Add the given amount of money to the account's 'balance'| Method                | deposit()  | amount: i32              |      i32    |
| Remove the given amount of money from the account's 'balance'. | Method           | withdraw() | amount: i32              |     i32     |
| Create an account summary as a string and return it     | Method                | summary()  | -                        | String   |

<!-- prettier-ignore-end -->

### bank overview

We want to create a project that simulates a bank

Each account will have 3 fields:

- id (unsigned integer)
- balance (integer)
- holder (String)

We will have a struct type representing a bank and another struct type representing an account.

The bank will have tied n number of accounts.
