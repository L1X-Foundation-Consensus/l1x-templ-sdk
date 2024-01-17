use std::{
    collections::{hash_map::Entry, HashMap},
    slice,
};

use l1x_test_utility::{
    AccountManager, TestUtilMockEnv, TestUtilMockEnvManager,
    DEFAULT_TEST_RT_ENV_ID,
};

pub struct TestRuntimeVMApi;

impl TestRuntimeVMApi {
    /*
     * Register API
     */
    pub fn read_register(
        register_id: l1x_sdk_primitives::RegisterId,
        result_addr: l1x_sdk_primitives::MemoryAddress,
    ) {
        log::debug!("ARGS :: Test Runtime for read_register()");
        log::debug!(
            "RegisterId: {:x}, MemoryAddress: {:x}",
            register_id,
            result_addr
        );

        if result_addr == 0 {
            panic!("result_addr is null");
        }

        let rt_vmlogic =
            TestUtilMockEnvManager::get_test_env_rt(DEFAULT_TEST_RT_ENV_ID)
                .unwrap_or_else(|| panic!("Invalid Test Runtime Context"));

        if let Some(data) = rt_vmlogic.registers.get(&register_id) {
            let data_len: usize = data.len();

            let dest_ptr = result_addr as *mut u8;

            // Safety: We need to ensure that the destination pointer is
            // valid and the memory regions do not overlap.
            unsafe {
                // Check if the destination pointer is valid for `len` bytes.
                if dest_ptr.is_null()
                    || dest_ptr.add(data_len) as u64 <= result_addr
                {
                    panic!("Invalid destination address or length");
                }

                // Perform the copy operation
                slice::from_raw_parts_mut(dest_ptr, data_len)
                    .copy_from_slice(data);
            }
        } else {
            panic!("register_id is Invalid");
        }
    }

    pub fn register_len(register_id: l1x_sdk_primitives::RegisterId) -> u64 {
        log::debug!("ARGS :: Test Runtime for register_len()");
        log::debug!("RegisterId: {:x}", register_id);

        if let Some(rt_vmlogic) =
            TestUtilMockEnvManager::get_test_env_rt(DEFAULT_TEST_RT_ENV_ID)
        {
            if let Some(data) = rt_vmlogic.registers.get(&register_id) {
                let len = data.len();
                len as u64
            } else {
                u64::MAX
            }
        } else {
            u64::MAX
        }
    }

    pub fn write_register(
        register_id: l1x_sdk_primitives::RegisterId,
        data_addr: l1x_sdk_primitives::MemoryAddress,
        data_len: u64,
    ) {
        log::debug!("ARGS :: Test Runtime for write_register()");
        log::debug!(
            "RegisterId: {:#?}, MemoryAddress: {:#?}",
            register_id,
            data_addr
        );

        let mut rt_vmlogic =
            TestUtilMockEnvManager::get_test_env_rt(DEFAULT_TEST_RT_ENV_ID)
                .unwrap_or_else(|| panic!("Invalid Test Runtime Context"));

        let data_slice = unsafe {
            slice::from_raw_parts(data_addr as *const u8, data_len as usize)
        };

        rt_vmlogic
            .registers
            .entry(register_id)
            .and_modify(|e| *e = data_slice.to_vec().into())
            .or_insert_with(|| data_slice.to_vec().into());

        TestUtilMockEnvManager::update_test_env_rt(
            DEFAULT_TEST_RT_ENV_ID,
            rt_vmlogic,
        );
    }

    /*
     * Storage API
     */
    // 0 or 1 depending on whether anything was replaced
    pub fn storage_write(
        key_addr: l1x_sdk_primitives::MemoryAddress,
        key_len: u64,
        value_addr: l1x_sdk_primitives::MemoryAddress,
        value_len: u64,
        evicted_register_id: l1x_sdk_primitives::RegisterId,
    ) -> l1x_sdk_primitives::ReturnCode {
        // Safely copy memory into slices, minimizing unsafe code
        let key_slice = unsafe {
            slice::from_raw_parts(key_addr as *const u8, key_len as usize)
        };
        let value_slice = unsafe {
            slice::from_raw_parts(value_addr as *const u8, value_len as usize)
        };

        log::debug!("ARGS :: Test Runtime for storage_write()");
        log::debug!("Key: {:#?}", hex::encode(&key_slice));

        let mut rt_vmlogic =
            TestUtilMockEnvManager::get_test_env_rt(DEFAULT_TEST_RT_ENV_ID)
                .unwrap_or_else(|| panic!("Invalid Test Runtime Context"));

        // Check if the key is new to the storage
        let is_new_insertion = !rt_vmlogic.emu_storage.contains_key(key_slice);

        // Insert key and value into the storage
        rt_vmlogic.emu_storage.insert(key_slice.to_vec(), value_slice.to_vec());

        TestUtilMockEnvManager::update_test_env_rt(
            DEFAULT_TEST_RT_ENV_ID,
            rt_vmlogic,
        );

        l1x_sdk_primitives::ReturnCode::from(is_new_insertion)
    }

    // 0 or 1 depending on whether anything was read
    pub fn storage_read(
        key_addr: l1x_sdk_primitives::MemoryAddress,
        key_len: u64,
        register_id: l1x_sdk_primitives::RegisterId,
    ) -> l1x_sdk_primitives::ReturnCode {
        // Initialize buffers for key and value
        let key_slice = unsafe {
            slice::from_raw_parts(key_addr as *const u8, key_len as usize)
        };

        log::debug!("ARGS :: Test Runtime for storage_read()");
        log::debug!("Key: {:#?}", hex::encode(&key_slice));

        let mut rt_vmlogic =
            TestUtilMockEnvManager::get_test_env_rt(DEFAULT_TEST_RT_ENV_ID)
                .unwrap_or_else(|| panic!("Invalid Test Runtime Context"));

        // Check if the key is new to the storage
        let r_val = rt_vmlogic.emu_storage.contains_key(key_slice);

        if r_val {
            // Insert key and value into the storage
            let read_data = rt_vmlogic
                .emu_storage
                .get(key_slice)
                .unwrap_or_else(|| panic!("Invalid Register Key"));

            rt_vmlogic
                .registers
                .entry(register_id)
                .and_modify(|e| *e = read_data.clone().into())
                .or_insert_with(|| read_data.clone().into());

            TestUtilMockEnvManager::update_test_env_rt(
                DEFAULT_TEST_RT_ENV_ID,
                rt_vmlogic,
            );
        }

        l1x_sdk_primitives::ReturnCode::from(r_val)
    }

    // 0 or 1 depending on whether anything was removed
    pub fn storage_remove(
        key_addr: l1x_sdk_primitives::MemoryAddress,
        key_len: u64,
        register_id: l1x_sdk_primitives::RegisterId,
    ) -> l1x_sdk_primitives::ReturnCode {
        // Initialize buffers for key and value
        let key_slice = unsafe {
            slice::from_raw_parts(key_addr as *const u8, key_len as usize)
        };

        log::debug!("ARGS :: Test Runtime for storage_remove()");
        log::debug!("Key: {:#?}", hex::encode(&key_slice));

        let mut rt_vmlogic =
            TestUtilMockEnvManager::get_test_env_rt(DEFAULT_TEST_RT_ENV_ID)
                .unwrap_or_else(|| panic!("Invalid Test Runtime Context"));

        // Insert key and value into the storage
        let r_val = rt_vmlogic.emu_storage.remove(key_slice).is_some();

        TestUtilMockEnvManager::update_test_env_rt(
            DEFAULT_TEST_RT_ENV_ID,
            rt_vmlogic,
        );

        l1x_sdk_primitives::ReturnCode::from(r_val)
    }

    /*
     * Context API
     */
    pub fn current_runtime_version() -> u64 {
        0u64
    }

    pub fn input(result_register_id: l1x_sdk_primitives::RegisterId) {
        log::debug!("ARGS :: Test Runtime for input()");
        log::debug!("RegisterId: {:x}", result_register_id);

        let mut rt_vmlogic =
            TestUtilMockEnvManager::get_test_env_rt(DEFAULT_TEST_RT_ENV_ID)
                .unwrap_or_else(|| panic!("Invalid Test Runtime Context"));

        rt_vmlogic
            .registers
            .entry(result_register_id)
            .and_modify(|e| *e = rt_vmlogic.input.clone().into())
            .or_insert_with(|| rt_vmlogic.input.clone().into());

        TestUtilMockEnvManager::update_test_env_rt(
            DEFAULT_TEST_RT_ENV_ID,
            rt_vmlogic,
        );
    }

    pub fn output(
        output_addr: l1x_sdk_primitives::MemoryAddress,
        output_len: u64,
    ) {
        log::debug!("ARGS :: Test Runtime for output()");
        log::debug!(
            "MemoryAddress: {:#?}, output_len: {:#?}",
            output_addr,
            output_len
        );

        let output_slice = unsafe {
            slice::from_raw_parts(output_addr as *const u8, output_len as usize)
        };

        let mut rt_vmlogic =
            TestUtilMockEnvManager::get_test_env_rt(DEFAULT_TEST_RT_ENV_ID)
                .unwrap_or_else(|| panic!("Invalid Test Runtime Context"));

        rt_vmlogic.return_data = output_slice.to_vec();

        TestUtilMockEnvManager::update_test_env_rt(
            DEFAULT_TEST_RT_ENV_ID,
            rt_vmlogic,
        );
    }

    pub fn contract_owner_address(register_id: u64) {
        log::debug!("ARGS :: Test Runtime for contract_owner_address()");
        log::debug!("register_id: {:x}", register_id);

        let mut rt_vmlogic =
            TestUtilMockEnvManager::get_test_env_rt(DEFAULT_TEST_RT_ENV_ID)
                .unwrap_or_else(|| panic!("Invalid RunTime ID"));

        let data = rt_vmlogic.contract_owner_address;

        rt_vmlogic
            .registers
            .entry(register_id)
            .and_modify(|e| *e = data.to_vec().into())
            .or_insert_with(|| data.to_vec().into());

        TestUtilMockEnvManager::update_test_env_rt(
            DEFAULT_TEST_RT_ENV_ID,
            rt_vmlogic,
        );
    }

    pub fn caller_address(register_id: u64) {
        log::debug!("ARGS :: Test Runtime for caller_address()");
        log::debug!("register_id: {:x}", register_id);

        let mut rt_vmlogic =
            TestUtilMockEnvManager::get_test_env_rt(DEFAULT_TEST_RT_ENV_ID)
                .unwrap_or_else(|| panic!("Invalid RunTime ID"));

        let data = rt_vmlogic.caller_address;

        rt_vmlogic
            .registers
            .entry(register_id)
            .and_modify(|e| *e = data.to_vec().into())
            .or_insert_with(|| data.to_vec().into());

        TestUtilMockEnvManager::update_test_env_rt(
            DEFAULT_TEST_RT_ENV_ID,
            rt_vmlogic,
        );
    }

    pub fn contract_instance_address(register_id: u64) {
        log::debug!("ARGS :: Test Runtime for contract_instance_address()");
        log::debug!("register_id: {:#?}", register_id);

        let mut rt_vmlogic =
            TestUtilMockEnvManager::get_test_env_rt(DEFAULT_TEST_RT_ENV_ID)
                .unwrap_or_else(|| panic!("Invalid RunTime ID"));

        let data = rt_vmlogic.contract_instance_address;

        rt_vmlogic
            .registers
            .entry(register_id)
            .and_modify(|e| *e = data.to_vec().into())
            .or_insert_with(|| data.to_vec().into());

        TestUtilMockEnvManager::update_test_env_rt(
            DEFAULT_TEST_RT_ENV_ID,
            rt_vmlogic,
        );
    }

    pub fn block_hash(
        output_addr: l1x_sdk_primitives::MemoryAddress,
        output_len: u64,
    ) {
        log::debug!("ARGS :: Test Runtime for block_hash()");
        log::debug!(
            "MemoryAddress: {:#?}, output_len: {:#?}",
            output_addr,
            output_len
        );

        if output_addr == 0 {
            panic!("output_addr is null");
        }

        let rt_vmlogic =
            TestUtilMockEnvManager::get_test_env_rt(DEFAULT_TEST_RT_ENV_ID)
                .unwrap_or_else(|| panic!("Invalid Test Runtime Context"));

        let block_hash = &rt_vmlogic.bc_env.get_block_hash();

        if output_len != block_hash.len() as u64 {
            panic!("block_hash :: Invalid Buffer Length");
        } else {
            let dest_ptr = output_addr as *mut u8;
            let dest_slice = unsafe {
                // Ensuring safety by limiting slice to output_len
                slice::from_raw_parts_mut(dest_ptr, output_len as usize)
            };
            dest_slice.copy_from_slice(block_hash);
        }
    }

    pub fn block_number(
        output_addr: l1x_sdk_primitives::MemoryAddress,
        output_len: u64,
    ) {
        log::debug!("ARGS :: Test Runtime for block_number()");
        log::debug!(
            "MemoryAddress: {:#?}, output_len: {:#?}",
            output_addr,
            output_len
        );

        if output_addr == 0 {
            panic!("output_addr is null");
        }

        let rt_vmlogic =
            TestUtilMockEnvManager::get_test_env_rt(DEFAULT_TEST_RT_ENV_ID)
                .unwrap_or_else(|| panic!("Invalid Test Runtime Context"));

        let block_number_slice =
            rt_vmlogic.bc_env.get_block_number().to_le_bytes();

        if output_len != block_number_slice.len() as u64 {
            panic!("block_number :: Invalid Buffer Length");
        } else {
            let dest_ptr = output_addr as *mut u8;
            let dest_slice = unsafe {
                // Ensuring safety by limiting slice to output_len
                slice::from_raw_parts_mut(dest_ptr, output_len as usize)
            };
            dest_slice.copy_from_slice(&block_number_slice);
        }
    }

    pub fn block_timestamp(
        output_addr: l1x_sdk_primitives::MemoryAddress,
        output_len: u64,
    ) {
        log::debug!("ARGS :: Test Runtime for block_timestamp()");
        log::debug!(
            "MemoryAddress: {:#?}, output_len: {:#?}",
            output_addr,
            output_len
        );

        if output_addr == 0 {
            panic!("output_addr is null");
        }

        let rt_vmlogic =
            TestUtilMockEnvManager::get_test_env_rt(DEFAULT_TEST_RT_ENV_ID)
                .unwrap_or_else(|| panic!("Invalid Test Runtime Context"));

        let block_timestamp_slice =
            rt_vmlogic.bc_env.get_block_timestamp().to_le_bytes();

        if output_len != block_timestamp_slice.len() as u64 {
            panic!("block_timestamp :: Invalid Buffer Length");
        } else {
            let dest_ptr = output_addr as *mut u8;
            let dest_slice = unsafe {
                // Ensuring safety by limiting slice to output_len
                slice::from_raw_parts_mut(dest_ptr, output_len as usize)
            };
            dest_slice.copy_from_slice(&block_timestamp_slice);
        }
    }

    /*
     * Economics API
     */
    pub fn address_balance(
        address_ptr: l1x_sdk_primitives::MemoryAddress,
        address_len: u64,
        result_register_id: l1x_sdk_primitives::RegisterId,
    ) {
        log::debug!("ARGS :: Test Runtime for address_balance()");
        log::debug!(
            "MemoryAddress: {:#?}, address_len: {:#?}, RegisterId: {:#?}",
            address_ptr,
            address_len,
            result_register_id
        );

        let address_slice = unsafe {
            slice::from_raw_parts(
                address_ptr as *const u8,
                address_len as usize,
            )
        };

        let mut rt_vmlogic =
            TestUtilMockEnvManager::get_test_env_rt(DEFAULT_TEST_RT_ENV_ID)
                .unwrap_or_else(|| panic!("Invalid Test Runtime Context"));

        let address_array: l1x_sdk_primitives::Address = {
            let mut arr = [0u8; 20];
            arr.copy_from_slice(&address_slice[0..20]); // Ensure address_slice has at least 20 bytes
            arr.into()
        };

        let balance_data = if let Some(account_state) =
            rt_vmlogic.account_state.get(&address_array)
        {
            account_state.balance.clone().to_le_bytes()
        } else {
            log::error!("Invalid Account {:#?}", address_array);
            panic!("Invalid Account and Account State");
        };

        rt_vmlogic
            .registers
            .entry(result_register_id)
            .and_modify(|e| *e = balance_data.clone().into())
            .or_insert_with(|| balance_data.clone().into());

        TestUtilMockEnvManager::update_test_env_rt(
            DEFAULT_TEST_RT_ENV_ID,
            rt_vmlogic,
        );
    }

    pub fn transfer_to(
        to_address_ptr: l1x_sdk_primitives::MemoryAddress,
        to_address_len: u64,
        amount_ptr: l1x_sdk_primitives::MemoryAddress,
        amount_len: u64,
    ) -> l1x_sdk_primitives::ReturnCode {
        log::debug!("ARGS :: Test Runtime for transfer_to()");
        log::debug!(
            "MemoryAddress: {:#?}, address_len: {:#?}, MemoryAddress: {:#?}, amount_len: {:#?}",
            to_address_ptr,
            to_address_len,
            amount_ptr,
            amount_len
        );

        let to_address_slice = unsafe {
            slice::from_raw_parts(
                to_address_ptr as *const u8,
                to_address_len as usize,
            )
        };

        let to_balance_amount_slice = unsafe {
            slice::from_raw_parts(amount_ptr as *const u8, amount_len as usize)
        };

        // Ensure slices have correct lengths
        if to_address_slice.len() != 20 || to_balance_amount_slice.len() != 16 {
            log::error!("Invalid slice lengths.");
            return 0u64;
        }

        let to_balance_amount = l1x_sdk_primitives::Balance::from_le_bytes(
            to_balance_amount_slice
                .try_into()
                .expect("Invalid length for balance amount slice"),
        );

        let mut rt_vmlogic =
            TestUtilMockEnvManager::get_test_env_rt(DEFAULT_TEST_RT_ENV_ID)
                .unwrap_or_else(|| panic!("Invalid Test Runtime Context"));

        let to_address_array: l1x_sdk_primitives::Address = to_address_slice
            .try_into()
            .expect("Invalid length for address slice");

        if !rt_vmlogic
            .account_state
            .contains_key(&rt_vmlogic.contract_instance_address)
            || !rt_vmlogic.account_state.contains_key(&to_address_array)
        {
            log::error!(
                "Invalid Account {:#?}",
                &rt_vmlogic.contract_instance_address
            );
            log::error!("Invalid Account {:#?}", &to_address_array);
            return 0u64;
        }

        let mut from_account = rt_vmlogic
            .account_state
            .get(&rt_vmlogic.contract_instance_address)
            .expect("No account found for contract instance address")
            .clone();

        let mut to_account = rt_vmlogic
            .account_state
            .get(&to_address_array)
            .expect("No account found for to_address")
            .clone();

        if let Err(err_code) = AccountManager::transfer(
            &mut from_account,
            &mut to_account,
            &to_balance_amount,
        ) {
            log::error!("Account Transfer Failed Error Value {:#?}", err_code);
            return 0u64;
        }

        rt_vmlogic
            .account_state
            .insert(rt_vmlogic.contract_instance_address, from_account.clone());

        rt_vmlogic.account_state.insert(to_address_array, to_account.clone());

        TestUtilMockEnvManager::update_test_env_rt(
            DEFAULT_TEST_RT_ENV_ID,
            rt_vmlogic,
        );

        1u64
    }

    pub fn transfer_from_caller(
        amount_ptr: l1x_sdk_primitives::MemoryAddress,
        amount_len: u64,
    ) -> l1x_sdk_primitives::ReturnCode {
        log::debug!("ARGS :: Test Runtime for transfer_from_caller()");
        log::debug!(
            "MemoryAddress: {:#?}, address_len: {:#?}",
            amount_ptr,
            amount_len,
        );

        let to_balance_amount_slice = unsafe {
            slice::from_raw_parts(amount_ptr as *const u8, amount_len as usize)
        };

        // Ensure slices have correct lengths
        if to_balance_amount_slice.len() != 16 {
            log::error!("Invalid slice lengths.");
            return 0u64;
        }

        let to_balance_amount = l1x_sdk_primitives::Balance::from_le_bytes(
            to_balance_amount_slice
                .try_into()
                .expect("Invalid length for balance amount slice"),
        );

        let mut rt_vmlogic =
            TestUtilMockEnvManager::get_test_env_rt(DEFAULT_TEST_RT_ENV_ID)
                .unwrap_or_else(|| panic!("Invalid Test Runtime Context"));

        if !rt_vmlogic
            .account_state
            .contains_key(&rt_vmlogic.contract_instance_address)
            || !rt_vmlogic
                .account_state
                .contains_key(&rt_vmlogic.caller_address)
        {
            log::error!(
                "Invalid Account {:#?}",
                &rt_vmlogic.contract_instance_address
            );
            log::error!("Invalid Account {:#?}", &rt_vmlogic.caller_address);
            return 0u64;
        }

        let mut from_account = rt_vmlogic
            .account_state
            .get(&rt_vmlogic.caller_address)
            .expect("No account found for to_address")
            .clone();

        let mut to_account = rt_vmlogic
            .account_state
            .get(&rt_vmlogic.contract_instance_address)
            .expect("No account found for contract instance address")
            .clone();

        if let Err(err_code) = AccountManager::transfer(
            &mut from_account,
            &mut to_account,
            &to_balance_amount,
        ) {
            log::error!("Account Transfer Failed Error Value {:#?}", err_code);
            return 0u64;
        }

        rt_vmlogic
            .account_state
            .insert(rt_vmlogic.caller_address, from_account.clone());

        rt_vmlogic
            .account_state
            .insert(rt_vmlogic.contract_instance_address, to_account.clone());

        TestUtilMockEnvManager::update_test_env_rt(
            DEFAULT_TEST_RT_ENV_ID,
            rt_vmlogic,
        );

        1u64
    }

    /*
     * Misc API
     */
    pub fn panic() -> ! {
        panic!("This function always panics");
    }

    pub fn msg(addr: l1x_sdk_primitives::MemoryAddress, len: u64) {
        log::debug!("ARGS :: Test Runtime for msg()");

        // Create a slice from raw parts
        let slice = unsafe {
            std::slice::from_raw_parts(addr as *const u8, len as usize)
        };

        // Attempt to convert the slice to a UTF-8 string and log
        match std::str::from_utf8(slice) {
            Ok(message) => log::info!("TEST_RT_SYS_MSG => {:#?}", message),
            Err(e) => log::error!("Failed to convert to str: {:#?}", e),
        }
    }

    pub fn call_contract(
        call_addr: l1x_sdk_primitives::MemoryAddress,
        len: u64,
        result_register_id: l1x_sdk_primitives::RegisterId,
    ) -> l1x_sdk_primitives::ReturnCode {
        0u64
    }

    pub fn emit_event_experimental(
        data_addr: l1x_sdk_primitives::MemoryAddress,
        len: u64,
    ) -> l1x_sdk_primitives::ReturnCode {
        0u64
    }
}
