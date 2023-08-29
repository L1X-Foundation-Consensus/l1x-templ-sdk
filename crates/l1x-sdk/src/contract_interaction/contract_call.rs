use borsh::BorshSerialize;

use crate::types;

#[derive(BorshSerialize)]
pub struct ContractCall {
    /// The target contract address
    pub conract_address: types::Address,
    /// The method should be called in the target contract
    pub method_name: String,
    /// JSON serialized arguments that will be passed to the method.
    pub args: Vec<u8>,
    /// Fee limit for the call.
    pub fee_limit: u128,
}
