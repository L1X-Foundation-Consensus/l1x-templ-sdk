//! Basic types

mod int;
mod primitives;
pub use int::{I128, I64, U128, U256, U64};
pub use primitives::{
    Address, AddressArray, Balance, BlockHash, BlockNumber, TimeStamp,
};

pub type Nonce = u128;
pub type RegisterId = u64;
pub type MemoryAddress = u64;
pub type ReturnCode = u64;
pub type BlockTimeStamp = TimeStamp;
