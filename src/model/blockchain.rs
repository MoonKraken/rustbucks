use std::collections::HashSet;

use fixed::types::I32F32;
use sha2::Digest;
use sha2::Sha256;

use super::{block::Block, transaction::Transaction};

#[derive(Debug, Clone, PartialEq)]
pub struct Blockchain {
    pub chain: Vec<Block>,

    //this is for adjusting the difficulty
    pub target_hash_prefix: String,

    pub confirmed_transactions: HashSet<Transaction>,
}

#[derive(Debug, PartialEq)]
pub enum BlockchainError {
    UnknownTransaction,
    IncorrectProof,
    InvalidIndex,
    PreviousHashDoesNotMatch,
    EmptyTransactions,
}

impl Blockchain {
    pub fn new() -> Self {todo!();}

    // This receives a newly mined block and does the appropriate validation
    pub fn add_new_block(&mut self, new_block: Block) -> Result<(), BlockchainError> {todo!();}

    // Validate the entire blockchain
    // things to validate
    // 1. Previous hash matches actual hash of previous block
    // 2. Hashes all have the target prefix
    pub fn is_valid(&self) -> bool {todo!();}
}

#[cfg(test)]
mod test {
    use fixed::types::I32F32;

    use crate::model::{block::Block, blockchain::BlockchainError, transaction::Transaction};

    use super::Blockchain;

    // this is really just to discover the nonce of the first block
    // should we ever need to update the contents of the block
    #[test]
    pub fn discover_nonce_for_first_block() {
        let chain = Blockchain::new();
        let mut first_block = chain.chain.first().expect("should have genesis block").clone();

        while !first_block.hash().starts_with(&chain.target_hash_prefix) {
            first_block.nonce = first_block.nonce + 1;
        }

        println!("nonce discovered:");
        dbg!(first_block.nonce);
    }

    #[test]
    pub fn is_valid_returns_false_for_chain_with_incorrect_target_prefix() {
        let mut chain = Blockchain::new();
        let previous_hash = chain
            .chain
            .first()
            .expect("should have genesis block")
            .hash();

        let block_with_hash_without_target_prefix = Block {
            index: 1,
            nonce: 0,
            previous_hash,
            transactions: vec![Transaction {
                sender: "Billy".to_string(),
                receiver: "Timmy".to_string(),
                timestamp: 0,
                amount: I32F32::from_num(1),
            }],
            timestamp: 0,
        };

        dbg!(block_with_hash_without_target_prefix.hash()); // make sure this doesn't miraculously start with 00

        chain.chain.push(block_with_hash_without_target_prefix);

        assert_eq!(chain.is_valid(), false);
    }

    #[test]
    pub fn is_valid_returns_false_for_chain_with_invalid_hash() {
        let invalid_block = Block {
            index: 1,
            nonce: 0,
            previous_hash: "asdf".to_string(), // this is incorrect
            transactions: vec![Transaction {
                sender: "Billy".to_string(),
                receiver: "Timmy".to_string(),
                timestamp: 0,
                amount: I32F32::from_num(1),
            }],
            timestamp: 0,
        };

        let mut chain = Blockchain::new();
        chain.chain.push(invalid_block);

        assert_eq!(chain.is_valid(), false);
    }

    #[test]
    pub fn should_not_add_invalid_block_invalid_nonce() {
        let mut chain = Blockchain::new();
        let previous_hash = chain.chain.first().expect("genesis block").hash();
        let invalid_block = Block {
            index: 1,
            // there is a chance this nonce will unintentionally yield a correct hash,
            // but it is unlikely. if this test ever fails
            // try changing the nonce to something else
            // and it will probably be fixed
            nonce: 0,
            previous_hash,
            transactions: vec![Transaction {
                sender: "Billy".to_string(),
                receiver: "Timmy".to_string(),
                timestamp: 0,
                amount: I32F32::from_num(1),
            }],
            timestamp: 0,
        };

        let res = chain.add_new_block(invalid_block);

        assert_eq!(res, Err(BlockchainError::IncorrectProof));
    }

    #[test]
    pub fn should_not_add_invalid_block_invalid_previous_hash() {
        let invalid_block = Block {
            index: 1,
            nonce: 245,
            previous_hash: "6109c0d119501c326c8a613b9d99069caf7372566e5725a72b47cc9d737f304d"
                .to_string(), // this is incorrect
            transactions: vec![Transaction {
                sender: "me".to_string(),
                receiver: "you".to_string(),
                timestamp: 0,
                amount: I32F32::from_num(50),
            }],
            timestamp: 1719876768,
        };

        let mut chain = Blockchain::new();
        let res = chain.add_new_block(invalid_block);

        assert_eq!(res, Err(BlockchainError::PreviousHashDoesNotMatch));
    }

    #[test]
    pub fn should_add_valid_block() {
        let mut chain = Blockchain::new();
        let previous_hash = chain.chain.first().expect("genesis block").hash();
        let mut valid_block = Block {
            index: 1,
            nonce: 245,
            previous_hash,
            transactions: vec![Transaction {
                sender: "me".to_string(),
                receiver: "you".to_string(),
                timestamp: 1719876768,
                amount: I32F32::from_num(50),
            }],
            timestamp: 1719876768,
        };

        // "mine" for a good nonce
        while !valid_block.hash().starts_with(&chain.target_hash_prefix) {
            valid_block.nonce = valid_block.nonce + 1;
        }

        let res = chain.add_new_block(valid_block);

        assert_eq!(res, Ok(()));
    }
}
