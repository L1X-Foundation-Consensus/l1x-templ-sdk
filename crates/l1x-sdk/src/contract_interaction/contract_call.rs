use borsh::BorshSerialize;

use l1x_sdk_primitives::Address;

#[derive(BorshSerialize)]
pub struct ContractCall {
    /// The target contract address
    pub contract_address: Address,
    /// The method should be called in the target contract
    pub method_name: String,
    /// JSON serialized arguments that will be passed to the method.
    pub args: Vec<u8>,
    /// Set `true` if this call should be read-only.
    pub read_only: bool,
    /// Fee limit for the call. Ignored in case of read-only call.
    pub fee_limit: u128,
}
