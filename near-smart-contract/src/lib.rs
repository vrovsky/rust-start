use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{
    env, near_bindgen, PanicOnDefault
};

#[near_bindgen]
#[derive(BorshSerialize, BorshDeserialize, PanicOnDefault)]
pub struct Contract;

#[near_bindgen]
impl Contract {
    pub fn hello() {
        env::log_str("Hello world".as_ref());
    }
}