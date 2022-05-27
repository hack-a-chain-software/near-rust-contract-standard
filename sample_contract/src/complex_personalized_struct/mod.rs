use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use crate::{AccountId, LookupMap, StorageKey};
use crate::errors::{ERR_101};
use self::internal_data_structure::{InternalDataStructure};

mod internal_data_structure;

/// This file should create the struct and implement all it's methods
#[derive(BorshSerialize, BorshDeserialize)]
pub struct ComplexPersonalizedStruct {
    pub struct_name: String,
    pub account_relationship_map: LookupMap<AccountId, InternalDataStructure>
}

impl ComplexPersonalizedStruct {
    pub fn new(struct_name: String) -> Self {
        Self {
            struct_name: struct_name.clone(),
            account_relationship_map: LookupMap::new(StorageKey::InternalDataStructureMap { struct_name })
        }
    }

    pub fn add_to_map(&mut self, account: AccountId, dummy_data: u128) {
        
        let new_internal_data_structure = InternalDataStructure::new(dummy_data);
        self.account_relationship_map.insert(&account, &new_internal_data_structure);
    }

    pub fn change_in_map(&mut self, account: AccountId, new_dummy_data: u128) {

        let mut target_struct = self.account_relationship_map.get(&account).expect(ERR_101);
        target_struct.change_dummy_data(new_dummy_data);
        self.account_relationship_map.insert(&account, &target_struct);
    }


}