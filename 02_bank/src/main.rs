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

#[derive(Debug)]
struct Bank {
    accounts: Vec<Account>,
}

impl Bank {
    fn new() -> Self {
        Bank { accounts: vec![]}
    }
}

fn print_account(account: &Account) {
    println!("{:#?}", account);
}
// Takes in as an argument a mutable reference of type Account
fn change_account(account: &mut Account) {
    account.balance = 10;
}
fn main() {

    let mut account = Account::new(1, String::from("me"));

    change_account(&mut account);

    println!("{:#?}", account);
}
