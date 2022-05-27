use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};

use crate::errors::{ERR_201};
/// This file should create the struct and implement all it's methods
/// It's important that the struct is implemented inside an enum, so that 
/// upgrades to the contract can use the same StorageKey and data structure,
/// which makes migration faster, more secure and easier
#[derive(BorshSerialize, BorshDeserialize)]
pub enum PersonalizedStruct {
    V1(PersonalizedStructV1)
}

#[allow(unreachable_patterns)]
impl PersonalizedStruct {
    pub fn new(data: u128, name: String) -> Self {
        PersonalizedStruct::V1(PersonalizedStructV1::new(data, name))
    }

    pub fn change_data(&mut self, new_data: u128) -> u128 {
        
        match self {
            PersonalizedStruct::V1(personalized_struct_v1) => {
                personalized_struct_v1.change_data(new_data)
            },
            _ => panic!("{}", ERR_201)
        }
        
    }

}

#[derive(BorshSerialize, BorshDeserialize)]
pub struct PersonalizedStructV1 {
    pub data: u128,
    pub last_data: Option<u128>,
    pub name: String
}

impl PersonalizedStructV1 {
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