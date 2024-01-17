use l1x_sdk_primitives::{Address, Balance, Nonce};

#[derive(Debug, Clone)]
pub enum AccountType {
    System,
    User,
}

impl AccountType {
    pub fn as_str(&self) -> &'static str {
        match *self {
            AccountType::System => "System",
            AccountType::User => "User",
        }
    }
}

#[derive(Debug, Clone)]
pub struct Account {
    pub address: Address,
    pub balance: Balance,
    pub nonce: Nonce,
    pub account_type: AccountType,
}

impl Account {
    pub fn new(address: Address) -> Account {
        Account {
            address,
            balance: 0,
            nonce: 0,
            account_type: AccountType::User,
        }
    }

    pub fn new_system(address: Address) -> Account {
        Account {
            address,
            balance: 0,
            nonce: 0,
            account_type: AccountType::System,
        }
    }
}
