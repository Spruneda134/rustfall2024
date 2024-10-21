#[derive(Debug)]
pub struct BankAccount {
    balance: f64,
}

impl BankAccount {
    pub fn new(initial_balance: f64) -> BankAccount {
        if initial_balance > 0.0 {
            BankAccount {
                balance: initial_balance,
            }
        } else {
            BankAccount {
                balance: 0.0,
            }
        }
    }
    

    pub fn deposit(&mut self, amount: f64) {
        // Implement this method
        if amount > 0.0 {
            self.balance += amount;
        }
    }

    pub fn withdraw(&mut self, amount: f64) {
        // Implement this method
        if amount > 0.0 && amount <= self.balance {
            self.balance -= amount;
        }
    }

    pub fn balance(&self) -> f64 {
        // Implement this method
        return self.balance;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_account() {
        let test_account = BankAccount::new(100.0);
        assert_eq!(test_account.balance(), 100.0);

        let test_account2 = BankAccount::new(-100.0);
        assert_eq!(test_account2.balance(), 0.0);
    }

    #[test]
    fn test_deposit() {
        let mut test_account = BankAccount::new(100.0);
        test_account.deposit(50.0);
        assert_eq!(test_account.balance(), 150.0);

        test_account.deposit(-50.0);
        assert_eq!(test_account.balance(), 150.0);
    }

    #[test]
    fn test_withdraw() {
        let mut test_account = BankAccount::new(100.0);

        test_account.withdraw(30.0);
        assert_eq!(test_account.balance(), 70.0);

        test_account.withdraw(100.0);
        assert_eq!(test_account.balance(), 70.0);

        test_account.withdraw(-10.0);
        assert_eq!(test_account.balance(), 70.0);
    }

    #[test]
    fn everything() {
        let mut test_account = BankAccount::new(100.0);
        test_account.deposit(50.5);
        test_account.withdraw(20.2);
        assert_eq!(test_account.balance(), 130.3);
    }
}
