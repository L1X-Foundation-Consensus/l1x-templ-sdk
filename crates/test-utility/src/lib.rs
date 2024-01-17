mod mock_rt_account_manager;
mod mock_rt_account_state;
mod mock_rt_blockchain_env;
mod mock_rt_env;
pub mod mock_rt_primitives;
mod mock_rt_vmlogic;

use lazy_static::lazy_static;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

pub use mock_rt_account_manager::AccountManager;
pub use mock_rt_account_state::Account;
pub use mock_rt_blockchain_env::BlockchainEnvironment;
pub use mock_rt_env::TestUtilMockEnv;
pub use mock_rt_vmlogic::TestRuntimeVMLogic;

pub const DEFAULT_TEST_RT_ENV_ID: usize = 10usize;

// Shared, thread-safe hash map with Mutex for locking
lazy_static! {
    static ref TEST_VMLOGIC_LUT: Arc<Mutex<HashMap<usize, TestRuntimeVMLogic>>> =
        Arc::new(Mutex::new(HashMap::new()));
}

pub struct TestUtilMockEnvManager;

impl TestUtilMockEnvManager {
    // Function to update a TestUtilMockEnv object in the shared hash map
    pub fn update_test_env_rt(
        test_vmlogic_id: usize,
        rt_vmlogic: TestRuntimeVMLogic,
    ) {
        let mut lut = TEST_VMLOGIC_LUT.lock().unwrap();
        lut.insert(test_vmlogic_id, rt_vmlogic);
    }

    // Function to retrieve a mutable reference to a TestUtilMockEnv object
    pub fn get_test_env_rt(
        test_vmlogic_id: usize,
    ) -> Option<TestRuntimeVMLogic> {
        let lut = TEST_VMLOGIC_LUT.lock().unwrap();
        lut.get(&test_vmlogic_id).cloned()
    }
}
