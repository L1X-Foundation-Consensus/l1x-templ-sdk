use crate::{Account, BlockchainEnvironment, TestUtilMockEnv};
use std::collections::HashMap;

const CURRENT_RUNTIME_VERSION: u64 = 1;

#[derive(Debug, Clone)]
pub struct TestRuntimeVMLogic {
    pub runtime_version: u64,
    pub registers: HashMap<l1x_sdk_primitives::RegisterId, Box<[u8]>>,
    pub emu_storage: HashMap<Vec<u8>, Vec<u8>>,
    pub caller_address: l1x_sdk_primitives::Address,
    pub contract_owner_address: l1x_sdk_primitives::Address,
    pub contract_instance_address: l1x_sdk_primitives::Address,
    pub input: Vec<u8>,
    pub return_data: Vec<u8>,
    pub bc_env: BlockchainEnvironment,
    pub account_state: HashMap<l1x_sdk_primitives::Address, Account>,
}

impl Default for TestRuntimeVMLogic {
    fn default() -> Self {
        Self::new()
    }
}

impl TestRuntimeVMLogic {
    pub fn new() -> Self {
        Self {
            runtime_version: CURRENT_RUNTIME_VERSION,
            registers: HashMap::new(),
            emu_storage: HashMap::new(),
            caller_address: TestUtilMockEnv::mock_env_generate_random_address(),
            contract_owner_address:
                TestUtilMockEnv::mock_env_generate_random_address(),
            contract_instance_address:
                TestUtilMockEnv::mock_env_generate_random_address(),
            input: vec![],
            return_data: vec![],
            bc_env: Default::default(),
            account_state: Default::default(),
        }
    }
}
