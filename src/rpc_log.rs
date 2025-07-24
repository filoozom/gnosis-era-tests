use hex::encode;
use std::fmt;

#[derive(Debug)]
pub struct RpcLog {
    pub address: Vec<u8>,
    pub topic_0: Vec<u8>,
    pub topic_1: Vec<u8>,
    pub topic_2: Vec<u8>,
    pub topic_3: Vec<u8>,
    pub data: Vec<u8>,
    pub block_number: u64,
    pub transaction_hash: Vec<u8>,
    pub transaction_index: u64,
    pub block_hash: Vec<u8>,
    pub log_index: u64,
}

impl fmt::Display for RpcLog {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "RpcLog {{\n  address: 0x{},\n  topic_0: 0x{},\n  topic_1: 0x{},\n  topic_2: 0x{},\n  topic_3: 0x{},\n  data: 0x{},\n  block_number: {},\n  transaction_hash: 0x{},\n  transaction_index: {},\n  block_hash: 0x{},\n  log_index: {}\n}}",
            encode(&self.address),
            encode(&self.topic_0),
            encode(&self.topic_1),
            encode(&self.topic_2),
            encode(&self.topic_3),
            encode(&self.data),
            self.block_number,
            encode(&self.transaction_hash),
            self.transaction_index,
            encode(&self.block_hash),
            self.log_index
        )
    }
}
