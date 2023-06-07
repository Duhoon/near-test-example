// Find all our documentation at https://docs.near.org
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{log, near_bindgen, ext_contract, PanicOnDefault, AccountId, Promise};
use near_sdk::json_types::U128;

use crate::external::*;

mod external;

// Define the default message
const DEFAULT_MESSAGE: &str = "Hello";

// Define the contract structure
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
    message: String,
    account_id_promise: AccountId
}

// Define the default, which automatically initializes the contract
// impl Default for Contract{
//     fn default() -> Self{
//         Self{message: DEFAULT_MESSAGE.to_string()}
//     }
// }

// Implement the contract structure
#[near_bindgen]
impl Contract {
    #[init]
    pub fn init(_message: String, _account_id_promise: AccountId)-> Self {
        Self{
            message: _message,
            account_id_promise: _account_id_promise
        }
    }

    // Public method - returns the greeting saved, defaulting to DEFAULT_MESSAGE
    pub fn get_greeting(&self) -> String {
        return self.message.clone();
    }

    // Public method - accepts a greeting, such as "howdy", and records it
    pub fn set_greeting(&mut self, message: String) {
        log!("Saving greeting {}", message);
        self.message = message;
    }

    pub fn promise_set_balance(&self, _balance: U128) -> Promise {
        promise_contract::ext(self.account_id_promise.clone())
        .extern_set_balance(_balance.clone())
    }
}

/*
 * The rest of this file holds the inline tests for the code above
 * Learn more about Rust tests: https://doc.rust-lang.org/book/ch11-01-writing-tests.html
 */
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_default_greeting() {
        let contract = Contract::default();
        // this test did not call set_greeting so should return the default "Hello" greeting
        assert_eq!(
            contract.get_greeting(),
            "Hello".to_string()
        );
    }

    #[test]
    fn set_then_get_greeting() {
        let mut contract = Contract::default();
        contract.set_greeting("howdy".to_string());
        assert_eq!(
            contract.get_greeting(),
            "howdy".to_string()
        );
    }
}
