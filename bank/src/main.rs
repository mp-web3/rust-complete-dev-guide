#[derive(Debug)]
struct Account {
    id: u32,
    balance: i32,
    holder: String,
}

impl Account {
    fn new(id: u32, holder: String) -> Self {
        Account {
            id,
            balance: 0,
            holder,
        }
    }

    fn deposit(&mut self, amount: i32) -> i32 {
        self.balance += amount;
        self.balance
    }

    fn withdraw(&mut self, amount: i32) -> i32 {
        self.balance -= amount;
        self.balance
    }

    fn summary(&self) -> String {
        format!("Account ID: {}, Holder: {}, Balance: {}", self.id, self.holder, self.balance)
    }
}

#[derive(Debug)]
struct Bank {
    accounts: Vec<Account>,
}

impl Bank {
    fn new() -> Self {
        Bank { accounts: vec![] }
    }

    fn add_account(&mut self, account: Account) {
        self.accounts.push(account);
    }

    fn total_balance(&self) -> i32 {
        self.accounts.iter().map(|account| account.balance).sum()
    }

    fn summary(&self) -> Vec<String> {
        self.accounts.iter().map(|account| account.summary()).collect()
    }
}

fn print_account(account: &Account) {
    println!("{:#?}", account);
}


fn main() {
    let mut bank = Bank::new();
    let account = Account::new(1, String::from("Mattia"));
    let account2 = Account::new(2, String::from("John"));

    bank.add_account(account);
    bank.add_account(account2);

    bank.accounts[0].deposit(100);
    bank.accounts[0].withdraw(30);

    println!("{}", bank.total_balance());
    println!("{:#?}", bank.summary());
}
