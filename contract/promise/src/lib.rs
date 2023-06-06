use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{near_bindgen, Balance, AccountId};
use near_sdk::json_types::U128;

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Contract {
    balance: Balance,
}

impl Default for Contract{
    fn default() -> Self{
        Self {
            balance: 1000000 
        }
    }
}

#[near_bindgen]
impl Contract {
    fn get_balance(&self) -> U128{
        return self.balance.into()
    }

    fn set_balance(&mut self, _balance: U128) -> U128{
        self.balance = _balance.0;

        self.balance.into()
    }
}