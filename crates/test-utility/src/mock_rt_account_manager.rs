use crate::{mock_rt_primitives, Account};
use anyhow::{anyhow, Error};

use l1x_sdk_primitives;

pub struct AccountManager;

impl AccountManager {
    pub fn transfer(
        from_account: &mut Account,
        to_account: &mut Account,
        amount: &l1x_sdk_primitives::Balance,
    ) -> Result<(), Error> {
        if !Self::has_sufficient_balance(from_account, amount) {
            return Err(anyhow!("Insufficient balance"));
        }

        let account_balance = from_account
            .balance
            .checked_sub(amount.clone())
            .ok_or_else(|| anyhow!("Error Subtracting balance"))?;

        from_account.balance = account_balance;

        let to_account_balance = to_account
            .balance
            .checked_add(amount.clone())
            .ok_or_else(|| anyhow!("Error Adding balance"))?;

        to_account.balance = to_account_balance;

        Ok(())
    }

    pub fn has_sufficient_balance(
        from_account: &Account,
        amount: &l1x_sdk_primitives::Balance,
    ) -> bool {
        from_account.balance >= *amount
    }
}
