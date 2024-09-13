use chrono::Utc;
use sha2::Sha256;
use sha2::Digest;
use tracing::instrument;

use crate::model::{blockchain::Blockchain, transaction::Transaction, block::Block};

// for the given blockchain and pending transactions, try to mine a new block
// with a subset of the pending transactions. Theoretically we shouldn't need
// the entire blockchain, just the last block.
#[instrument]
pub fn mine_pending_transactions(blockchain: &Blockchain, pending_transactions: Vec<Transaction>) -> Block {
    todo!();
}
