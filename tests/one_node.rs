use fixed::types::I32F32;
use rustbucks::{mine::mine_pending_transactions, model::{node::Node, transaction::Transaction}};

#[tokio::test]
pub async fn one_node_should_accept_one_block() {
    let mut node = Node::new();

    let new_transactions = vec![
        Transaction {
            timestamp: 0,
            sender: "Timmy".to_string(),
            receiver: "Bobby".to_string(),
            amount: I32F32::from_num(100),
        },
        Transaction {
            timestamp: 1,
            sender: "Alice".to_string(),
            receiver: "Charlie".to_string(),
            amount: I32F32::from_num(100),
        },
        Transaction {
            timestamp: 2,
            sender: "Jill".to_string(),
            receiver: "Jane".to_string(),
            amount: I32F32::from_num(20),
        },
    ];

    // no need to submit to the node pending transactions for this #[cfg(test)]
    let new_block = mine_pending_transactions(&node.blockchain, new_transactions);

    let res = node.submit_mined_block(new_block).await;

    assert_eq!(Ok(()), res);
}
