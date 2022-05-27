/// This file should create the struct that names it and impl it's methods

use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};

#[derive(BorshSerialize, BorshDeserialize)]
pub struct InternalDataStructure {
    pub dummy_data: u128
}

impl InternalDataStructure {
    pub fn new(dummy_data: u128) -> Self {
        Self {
            dummy_data
        }
    }

    pub fn change_dummy_data(&mut self, new_dummy_data: u128) {
        self.dummy_data = new_dummy_data;
    }
}