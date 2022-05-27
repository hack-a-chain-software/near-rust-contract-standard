use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use crate::{AccountId, LookupMap, StorageKey};
use crate::errors::{ERR_101, ERR_102};
use self::internal_data_structure::{InternalDataStructure};

mod internal_data_structure;

/// This file should create the struct and implement all it's methods
/// It's important that the struct is implemented inside an enum, so that 
/// upgrades to the contract can use the same StorageKey and data structure,
/// which makes migration faster, more secure and easier
#[derive(BorshSerialize, BorshDeserialize)]
pub enum ComplexPersonalizedStruct {
    V1(ComplexPersonalizedStructV1)
}

#[allow(unreachable_patterns)]
impl ComplexPersonalizedStruct {
    pub fn new(struct_name: String) -> Self {
        ComplexPersonalizedStruct::V1(ComplexPersonalizedStructV1::new(struct_name))
    }

    pub fn add_to_map(&mut self, account: AccountId, dummy_data: u128) {
        
        match self {
            ComplexPersonalizedStruct::V1(personalized_struct_v1) => {
                personalized_struct_v1.add_to_map(account, dummy_data);
            },
            _ => panic!("{}", ERR_102)
        }

    }

    pub fn change_in_map(&mut self, account: AccountId, new_dummy_data: u128) {

        match self {
            ComplexPersonalizedStruct::V1(personalized_struct_v1) => {
                personalized_struct_v1.add_to_map(account, new_dummy_data);
            },
            _ => panic!("{}", ERR_102)
        }

    }

}

#[derive(BorshSerialize, BorshDeserialize)]
pub struct ComplexPersonalizedStructV1 {
    pub struct_name: String,
    pub account_relationship_map: LookupMap<AccountId, InternalDataStructure>
}

impl ComplexPersonalizedStructV1 {
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
