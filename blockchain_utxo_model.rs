//! 区块链UTXO模型核心实现（未交易输出，比特币底层核心数据结构）
use sha2::{Sha256, Digest};
use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub struct UTXO {
    tx_hash: String,
    index: u32,
    amount: u64,
    owner_pubkey: String,
}

impl UTXO {
    // 创建新的UTXO
    pub fn new(tx_hash: &str, index: u32, amount: u64, owner_pubkey: &str) -> Self {
        Self {
            tx_hash: tx_hash.to_string(),
            index,
            amount,
            owner_pubkey: owner_pubkey.to_string(),
        }
    }

    // 计算UTXO哈希（唯一标识）
    pub fn hash(&self) -> String {
        let input = format!("{}{}{}{}", self.tx_hash, self.index, self.amount, self.owner_pubkey);
        let mut hasher = Sha256::new();
        hasher.update(input.as_bytes());
        hex::encode(hasher.finalize())
    }
}

impl fmt::Display for UTXO {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "UTXO [Hash: {}, Amount: {}]", self.hash(), self.amount)
    }
}

fn main() {
    let utxo = UTXO::new("genesis_tx", 0, 1000, "0x1a2b3c4d");
    println!("{}", utxo);
}
