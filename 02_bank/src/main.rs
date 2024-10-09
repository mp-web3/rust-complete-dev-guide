#[derive(Debug)]
struct Account {
    id: u32,
    balance: f64,
    holder: String
}

impl Account {
    fn new(id: u32, holder: String) -> Self {
        Account {
            id,
            holder,
            balance: 0.0
        }
    }

    fn deposit(&mut self, amount: f64) {
        self.balance += amount;
    }

    fn withdraw(&mut self, amount: f64) {

        self.balance -= amount;
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

    fn total_balance(&mut self) -> f64{
        let mut bank_balance= 0.0;
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

    let mut account = Account::new(0, String::from("Mattia"));
    print_account(&account);
    // Deposit
    account.deposit(100.5);
    print_account(&account);
    // Succesful Withdraw
    account.withdraw(50.5);
    print_account(&account);

    bank.add_account(account);
    bank.print_accounts();

    let mut account1 = Account::new(1, String::from("Patrick"));
    bank.add_account(account1);

    // Since the "account" has been moved in "bank", if we want to modify the account balance we need
    // to access it through bank

    bank.accounts[0].deposit(33.0);
    bank.accounts[1].deposit(560.90);
    bank.print_accounts();


    bank.total_balance();
}
