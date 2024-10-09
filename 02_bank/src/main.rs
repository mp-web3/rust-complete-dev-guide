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

fn make_and_print_account() -> &Account {
    let account = Account::new(1, String::from("me"));
    println!("{:#?}", account);

    &account
}
fn main() {
    make_and_print_account();

}
