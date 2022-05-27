/// This file defines all functions meant to be called be user of type owner
/// All actions should only handle arguments, perform user validations and call internal functions
/// Actions include both change and view functions
use crate::*;
use near_sdk::utils::{assert_one_yocto};

#[allow(dead_code)]
#[near_bindgen]
impl Contract {
    #[payable]
    pub fn owner_private_function(&mut self, new_owner: AccountId) {
        self.only_owner();
        assert_one_yocto();
        self.internal_change_owner(new_owner);
    }  
}