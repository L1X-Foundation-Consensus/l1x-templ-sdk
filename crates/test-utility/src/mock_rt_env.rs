use crate::{
    Account, TestRuntimeVMLogic, TestUtilMockEnvManager, DEFAULT_TEST_RT_ENV_ID,
};

use rand::{thread_rng, Rng};
use sha2::{Digest, Sha256};

const DEFAULT_USER_ACCOUNT_BALANCE: l1x_sdk_primitives::Balance =
    10_000_000_000_000;
const DEFAULT_SYSTEM_ACCOUNT_BALANCE: l1x_sdk_primitives::Balance =
    10_000_000_000_000;

use l1x_sdk_primitives::Address;

#[derive(Debug, Clone)]
pub struct TestUtilMockEnv {
    pub test_vmlogic_id: usize,
}

impl Default for TestUtilMockEnv {
    fn default() -> Self {
        Self { test_vmlogic_id: DEFAULT_TEST_RT_ENV_ID }
    }
}

impl TestUtilMockEnv {
    pub fn new(test_vmlogic_id: usize) -> Self {
        TestUtilMockEnvManager::update_test_env_rt(
            test_vmlogic_id,
            TestRuntimeVMLogic::default(),
        );

        Self { test_vmlogic_id }
    }

    pub fn mock_env_set_caller_address(
        &self,
        caller_address: &l1x_sdk_primitives::Address,
    ) {
        let mut rt_vmlogic =
            TestUtilMockEnvManager::get_test_env_rt(self.test_vmlogic_id)
                .unwrap_or_else(|| panic!("Invalid RunTime ID"));

        rt_vmlogic.caller_address = caller_address.clone();

        TestUtilMockEnvManager::update_test_env_rt(
            self.test_vmlogic_id,
            rt_vmlogic,
        );
    }

    pub fn mock_env_set_contract_owner_address(
        &self,
        contract_owner_address: &l1x_sdk_primitives::Address,
    ) {
        let mut rt_vmlogic =
            TestUtilMockEnvManager::get_test_env_rt(self.test_vmlogic_id)
                .unwrap_or_else(|| panic!("Invalid RunTime ID"));

        rt_vmlogic.contract_owner_address = contract_owner_address.clone();

        TestUtilMockEnvManager::update_test_env_rt(
            self.test_vmlogic_id,
            rt_vmlogic,
        );
    }

    pub fn mock_env_generate_random_address() -> l1x_sdk_primitives::Address {
        let mut rng = thread_rng();
        let mut hasher = Sha256::new();
        let mut random_bytes = [0u8; 32]; // Adjust the size as needed
        rng.fill(&mut random_bytes);
        hasher.update(&random_bytes);
        let hash_result = hasher.finalize();

        let address_bytes: [u8; 20] =
            hash_result[..20].try_into().expect("Slice with incorrect length");
        let address: l1x_sdk_primitives::Address = address_bytes.into();

        address
    }

    pub fn mock_env_generate_user_address(
        user_mnemonics: &[u8],
    ) -> l1x_sdk_primitives::Address {
        let mut hasher = Sha256::new();
        hasher.update(user_mnemonics);
        let hash_result = hasher.finalize();

        let address_bytes: [u8; 20] =
            hash_result[..20].try_into().expect("Slice with incorrect length");
        let address: l1x_sdk_primitives::Address = address_bytes.into();

        address
    }

    pub fn mock_env_set_contract_instance_address(
        &self,
        contract_instance_address: &l1x_sdk_primitives::Address,
    ) {
        let mut rt_vmlogic =
            TestUtilMockEnvManager::get_test_env_rt(self.test_vmlogic_id)
                .expect("Invalid RunTime ID");

        rt_vmlogic.contract_instance_address =
            contract_instance_address.clone();

        TestUtilMockEnvManager::update_test_env_rt(
            self.test_vmlogic_id,
            rt_vmlogic,
        );
    }

    pub fn mock_env_endow_user_account(
        &self,
        user_address: &l1x_sdk_primitives::Address,
        balance: Option<l1x_sdk_primitives::Balance>,
    ) {
        let mut rt_vmlogic =
            TestUtilMockEnvManager::get_test_env_rt(self.test_vmlogic_id)
                .expect("Invalid RunTime ID");

        let mut user_account = Account::new(user_address.clone());

        user_account.balance = balance.unwrap_or(DEFAULT_USER_ACCOUNT_BALANCE);

        rt_vmlogic.account_state.insert(user_address.clone(), user_account);

        TestUtilMockEnvManager::update_test_env_rt(
            self.test_vmlogic_id,
            rt_vmlogic,
        );
    }

    pub fn mock_env_endow_system_account(
        &self,
        system_address: &l1x_sdk_primitives::Address,
        balance: Option<l1x_sdk_primitives::Balance>,
    ) {
        let mut rt_vmlogic =
            TestUtilMockEnvManager::get_test_env_rt(self.test_vmlogic_id)
                .expect("Invalid RunTime ID");

        let mut system_account = Account::new_system(system_address.clone());

        system_account.balance =
            balance.unwrap_or(DEFAULT_USER_ACCOUNT_BALANCE);

        rt_vmlogic.account_state.insert(system_address.clone(), system_account);

        TestUtilMockEnvManager::update_test_env_rt(
            self.test_vmlogic_id,
            rt_vmlogic,
        );
    }
}
