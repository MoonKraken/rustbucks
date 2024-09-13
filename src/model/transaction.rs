use fixed::types::I32F32;

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct Transaction {
    pub sender: String,
    pub receiver: String,
    pub amount: I32F32,
    pub timestamp: i64,
}
