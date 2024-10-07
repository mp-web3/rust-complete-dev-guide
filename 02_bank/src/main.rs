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

fn print_account(account: Account) -> Account {
    println!("{:#?}", account);
    account
}
fn main() {
    let bank = Bank::new();

    let mut account = Account::new(1, String::from("Mattia"));

    account = print_account(account);
    account = print_account(account);
}
