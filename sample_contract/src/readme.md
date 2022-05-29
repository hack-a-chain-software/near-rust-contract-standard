# Project structure
  

### Terminology

application -> The entire smart contract application;  
Contract or contract struct -> Struct that implements the #[near_bindgen] attribute macro;  
internal Struct -> Struct defined in the code that is used inside the Contract's variables;

### Folder organization

The organization of folders inside the project should follow the standard:
  
lib.rs -> Contract definition and implementation of Contract method's that are not exposed to the blockchain. Also implements the Contract initialization_function (only blockchain public function outside of actions).  
  
errors.rs -> Lists the different error messages that the application uses as constants of type &str;

ext_interface.rs -> implements the interfaces for the cross contract calls performed by the application;  
  
events.rs -> implements the functions to log events in the application;
  
utils.rs -> implements helper functions (not methods for any struct);
  
internal_struct.rs (must be named after the internal struct being implemented) -> implements an Internal Struct and its methods (note that internal structs should be wrapped in an enum, as explained in internal_structs.md);  
  
internal_struct folder (must be named after the internal struct being implemented) -> implements an Internal Struct and its methods (note that internal structs should be wrapped in an enum, as explained in internal_structs.md).  
The folder approach is to be used when the struct is complex - meaning that it nests other complex data structures inside of it;  
  
actions folder -> implements all the blockchain public methods of the application. Should be divided into submodules for each user type.