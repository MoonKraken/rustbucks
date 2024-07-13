use std::{
    sync::{Arc, RwLock},
    thread::{self, sleep, JoinHandle},
};

use anyhow::anyhow;
use fixed::types::I32F32;
use futures::executor::block_on;
use rand::seq::SliceRandom;
use rand::thread_rng;
use rustbucks::{
    mine::mine_pending_transactions,
    model::{node::Node, transaction::Transaction},
};
use tokio::time::Duration;
#[tokio::test]
pub async fn three_node_async_convergence() {
    let participant_names = [
        "Timmy",
        "Bobby",
        "Alice",
        "Charlie",
        "Spongebob",
        "Patrick",
        "Crabs",
    ];

    let mut rng = thread_rng();

    let transactions: Vec<Transaction> = (0..1000)
        .map(|i| {
            let sender = participant_names
                .choose(&mut rng)
                .ok_or(anyhow!("participant choice failure"))?;
            let mut receiver = participant_names
                .choose(&mut rng)
                .ok_or(anyhow!("participant choice failure"))?;
            while receiver == sender {
                receiver = participant_names
                    .choose(&mut rng)
                    .ok_or(anyhow!("participant choice failure"))?;
            }

            Ok(Transaction {
                timestamp: i,
                sender: sender.to_string(),
                receiver: receiver.to_string(),
                amount: I32F32::from_num(100),
            })
        })
        .collect::<Result<Vec<Transaction>, anyhow::Error>>()
        .expect("issue creating transactions");

    let transactions = Arc::new(transactions);
    // the basic idea here is that transactions are submitted to random nodes
    // and miners submit their blocks to random nodes
    // and in the end all nodes should have the same set of confirmed transactions
    let a = Arc::new(RwLock::new(Node::new()));
    let b = Arc::new(RwLock::new(Node::new()));
    let c = Arc::new(RwLock::new(Node::new()));

    let nodes = vec![a.clone(), b.clone(), c.clone()];

    let nodes_cloned = nodes.clone();
    let nodes_cloned_2 = nodes.clone();
    let nodes_cloned_3 = nodes.clone();

    let transactions_2 = transactions.clone();
    let transaction_blaster = thread::spawn(move || {
        let mut rng = thread_rng();
        for transaction in transactions_2.iter() {
            let node = nodes_cloned
                .choose(&mut rng)
                .ok_or(anyhow!(""))
                .expect("rng issue");

            sleep(Duration::from_millis(2));
            println!("submitting transaction");
            let mut lock = node.write().expect("issue getting write lock");
            block_on(lock.submit_transaction(transaction.clone()));
        }
    });

    let miner = thread::spawn(move || {
        for _ in 0..1000 {
            let mut rng = thread_rng();
            let node = nodes_cloned_2
                .choose(&mut rng)
                .ok_or(anyhow!("node choice failure"))
                .expect("rng issue");
            sleep(Duration::from_millis(10));
            let new_block = {
                let lock = node.read().expect("issue getting read lock");
                // don't try to mine an empty block
                if lock.pending_transactions.is_empty() {
                    continue;
                }
                mine_pending_transactions(
                    &lock.blockchain,
                    lock.pending_transactions.clone().into_iter().collect(),
                )
            };
            println!("miner mining block");
            let mut lock = node.write().expect("issue getting write lock");
            let _ = block_on(lock.submit_mined_block(new_block));
        }
    });

    let mut distribution_tasks: Vec<JoinHandle<_>> = nodes
        .into_iter()
        .flat_map(|node| {
            //get all nodes except the current one
            let other_nodes: Vec<Arc<RwLock<Node>>> = nodes_cloned_3
                .iter()
                .filter_map(|other_node| {
                    if !Arc::ptr_eq(&other_node, &node) {
                        Some(other_node.clone())
                    } else {
                        None
                    }
                })
                .collect();

            let other_nodes_2 = other_nodes.clone();
            let node_2 = node.clone();
            let chain_receive_tasks = thread::spawn(move || {
                for _ in 0..10000 {
                    for other_node in &other_nodes_2 {
                        println!("node receiving chain");
                        let blockchain = {
                            let node_read_lock = other_node.read().expect("should get read lock");
                            node_read_lock.blockchain.clone() //TODO remove this as soon as possible!
                        };
                        let mut node_write_lock = node_2.write().expect("couldn't get read lock");
                        block_on(node_write_lock.receive_chain(&blockchain));
                    }
                }

                sleep(Duration::from_millis(5));
            });

            let transaction_tasks = thread::spawn(move || {
                for _ in 0..1000 {
                    for other_node in &other_nodes {
                        println!("node receiving transactions");
                        let pending_transactions = {
                            let node_read_lock = other_node.read().expect("should get read lock");
                            node_read_lock.pending_transactions.clone()
                        };
                        let mut node_write_lock = node.write().expect("couldn't get read lock");
                        block_on(node_write_lock.receive_transactions(&pending_transactions))
                    }
                }

                sleep(Duration::from_millis(8));
            });

            vec![transaction_tasks, chain_receive_tasks]
        })
        .collect();

    distribution_tasks.push(transaction_blaster);
    distribution_tasks.push(miner);
    // distribution_tasks.push(slow_miner);
    for handle in distribution_tasks.into_iter() {
        handle.join().expect("panic at the disco");
    }

    //ok, validate everything has the same blockchain
    //validate all transactions are
    let a = a.read().expect("read lock failure");
    let b = b.read().expect("read lock failure");
    let c = c.read().expect("read lock failure");

    assert_eq!(
        transactions.len() + 1,
        a.blockchain.confirmed_transactions.len(),
    );
    assert_eq!(
        a.blockchain,
        b.blockchain
    );
    assert_eq!(
        a.blockchain,
        c.blockchain
    );
}

// didn't wind up using this, leaving the code anyway
fn extract_node(node: Arc<RwLock<Node>>) -> Node {
    Arc::try_unwrap(node)
        .expect("noah not here")
        .into_inner()
        .expect("poisoned lock")
}
