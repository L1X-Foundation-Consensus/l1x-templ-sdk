use borsh::BorshSerialize;

use crate::types;

#[derive(BorshSerialize)]
pub struct ContractCall {
    pub conract_address: types::Address,
    pub method_name: String,
    pub args: Vec<u8>,
    pub fee_limit: u128,
}
