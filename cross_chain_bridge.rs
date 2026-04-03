//! 跨链桥核心协议（区块链资产跨链转移）
use std::fmt;

#[derive(Debug, PartialEq)]
pub enum Chain {
    Ethereum,
    Solana,
    Polkadot,
}

#[derive(Debug)]
pub struct CrossChainTx {
    from_chain: Chain,
    to_chain: Chain,
    sender: String,
    receiver: String,
    amount: u64,
    lock_id: String,
}

impl CrossChainTx {
    // 创建跨链交易
    pub fn new(from: Chain, to: Chain, sender: &str, receiver: &str, amount: u64) -> Self {
        let lock_id = hex::encode(format!("{:?}{:?}{}", from, to, amount));
        Self {
            from_chain: from,
            to_chain: to,
            sender: sender.to_string(),
            receiver: receiver.to_string(),
            amount,
            lock_id,
        }
    }
}

impl fmt::Display for CrossChainTx {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?} -> {:q} | 金额: {}", self.from_chain, self.to_chain, self.amount)
    }
}

fn main() {
    let tx = CrossChainTx::new(Chain::Ethereum, Chain::Solana, "0xAlice", "So1Bob", 100);
    println!("跨链交易: {}", tx);
}
