/// Счёт клиента в банке
struct BankAccount {
    balance: u32, // может быть unsigned, так как всегда >= 0
}

struct TransferAccounts {
    from: BankAccount,
    to: BankAccount,
}

/// Операции с банковским счётом
enum Operation {
    Deposit(u32),
    Withdraw(u32),
    Nullify,
}

/// Принимает аккаунт, обновляет его, и возвращает обновлённые данные
fn operate(mut account: BankAccount, operation: Operation) -> BankAccount {
    match operation {
        // выполняем операцию в соответствии с входными данными
        Operation::Deposit(amount) => {
            account.balance += amount;
        }
        Operation::Withdraw(amount) => {
            account.balance -= amount;
        }
        Operation::Nullify => {
            account.balance = 0;
        }
    }

    account
}

fn swap(account: TransferAccounts, amount: u32) -> TransferAccounts {
    if account.from.balance < amount {
        return account;
    }

    TransferAccounts {
        from: operate(account.from, Operation::Withdraw(amount)),
        to: operate(account.to, Operation::Deposit(amount)),
    }
}

fn transfer(sender: &mut BankAccount, receivers: &mut [BankAccount], amount: u32) -> bool {
    if sender.balance < (amount * receivers.len() as u32) {
        return false;
    }

    for account in receivers {
        sender.balance = sender.balance - amount;
        account.balance = account.balance + amount;
    }

    return true;
}

fn test_operate() {
    let rich_account = BankAccount { balance: 100 };
    let poor_account = BankAccount { balance: 10 };

    let rich_account = operate(rich_account, Operation::Deposit(100));
    let poor_account = operate(poor_account, Operation::Withdraw(1));

    println!("rich account balance: {}", rich_account.balance);
    println!("poor account balance: {}", poor_account.balance);

    let nullified_account = operate(rich_account, Operation::Nullify);
    println!("nullified account balance: {}", nullified_account.balance);
}

fn test_swap() {
    let rich_account = BankAccount { balance: 100 };
    let poor_account = BankAccount { balance: 10 };
    let transfer_accounts = TransferAccounts {
        from: rich_account,
        to: poor_account,
    };

    let transfer_accounts = swap(transfer_accounts, 10);
    println!("rich account balance: {}", transfer_accounts.from.balance);
    println!("poor account balance: {}", transfer_accounts.to.balance);
}

pub fn test_transfer() {
    let mut rich_account = BankAccount { balance: 20 };
    let accounts: &mut [BankAccount] = &mut [
        BankAccount { balance: 10 },
        BankAccount { balance: 20 },
        BankAccount { balance: 30 },
    ];

    if !transfer(&mut rich_account, accounts, 10) {
        println!("failed to transfer: not enough money");
        return;
    }

    for account in accounts {
        println!("account balance: {}", account.balance);
    }

    println!("rich_account balance: {}", rich_account.balance);
}
