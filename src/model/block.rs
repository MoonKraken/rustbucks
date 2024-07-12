use super::transaction::Transaction;
use sha2::Sha256;
use sha2::Digest;

#[derive(Debug, Clone)]
pub struct Block {
    pub index: u64,
    pub transactions: Vec<Transaction>,
    pub nonce: u64,
    pub previous_hash: String,
    pub timestamp: i64,
}

impl Block {
    pub fn hash(&self) -> String {
        let mut hasher = Sha256::new();
        hasher.update(format!("{:?}", self));
        format!("{:x}", hasher.finalize())
    }
}
