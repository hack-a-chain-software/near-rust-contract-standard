/// This file defines all functions meant to be called be user of regular type
/// All actions should only handle arguments and call internal functions
/// Actions include both change and view functions
use crate::*;

#[allow(dead_code)]
#[near_bindgen]
impl Contract {
    
    pub fn open_function(&mut self, data: u128, name: String) {
        self.add_personalized_struct(data, name, env::predecessor_account_id());
    }  
}