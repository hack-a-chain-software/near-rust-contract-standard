/// This file should create the struct that names it and impl it's methods

use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use crate::errors::{ERR_201};

/// This file should create the struct and implement all it's methods
/// It's important that the struct is implemented inside an enum, so that 
/// upgrades to the contract can use the same StorageKey and data structure,
/// which makes migration faster, more secure and easier
#[derive(BorshSerialize, BorshDeserialize)]
pub enum InternalDataStructure {
    V1(InternalDataStructureV1)
}

#[allow(unreachable_patterns)]
impl InternalDataStructure {
    pub fn new(dummy_data: u128) -> Self {
        InternalDataStructure::V1(InternalDataStructureV1::new(dummy_data))
    }

    pub fn change_dummy_data(&mut self, new_dummy_data: u128) {

        match self {
            InternalDataStructure::V1(personalized_struct_v1) => {
                personalized_struct_v1.change_dummy_data(new_dummy_data)
            },
            _ => panic!("{}", ERR_201)
        }

    }
}

#[derive(BorshSerialize, BorshDeserialize)]
pub struct InternalDataStructureV1 {
    pub dummy_data: u128
}

impl InternalDataStructureV1 {
    pub fn new(dummy_data: u128) -> Self {
        Self {
            dummy_data
        }
    }

    pub fn change_dummy_data(&mut self, new_dummy_data: u128) {
        self.dummy_data = new_dummy_data;
    }
}