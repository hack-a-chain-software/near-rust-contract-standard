# Code architecture and organization standard for NEAR protocol rust smart contract

### Goals

The goal of this specification is to enforce best practices on designing rust smart contracts for the NEAR protocol for the Hack-a-Chain team and for anyone that aims to build high quality easy to maintain and upgradable smart contract code. 

### Architecture

The architecture of the contract can be found inside the sample_contract folder.  
This spec is not concerned with your entire repo organization and architecture, just with the code that lives inside each rust crate.  
  
The main feature of this architecture is to fully separate the contract logic inside of a Contract struct and its internal method implementations from the blockchain accessible functions, which are implemented inside the actions module.  
  
### Explanation
  
The explanation of the entire architecture can be explored inside the md files in the sample_contract folder. The readme.md in sample_contract explains the overall visualization and architecture goals.  
  
Further md files specific the choices made inside of each rust module that composes the application.

### Contributions
  
Contributions are open to anyone that wants to help to improve the quality of rust smart contracts written for the NEAR protocol. Please raise github issues or post comments for errors and new ideas that you find!