use rustbucks::{mine::mine_pending_transactions, model::{node::Node, transaction::Transaction}};

#[tokio::test]
pub async fn two_nodes_with_distinct_blockchains_should_converge() {
    let mut a = Node::new();

    let a_transactions = vec![
        Transaction {
            timestamp: 0,
            sender: "Timmy".to_string(),
            receiver: "Bobby".to_string(),
            amount: 100,
        },
        Transaction {
            timestamp: 1,
            sender: "Alice".to_string(),
            receiver: "Charlie".to_string(),
            amount: 100,
        },
    ];

    let a_transactions_2 = vec![
        Transaction {
            timestamp: 2,
            sender: "Jill".to_string(),
            receiver: "Jane".to_string(),
            amount: 20,
        },
    ];

    a.submit_transaction(a_transactions[0].clone()).await;
    a.submit_transaction(a_transactions[1].clone()).await;
    a.submit_transaction(a_transactions_2[0].clone()).await;

    //submit the first block
    let new_block = mine_pending_transactions(&a.blockchain, a_transactions.clone());
    let block_submission_res = a.submit_mined_block(new_block).await;
    assert_eq!(Ok(()), block_submission_res);
    assert_eq!(a.blockchain.chain[1].transactions, a_transactions);
    assert_eq!(1, a.pending_transactions.len());

    //submit the second block (just one transaction)
    let new_block = mine_pending_transactions(&a.blockchain, a_transactions_2.clone());
    let block_submission_res = a.submit_mined_block(new_block).await;
    assert_eq!(Ok(()), block_submission_res);
    assert_eq!(a.blockchain.chain[1].transactions, a_transactions);
    assert!(a.pending_transactions.is_empty());

    let mut b = Node::new();
    let b_transactions = vec![
        Transaction {
            timestamp: 3,
            sender: "Spock".to_string(),
            receiver: "Kirk".to_string(),
            amount: 100,
        },
        Transaction {
            timestamp: 1,
            sender: "Picard".to_string(),
            receiver: "Janeway".to_string(),
            amount: 100,
        },
    ];

    b.submit_transaction(b_transactions[0].clone()).await;
    b.submit_transaction(b_transactions[1].clone()).await;

    let new_block = mine_pending_transactions(&b.blockchain, b_transactions.clone());
    let block_submission_res = b.submit_mined_block(new_block).await;

    assert_eq!(Ok(()), block_submission_res);
    assert!(b.pending_transactions.is_empty());
    assert_eq!(b.blockchain.chain[1].transactions, b_transactions);

    // when b shows a its blockchain, its blockchain should not be replaced
    a.receive_chain(&b.blockchain).await;
    assert_eq!(a.blockchain.chain[1].transactions, a_transactions);

    // when a shows b its blockchain, b should replace its blockchain
    // and put all its transactions back into a pending state
    b.receive_chain(&a.blockchain).await;
    assert_eq!(b.blockchain.chain[1].transactions, a_transactions);
    assert_eq!(b.blockchain.chain[2].transactions, a_transactions_2);

    // transactions that we previously confirmed in b's blockchain should be pending again
    assert!(b.pending_transactions.contains(&b_transactions[0]));
    assert!(b.pending_transactions.contains(&b_transactions[1]));

    //mine the b transactions again
    let new_block = mine_pending_transactions(&b.blockchain, b_transactions.clone());
    let block_submission_res = b.submit_mined_block(new_block).await;

    //make sure the pending transactions are now empty
    assert_eq!(Ok(()), block_submission_res);
    assert!(b.pending_transactions.is_empty());

    //make sure all of the transactions are now in b's blockchain
    assert_eq!(b.blockchain.chain[1].transactions, a_transactions);
    assert_eq!(b.blockchain.chain[2].transactions, a_transactions_2);
    assert_eq!(b.blockchain.chain[3].transactions, b_transactions);

    //a should see b's blockchain and accept it
    a.receive_chain(&b.blockchain).await;
    //make sure all of the transactions are now in a's blockchain
    assert_eq!(a.blockchain.chain[1].transactions, a_transactions);
    assert_eq!(a.blockchain.chain[2].transactions, a_transactions_2);
    assert_eq!(a.blockchain.chain[3].transactions, b_transactions);

    // now and a nd b both have consensus!
}
