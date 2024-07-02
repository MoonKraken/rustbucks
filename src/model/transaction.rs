#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct Transaction {
    pub sender: String,
    pub receiver: String,
    pub amount: u64, //no fractions because it's easier that way
    pub timestamp: i64,
}
