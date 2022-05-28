use near_sdk::{ext_contract};
use near_sdk::json_types::{U64, U128};

/// This file is used to construct the interfaces for cross contract calls.
/// Documentation on cross contract calls and its interfaces can be found at: https://www.near-sdk.io/cross-contract/callbacks
/// This file should include interfaces to call external contracts and also
/// interfaces to perform callbacks on itself
#[ext_contract(ext_calculator)]
trait Calculator {
    fn mult(&self, a: U64, b: U64) -> U128;

    fn sum(&self, a: U128, b: U128) -> U128;
}

/// The interface to callback the contract should always take the name ext_self
#[ext_contract(ext_self)]
trait Callbacks {
    fn callback_function(#[callback] val: bool, account: AccountId);
}