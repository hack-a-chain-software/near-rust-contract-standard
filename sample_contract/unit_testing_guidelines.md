# Unit testing guidelines

Performing unit testing using this architecture is made very simple.  
Unit tests should test all methods defined in all of the files and check that they are performing their correct functionality.  
Each test tests only one method of the application and nothing else. There's a discussion in the testing community regarding tests for private functions, in this spec we mantain that private functions in the Rust sense (that means, functions that cannot be called outside the scope of the impl section in which they're defined) don't need to be tested. However, every method from the Contract and whichever enum or struct you create in the project must be tested, even if it is not exposed on the blockchain.  
  
When testing methods that call other methods, it's only required to test the functionality implemented by the higher level function.  
If all it does is call another function, you should mock the function called with a monkey patch, to remove the actual functionality and only assert that the function has been called, it is not necessary to make any assertions about the functioning of the inner function that has already been tested.  
If a function calls another function somewhere, but also implements logic you should make assertions about the logic that it implements and also assert that the lower level function is being called.  
If you see a function that calls other functions and that apparently cannot be tested without making assertions about the inner state of the lower level functions or that has functionality that won't work if you monkey patch the inner functions that is a sign that you should break the function down into smaller functions that are called separatelly.   
  
### Testing without mocking the blockchain  
  
Unit tests ideally don't need to mock the blockchain features, t

### Differences between unit testing and integration testing in the NEAR protocol  

Building integration tests for the near protocol requires us to actually run a local blockchain in our machine (or use a ci testnet) to deploy a contract and perform operations on it, the same way as the end user would.  
  
That's not the goal for unit testing. Unit tests exist for us to ensure that each function is doing precisely what it is intended to do and nothing else. Most functions are internal to the Contract struct or its internal data structures and might therefore be tested without the need to mock the blockchain environment.
However, in the case of functions that utilize env::predecessor_account_id(), env::attached_deposit() and other blockchain variables, we need to mock the blockchain to be able to unit test them.  
Ideally, the only functionas that should utilise the near_sdk::env variables are the action functions. In some rare cases they might be needed inside internal functions as well. In case the function you're testing requires near_sdk::env variables, you should apply the following context mocking framework:  
  
```
/// This function can actually implement more or less blockchain attributes as variables, depending on the testing needs of the application
pub fn get_context(input: Vec<u8>, is_view: bool, attached_deposit: u128, account_balance: u128, signer_id: AccountId) -> VMContext {
    VMContext {
        current_account_id: CONTRACT_ACCOUNT.to_string(),
        signer_account_id: signer_id.clone(),
        signer_account_pk: vec![0, 1, 2],
        predecessor_account_id: signer_id.clone(),
        input,
        block_index: 0,
        block_timestamp: 0,
        account_balance,
        account_locked_balance: 0,
        storage_usage: 0,
        attached_deposit,
        prepaid_gas: 10u64.pow(18),
        random_seed: vec![0, 1, 2],
        is_view,
        output_data_receivers: vec![],
        epoch_height: 19,
    }
  }

/// to run the test we first setup the blockchain mock and then perform the function call
#[test]
fn test_new() {
  let mut context = get_context(vec!(), false, 0, 0, OWNER_ACCOUNT.to_string()); 
  testing_env!(context);
  
  let contract = Contract::new(OWNER_ACCOUNT.to_string(), TOTAL_SUPPLY.into(), get_test_meta());
  let contract_metadata = contract.metadata.get().unwrap();

  assert_eq!(contract.ft_total_supply().0, TOTAL_SUPPLY);
  assert_eq!(contract.ft_balance_of( ValidAccountId::try_from(OWNER_ACCOUNT).unwrap() ).0, TOTAL_SUPPLY);
  assert_eq!(contract_metadata.spec, get_test_meta().spec)
}

```
  
Note that even though we're mocking the blockchain, we're only calling one function of the contract in the test. Unit tests should never call more than one contract function.  
If the function you're testing requires previous setup of the Contract this setup should be done manually and not through the use of different function calls.  
Remember: If you're calling more than one function from your application you're doing an integration test and not a unit test.
