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

    fn deposit(&mut self, amount: i32) -> i32 {
        self.balance += amount;
        self.balance
    }

    fn withdraw(&mut self, amount: i32) -> i32 {

        self.balance -= amount;
        self.balance
    }

    // fn summary(){}
}

#[derive(Debug)]
struct Bank {
    accounts: Vec<Account>,
}

impl Bank {
    fn new() -> Self {
        Bank { accounts: vec![]}
    }

    fn print_accounts(&mut self) {
        for account in &self.accounts {
            println!("{:#?}", account);
        }
    }

    fn add_account(&mut self, account: Account) {
        self.accounts.push(account);
    }

    fn total_balance(&mut self) -> i32 {
        let mut bank_balance: i32 = 0;
        for account in &self.accounts {
            bank_balance += account.balance;
        }
        println!("Bank Total Balance is: {:#?}", bank_balance);
        bank_balance
    }
}

fn print_account(account: &Account) {
    println!("{:#?}", account);
}

// fn make_and_print_account() {
//     let account = Account::new(1, String::from("me"));
//     println!("{:#?}", account);

// }
fn main() {
    let mut bank = Bank::new();

    let mut account = Account::new(1, String::from("me"));

    account.deposit(500);
    account.withdraw(250);
    bank.add_account(account);
    println!("{:#?}", bank);

}
