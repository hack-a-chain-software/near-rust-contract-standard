/// This file defines callbacks that are performed by the contract to itself
/// More documentation on this process can be found at: https://www.near-sdk.io/cross-contract/callbacks
/// All actions should only handle arguments and call internal functions
/// Actions include both change and view functions
use crate::*;

#[allow(dead_code)]
#[near_bindgen]
impl Contract {
    #[private]
    pub fn callback_function(&mut self, #[callback] val: bool, account: AccountId) {
        if val {
            if let Some(mut old_struct) = self.data_structure_map.get(&account) {
                old_struct.change_data(123);
                self.data_structure_map.insert(&account, &old_struct);
            } else {
                self.data_structure_map.insert(&account, &PersonalizedStruct::new(321, env::signer_account_id()));
            }
        }
    }
}