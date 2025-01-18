#[derive(Debug)]
struct Account{
    id: u32,
    balance: u32,
    owner: String
}

#[derive(Debug)]
struct Bank{
    accounts: Vec<Account>,
}   

impl Bank {
    fn new() -> Self {
        Bank { accounts: Vec::new()}
    }
    fn add_account(&mut self, account: Account) {
        self.accounts.push(account);
    }
    fn total_balance(&self) -> u32 {
        let mut total = 0;
        for account in &self.accounts {
            total += account.balance;
        }
        total
    }
    fn total_summary(&self) -> Vec<String> {
        let mut summary = vec![];
        for account in &self.accounts {
            summary.push(account.summary());
        }
        summary
    }
}

impl Account {
    fn new(_id: u32, _holder: String) -> Self {
        Account {id: _id, balance: 0, owner: _holder}
    }
    fn deposit(&mut self, amount: u32) -> u32{
        self.balance += amount;
        self.balance
    }
    fn withdraw(&mut self, amount: u32) -> u32{
        self.balance -= amount;
        self.balance
    }
    fn summary(&self) -> String {
        format!("{} has a balance: {} ", self.owner, self.balance)
    }
}



fn main() {
    let mut bank = Bank::new();
    let mut account = Account::new(1, "John".to_string());
    println!("Updated balance: {}", account.deposit(500));
    println!("Updated balance: {}", account.withdraw(200));
    println!("{}", account.summary());
    let mut account2 = Account::new(2, "Kevin".to_string());
    let mut account3 = Account::new(3, "Martin".to_string());
    let mut account4 = Account::new(4, "Noah".to_string());

    account2.deposit(100);
    account3.deposit(200);
    account4.deposit(800);
    
    bank.add_account(account);
    bank.add_account(account2);
    bank.add_account(account3);
    bank.add_account(account4);

    println!("{:#?}", bank);
    println!("Total balance: {}", bank.total_balance());
    println!("Total summary: {:?}", bank.total_summary());
}
