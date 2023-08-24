use borsh::BorshSerialize;
pub use l1x_sdk_macros::contract;
pub use l1x_sys as sys;
use std::panic as std_panic;
use types::Address;

pub mod contract_interaction;
pub mod store;
pub mod types;
use contract_interaction::ContractCall;
pub mod utils;
pub(crate) use crate::utils::*;

const EVICTED_REGISTER: u64 = std::u64::MAX - 1;
const ATOMIC_OP_REGISTER: u64 = std::u64::MAX - 2;

macro_rules! try_method_into_register {
    ( $method:ident ) => {{
        unsafe { l1x_sys::$method(ATOMIC_OP_REGISTER) };
        read_register(ATOMIC_OP_REGISTER)
    }};
}

macro_rules! method_into_register {
    ( $method:ident ) => {{
        expect_register(try_method_into_register!($method))
    }};
}

fn register_len(register_id: u64) -> Option<u64> {
    let len = unsafe { l1x_sys::register_len(register_id) };
    if len == std::u64::MAX {
        None
    } else {
        Some(len)
    }
}

fn read_register(register_id: u64) -> Option<Vec<u8>> {
    let len: usize =
        register_len(register_id)?.try_into().unwrap_or_else(|_| abort());

    let mut buffer = Vec::with_capacity(len);

    unsafe {
        l1x_sys::read_register(register_id, buffer.as_mut_ptr() as u64);

        buffer.set_len(len);
    }
    Some(buffer)
}

fn expect_register<T>(option: Option<T>) -> T {
    option.unwrap_or_else(|| abort())
}

fn panic_hook_impl(info: &std_panic::PanicInfo) {
    msg(&info.to_string());
    abort();
}

pub fn setup_panic_hook() {
    std_panic::set_hook(Box::new(panic_hook_impl));
}

pub fn abort() -> ! {
    #[cfg(test)]
    panic("Mocked panic function called!");
    #[cfg(not(test))]
    unsafe {
        l1x_sys::panic()
    }
}

pub fn panic(message: &str) -> ! {
    msg(message);

    abort()
}

pub fn input() -> Option<Vec<u8>> {
    #[cfg(test)]
    {
        return tests::input();
    }
    #[cfg(not(test))]
    try_method_into_register!(input)
}

pub fn output(data: &[u8]) {
    #[cfg(test)]
    {
        return tests::output(data);
    }
    #[cfg(not(test))]
    unsafe {
        sys::output(data.as_ptr() as _, data.len() as _)
    }
}

pub fn msg(message: &str) {
    #[cfg(test)]
    {
        return tests::msg(message);
    }
    #[cfg(not(test))]
    {
        #[cfg(all(debug_assertions, not(target_arch = "wasm32")))]
        eprintln!("{}", message);

        unsafe { l1x_sys::msg(message.as_ptr() as _, message.len() as _) }
    }
}

pub fn storage_write(key: &[u8], value: &[u8]) -> bool {
    #[cfg(test)]
    {
        return tests::storage_write(key, value);
    }
    #[cfg(not(test))]
    match unsafe {
        sys::storage_write(
            key.as_ptr() as _,
            key.len() as _,
            value.as_ptr() as _,
            value.len() as _,
            EVICTED_REGISTER,
        )
    } {
        0 => false,
        1 => true,
        _ => abort(),
    }
}

pub fn storage_remove(key: &[u8]) -> bool {
    #[cfg(test)]
    {
        return tests::storage_remove(key);
    }

    #[cfg(not(test))]
    match unsafe {
        sys::storage_remove(key.as_ptr() as _, key.len() as _, EVICTED_REGISTER)
    } {
        0 => false,
        1 => true,
        _ => abort(),
    }
}

pub fn storage_read(key: &[u8]) -> Option<Vec<u8>> {
    #[cfg(test)]
    {
        return tests::storage_read(key);
    }

    #[cfg(not(test))]
    match unsafe {
        sys::storage_read(key.as_ptr() as _, key.len() as _, ATOMIC_OP_REGISTER)
    } {
        0 => None,
        1 => Some(expect_register(read_register(ATOMIC_OP_REGISTER))),
        _ => abort(),
    }
}

pub fn contract_owner_address() -> Address {
    #[cfg(test)]
    {
        return tests::contract_owner_address();
    }
    #[cfg(not(test))]
    method_into_register!(contract_owner_address)
        .try_into()
        .unwrap_or_else(|_| abort())
}

pub fn caller_address() -> Address {
    #[cfg(test)]
    {
        return tests::caller_address();
    }
    #[cfg(not(test))]
    method_into_register!(caller_address).try_into().unwrap_or_else(|_| abort())
}

pub fn contract_instance_address() -> Address {
    #[cfg(test)]
    {
        return tests::contract_instance_address();
    }
    #[cfg(not(test))]
    method_into_register!(contract_instance_address)
        .try_into()
        .unwrap_or_else(|_| abort())
}

pub fn call_contract(call: &ContractCall) -> Option<Vec<u8>> {
    let call =
        call.try_to_vec().expect("Can't serialize the function arguments");
    match unsafe {
        sys::call_contract(
            call.as_ptr() as _,
            call.len() as _,
            ATOMIC_OP_REGISTER,
        )
    } {
        0 => None,
        1 => Some(expect_register(read_register(ATOMIC_OP_REGISTER))),
        _ => abort(),
    }
}

pub fn emit_event_experimental<T>(event: T)
where
    T: BorshSerialize,
{
    let event_data = event.try_to_vec().expect("Can't serialize the event");
    match unsafe {
        sys::emit_event_experimental(
            event_data.as_ptr() as _,
            event_data.len() as _,
        )
    } {
        0 => abort(),
        _ => (),
    }
}

#[cfg(test)]
mod tests {

    use lazy_static::lazy_static;
    use std::collections::HashMap;
    use std::sync::Mutex;

    use crate::types::Address;

    lazy_static! {
        static ref MOCK_DATA: Mutex<MockData> = Mutex::new(MockData::new());
    }

    const CONTRACT_OWNER_ADDRESS: &[u8; 20] = b"mock_owner_address11";
    const CONTRACT_INSTANCE_ADDRESS: &[u8; 20] = b"mock_instance_addres";
    const CALLER_ADDRESS: &[u8; 20] = b"mock_caller_address1";

    pub struct MockData {
        storage: HashMap<Vec<u8>, Vec<u8>>,
        input: Option<Vec<u8>>,
        output: Vec<u8>,
        messages: Vec<String>,
        contract_owner_address: Address,
        caller_address: Address,
        contract_instance_address: Address,
    }

    impl MockData {
        pub fn new() -> Self {
            Self {
                storage: HashMap::new(),
                input: Some(Vec::new()),
                output: Vec::new(),
                messages: Vec::new(),
                contract_owner_address: Address::test_create_address(
                    &CONTRACT_OWNER_ADDRESS.to_vec(),
                ),
                caller_address: Address::test_create_address(
                    &CALLER_ADDRESS.to_vec(),
                ),
                contract_instance_address: Address::test_create_address(
                    &CONTRACT_INSTANCE_ADDRESS.to_vec(),
                ),
            }
        }
    }

    pub fn storage_write(key: &[u8], value: &[u8]) -> bool {
        MOCK_DATA.lock().unwrap().storage.insert(key.to_vec(), value.to_vec());
        true
    }

    pub fn storage_read(key: &[u8]) -> Option<Vec<u8>> {
        MOCK_DATA.lock().unwrap().storage.get(key).cloned()
    }

    pub fn storage_remove(key: &[u8]) -> bool {
        MOCK_DATA.lock().unwrap().storage.remove(key).is_some()
    }

    pub fn contract_owner_address() -> Address {
        MOCK_DATA.lock().unwrap().contract_owner_address.clone()
    }

    pub fn caller_address() -> Address {
        MOCK_DATA.lock().unwrap().caller_address.clone()
    }

    pub fn contract_instance_address() -> Address {
        MOCK_DATA.lock().unwrap().contract_instance_address.clone()
    }

    pub fn remove_from_mock_storage(key: &[u8]) -> bool {
        MOCK_DATA.lock().unwrap().storage.remove(key).is_some()
    }

    pub fn input() -> Option<Vec<u8>> {
        MOCK_DATA.lock().unwrap().input.clone()
    }

    pub fn output(data: &[u8]) {
        MOCK_DATA.lock().unwrap().output = data.to_vec();
    }

    pub fn msg(message: &str) {
        MOCK_DATA.lock().unwrap().messages.push(message.to_owned());
    }

    pub fn set_mock_input(data: Vec<u8>) {
        MOCK_DATA.lock().unwrap().input = Some(data);
    }

    pub fn get_mock_output() -> Vec<u8> {
        MOCK_DATA.lock().unwrap().output.clone()
    }

    pub fn get_mock_msgs() -> Vec<String> {
        MOCK_DATA.lock().unwrap().messages.clone()
    }

    pub fn clear_mock_io() {
        let mut data = MOCK_DATA.lock().unwrap();
        data.input = None;
        data.output = Vec::new();
        data.messages = Vec::new();
    }

    pub fn set_mock_contract_owner_address(owner_address: Vec<u8>) {
        MOCK_DATA.lock().unwrap().contract_owner_address =
            Address::test_create_address(&owner_address);
    }

    pub fn set_mock_caller_address(caller_address: Vec<u8>) {
        MOCK_DATA.lock().unwrap().caller_address =
            Address::test_create_address(&caller_address);
    }

    pub fn set_mock_contract_instance_address(
        contract_instance_address: Vec<u8>,
    ) {
        MOCK_DATA.lock().unwrap().contract_instance_address =
            Address::test_create_address(&contract_instance_address);
    }

    ////////////////////////////////////////////// TESTS ////////////////////////////////////////////////////////////
    #[test]
    fn test_storage() {
        // Prepare key-value
        let key = b"key";
        let value = b"value";

        // Write to storage
        assert!(storage_write(key, value));

        // Read from storage
        let stored_value = storage_read(key).unwrap();
        assert_eq!(stored_value, value);

        // Remove from storage
        assert!(storage_remove(key));

        // Try to read removed key
        assert!(storage_read(key).is_none());
    }

    #[test]
    fn test_msg() {
        let message = "Test message";
        msg(message);

        let mock_messages = get_mock_msgs();
        assert_eq!(mock_messages.len(), 1);
        assert_eq!(mock_messages[0], message);
    }

    #[test]
    fn test_input_output() {
        let data = vec![1, 2, 3, 4];

        set_mock_input(data.clone());

        // Check input
        let input_data = input().unwrap();
        assert_eq!(input_data, data);

        // Output
        output(&data);

        // Check output
        let output_data = get_mock_output();
        assert_eq!(output_data, data);

        // Clear
        clear_mock_io();

        // Check input and output are cleared
        assert!(input().is_none());
        assert!(get_mock_output().is_empty());
    }

    #[test]
    fn test_storage_write_and_read() {
        let key = vec![1, 2, 3];
        let value = vec![4, 5, 6];

        // Write to storage
        storage_write(&key, &value);

        // Read from storage and check value
        let stored_value = storage_read(&key).unwrap();
        assert_eq!(stored_value, value);
    }

    #[test]
    fn test_remove_from_mock_storage() {
        let key = vec![1, 2, 3];
        let value = vec![4, 5, 6];

        // Write to storage and then remove
        storage_write(&key, &value);
        remove_from_mock_storage(&key);

        // Check value is removed
        let stored_value = storage_read(&key);
        assert!(stored_value.is_none());
    }

    #[test]
    fn test_contract_owner_address_and_caller_address() {
        let mock_owner_address = b"current_address12345".to_vec();
        let mock_caller_address = b"caller_address123456".to_vec();
        let mock_instance_address = b"instance_address3456".to_vec();

        // Set mock data
        set_mock_contract_owner_address(mock_owner_address.clone());
        set_mock_caller_address(mock_caller_address.clone());
        set_mock_contract_instance_address(mock_instance_address.clone());

        // Test contract_owner_address
        assert_eq!(
            contract_owner_address(),
            Address::test_create_address(&mock_owner_address)
        );

        // Test caller_address
        assert_eq!(
            caller_address(),
            Address::test_create_address(&mock_caller_address)
        );

        assert_eq!(
            contract_instance_address(),
            Address::test_create_address(&mock_instance_address)
        );
    }

    #[test]
    fn test_input_and_output() {
        let data = vec![1, 2, 3];

        // Set mock input and verify it
        set_mock_input(data.clone());
        assert_eq!(input().unwrap(), data);

        // Write to output
        output(&data);

        // Verify output
        assert_eq!(get_mock_output(), data);
    }

    #[test]
    fn test_clear_mock_io() {
        // Set some mock input/output data and a message
        set_mock_input(vec![1, 2, 3]);
        output(&vec![4, 5, 6]);
        msg("Hello, world!");

        // Clear the mock I/O data
        clear_mock_io();

        // Verify everything was cleared
        assert!(input().is_none());
        assert_eq!(get_mock_output(), vec![] as Vec<u8>);
        assert_eq!(get_mock_msgs(), Vec::<String>::new());
    }
}
