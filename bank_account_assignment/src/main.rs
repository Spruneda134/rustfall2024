mod bank_account;

fn main() {
    let mut bank_acc = bank_account::BankAccount::new(100.0);
    println!("Created a new account with a balance of: ${}", bank_acc.balance());

    bank_acc.deposit(500.0);
    println!("Deposit $500.0, balance: ${}", bank_acc.balance());
    println!("(should be: $600.0)\n");
    
    bank_acc.deposit(-20.0);
    println!("Deposit -$20.0, balance: ${}", bank_acc.balance());
    println!("(should be: $600.0)\n");

    bank_acc.withdraw(30.0);
    println!("Withdraw $30.0, balance: ${}", bank_acc.balance());
    println!("(should be: $570.0)\n");

    bank_acc.withdraw(570.1);
    println!("Withdraw $570.1, balance: ${}", bank_acc.balance());
    println!("(should be: $570.0)\n");

    bank_acc.withdraw(-50.0);
    println!("Withdraw -$50.0, balance: ${}", bank_acc.balance());
    println!("(should be: $570.0)\n");
}
