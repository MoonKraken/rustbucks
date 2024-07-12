use chrono::Utc;
use fixed::types::I32F32;
use rustbucks::{mine::mine_pending_transactions, model::{blockchain::Blockchain, transaction::Transaction}};


fn main() {
    let bc = Blockchain::new();

    let transaction = Transaction {
        amount: I32F32::from_num(50),
        sender: "me".to_string(),
        receiver: "you".to_string(),
        timestamp: Utc::now().timestamp(),
    };

    let new_block = mine_pending_transactions(&bc, vec![transaction]);
    dbg!(new_block);
}
