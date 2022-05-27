use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};

/// This file should create the struct and implement all it's methods
#[derive(BorshSerialize, BorshDeserialize)]
pub struct PersonalizedStruct {
    pub data: u128,
    pub last_data: Option<u128>,
    pub name: String
}

impl PersonalizedStruct {
    pub fn new(data: u128, name: String) -> Self {
        Self {
            data,
            last_data: None,
            name
        }
    }

    pub fn change_data(&mut self, new_data: u128) -> u128 {
        
        let old_data = self.data;
        self.last_data = Some(old_data);
        self.data = new_data;
        old_data

    }

}