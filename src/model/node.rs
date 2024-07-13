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
    pub async fn receive_chain(&mut self, recieved_chain: &Blockchain) {
        // replace the current chain with the received chain
        // if the received one is valid and longer
        if recieved_chain.chain.len() <= self.blockchain.chain.len() {
            return; // reject the received chain
        }

        // ok so we find your chain intriguing,
        // let's verify that it is valid
        if !recieved_chain.is_valid() {
            return;
        }

        //we are replacing the chain, we need to return
        //any transactions that exist in the old chain but not the
        //new one to a pending state
        let previous_confirmed_transactions = self.blockchain.confirmed_transactions.clone();
        let orphaned =
            previous_confirmed_transactions.difference(&recieved_chain.confirmed_transactions);

        self.blockchain = recieved_chain.clone();

        // remove any pending transactions that are in the new chain
        for block in self.blockchain.chain.iter() {
            for transaction in block.transactions.iter() {
                if self.pending_transactions.contains(transaction) {
                    self.pending_transactions.remove(transaction);
                }
            }
        }

        // put transactions orphaned by the new chain back into a pending state
        for transaction in orphaned {
            self.pending_transactions.insert(transaction.clone());
        }
    }

    pub async fn submit_mined_block(&mut self, new_block: Block) -> Result<(), BlockchainError> {
        match self.blockchain.add_new_block(new_block.clone()) {
            Ok(()) => {
                for confirmed_transaction in new_block.transactions {
                    self.pending_transactions.remove(&confirmed_transaction);
                }

                Ok(())
            },
            Err(e) => Err(e),
        }
    }

    pub async fn receive_transactions(&mut self, received_transactions: &HashSet<Transaction>) {
        //only add them if they don't already exist in our blockchain
        for transaction in received_transactions {
            if !self.blockchain.confirmed_transactions.contains(transaction) {
                self.pending_transactions.insert(transaction.clone());
            }
        }
    }

    pub async fn broadcast_transactions(&self, nodes: &mut Vec<Node>) {
        for node in nodes {
            node.receive_transactions(&self.pending_transactions).await
        }
    }

    pub async fn submit_transaction(&mut self, transaction: Transaction) {
        //ignore if the transaction was already confirmed
        if !self
            .blockchain
            .confirmed_transactions
            .contains(&transaction)
        {
            self.pending_transactions.insert(transaction);
        }
    }
}
