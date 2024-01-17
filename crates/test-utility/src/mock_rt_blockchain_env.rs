use l1x_sdk_primitives::{BlockHash, BlockNumber, BlockTimeStamp};

#[derive(Debug, Clone)]
pub struct BlockchainEnvironment {
    block_number: BlockNumber,
    block_hash: BlockHash,
    block_timestamp: BlockTimeStamp,
}

impl Default for BlockchainEnvironment {
    fn default() -> Self {
        BlockchainEnvironment::new()
    }
}

impl BlockchainEnvironment {
    pub fn new() -> Self {
        let block_number: BlockNumber = 42;

        let block_hash: BlockHash = [
            1u8, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18,
            19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32,
        ];

        let block_timestamp: BlockTimeStamp = 0xff;

        Self { block_number, block_hash, block_timestamp }
    }

    pub fn get_block_number(&self) -> BlockNumber {
        self.block_number
    }

    pub fn get_block_hash(&self) -> BlockHash {
        self.block_hash
    }

    pub fn get_block_timestamp(&self) -> BlockTimeStamp {
        self.block_timestamp
    }
}
