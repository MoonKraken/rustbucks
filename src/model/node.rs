use std::collections::HashSet;

use super::{
    block::Block,
    blockchain::{Blockchain, BlockchainError},
    transaction::Transaction,
};

#[derive(PartialEq, Debug)]
pub struct Node {
    pub blockchain: Blockchain,
    pub pending_transactions: HashSet<Transaction>,
}

impl Node {
    pub fn new() -> Self {
        Node {
            blockchain: Blockchain::new(),
            pending_transactions: HashSet::new(),
        }
    }

    pub async fn broadcast_chain(&self, nodes: &mut Vec<Node>) {
        for node in nodes {
            node.receive_chain(&self.blockchain).await
        }
    }

    // in a real implementation, we probably wouldn't be receiving
    // entire blockchains, probably just strings of hashes
    // or maybe the most recent x blocks
    pub async fn receive_chain(&mut self, recieved_chain: &Blockchain) {todo!();}

    pub async fn submit_mined_block(&mut self, new_block: Block) -> Result<(), BlockchainError> {todo!();}

    // This accepts new pending transactions from other nodes
    pub async fn receive_transactions(&mut self, received_transactions: &HashSet<Transaction>) { todo!(); }

    pub async fn broadcast_transactions(&self, nodes: &mut Vec<Node>) {
        for node in nodes {
            node.receive_transactions(&self.pending_transactions).await
        }
    }

    //submit a new pending transaction
    pub async fn submit_transaction(&mut self, transaction: Transaction) {todo!();}
}
