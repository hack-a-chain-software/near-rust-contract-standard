# Standard specification and main concepts

### NEAR protocol smart contract overview
  
Smart contracts in the NEAR protocol are built using a struct with the attribute macro #[near_bindgen] in its definition.  
The public methods of the smart contract (methods that can be called through a blockchain transaction) have to be implemented inside impl sections of the Struct that was decorated with #[near_bindgen]. This impl sections must also use the #[near_bindgen] macro.
  
Example:
  
 ```
 #[near_bindgen]
 #[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
 struct Contract {
 	pub owner: AccountId,
    pub data: u128
 }
 
 /// this methods cannot be called through a blockchain transaction
 impl Contract {
 	pub fn change_data(&mut self, new_data: u128) {
    	self.data = new_data;
    }
 }
 
 /// this methods can be called through blockchain transactions
 #[near_bindgen]
 impl Contract {
 	pub fn blockchain_public_change_data(&mut self, new_data: U128) {
    	self.change_data(new_data.0);
    }
 }
 ```
 
### Code organization
   
Considering this property of NEAR contracts, the code should be organized in a way that allows us to differentiate between the internal logic of the contract and the blockchain calling logic.  

For that to happen we must separate the code in 2 parts: Logic and Actions:

#### Logic
Logic refers to everything in the contract's data structure and all its internal functions, which means: 
(1) the Contract struct and all its methods that are not exposed in the blockchain (lib.rs);
(2) all intenal data structures (enums and structs) and all its methods (each data structure must be created in a separate file);
(3) all interfaces for external contract calls (ext_interface.rs);
(4) documentation of all error messages (errors.rs)
  
### Actions
Actions refers to all functions of the Contract struct that can be called through blockchain transactions.  
We must separate actions per type of user, some actions are meant to be called by any user, some are meant to be called just by the owner and some are meant to be called by the contract itself.  
Depending on the application, there might by many different user categorizations, for better code organization, we must divide the actions in different files, according to the user type that is meant to call them.  