use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{LookupMap, UnorderedSet, UnorderedMap};
use near_sdk::{
    env, near_bindgen, AccountId, BorshStorageKey, PanicOnDefault
};

use crate::complex_personalized_struct::{ComplexPersonalizedStruct};
use crate::personalized_struct::{PersonalizedStruct};

use crate::errors::{ERR_001, ERR_002, ERR_003, ERR_004};

pub(crate) mod errors;
pub mod ext_interface;
pub mod complex_personalized_struct;
pub mod personalized_struct;
pub mod actions;

/// lib file is used to define the Contract struct, which is the state layer of the smart contract
/// In this file we'll also define the implementations of the methods of the contract struct
/// It is required to always have a initialization function, marked by #[init] ins
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
struct Contract {
    owner: AccountId,
    guardians: UnorderedSet<AccountId>,
    data_structure_map: LookupMap<AccountId, PersonalizedStruct>,
    second_data_structure_map: UnorderedMap<AccountId, ComplexPersonalizedStruct>
}

/// 1. It's a best practice to always use a StorageKey enum to initialize all near_sdk::collections
/// 2. It's important to make the StorageKey enum pub(crate) in case we need to initialize some form
/// of blockchain storage collection in different files.
/// 3. If needed, variants in the enum can also be complex if there's a need to create multiple collections
/// during the execution of the smart contract. That way a parameter can be passed to differentiate between 
/// collections of the same type
#[derive(BorshSerialize, BorshStorageKey)]
pub(crate) enum StorageKey {
    GuardiansSet,
    DataStructureMap,
    SecondDataStructureMap,
    InternalDataStructureMap { struct_name: String },
}

/// 1. There should be no #[near_bindgen] impl sections outside of actions folder,
/// the nly exception is the initialize_contract function, which must be marked with #[init]
/// and is responsible for instantiating the contract into the blockchain's state
/// 2. All other impl sections in this file must be internal to the struct and not exposed to the blockchain,
/// whenever we want to expose a functionality to the end user, it should be declared inside actions
/// 3. #[allow(dead_code)] is used so that rust compiler doesn't throw a unused function warning, this is needed
/// on every #[near_bindgen] impl since the functions are meant to be called on the blockchain and not inside
/// the code itself
#[allow(dead_code)]
#[near_bindgen]
impl Contract {
    #[init]
    pub fn initialize_contract(owner: AccountId) -> Self {
        assert!(!env::state_exists(), "{}", ERR_001);
        Self {
            owner,
            guardians: UnorderedSet::new(StorageKey::GuardiansSet),
            data_structure_map: LookupMap::new(StorageKey::DataStructureMap),
            second_data_structure_map: UnorderedMap::new(StorageKey::SecondDataStructureMap)
        }
    }
}

/// Here all Contract level functions will be defined
impl Contract {
    pub fn internal_change_owner(&mut self, new_owner: AccountId) {
        self.owner = new_owner;
    }

    pub fn internal_renounce_guardianship(&mut self, guardian: AccountId) {
        self.guardians.remove(&guardian);
    }

    pub fn add_personalized_struct(&mut self, data: u128, name: String, user: AccountId) {
        assert!(!self.data_structure_map.contains_key(&user), "{}", ERR_004);
        let new_struct = PersonalizedStruct::new(data, name);
        self.data_structure_map.insert(&user, &new_struct);
    } 
}

/// It is necessary to separate a impl block for helper functions, differen from the block that implements business logic workflows
impl Contract {
    pub fn only_owner(&self) {
        assert_eq!(self.owner, env::predecessor_account_id(), "{}", ERR_002)
    }

    pub fn only_guardians_or_owner(&self) {
        if env::predecessor_account_id() != self.owner {
            assert!(self.guardians.contains(&env::predecessor_account_id()), "{}", ERR_003)
        }
    }
}