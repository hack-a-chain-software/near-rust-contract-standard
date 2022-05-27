/// This file defines all functions meant to be called be user of type owner
/// All actions should only handle arguments, perform user validations and call internal functions
/// Actions include both change and view functions
use crate::*;
use near_sdk::utils::{assert_one_yocto};

#[allow(dead_code)]
#[near_bindgen]
impl Contract {
    #[payable]
    pub fn renounce_guardian_privilege(&mut self) {
        self.only_guardians_or_owner();
        assert_one_yocto();
        self.internal_renounce_guardianship(env::predecessor_account_id());
    }  
}