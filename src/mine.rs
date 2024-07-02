use chrono::Utc;
use sha2::Sha256;
use sha2::Digest;
use tracing::instrument;

use crate::model::{blockchain::Blockchain, transaction::Transaction, block::Block};

#[instrument]
pub fn mine_pending_transactions(blockchain: &Blockchain, pending_transactions: Vec<Transaction>) -> Block {
    // for now try to include all current transactions into the next block,
    // theoretically we could cherry pick a subset of the transactions
    let last_block = blockchain
        .chain
        .last()
        .expect("couldnt get last block. this shouldn't happen.");

    let mut hasher = Sha256::new();

    //compute hash of the previous block
    hasher.update(format!("{:?}", last_block));
    let previous_hash = format!("{:x}", hasher.finalize());
    let timestamp = Utc::now().timestamp();

    let mut new_block = Block {
        index: last_block.index + 1,
        transactions: pending_transactions.clone().into_iter().collect(),
        previous_hash,
        timestamp,
        nonce: 0,
    };

    loop {
        hasher = Sha256::new();
        hasher.update(format!("{:?}", new_block));
        let hash = format!("{:x}", hasher.finalize());
        if hash.starts_with(&blockchain.target_hash_prefix) {
            break;
        }

        new_block.nonce = new_block.nonce + 1;
    }

    new_block
}
