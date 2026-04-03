//! 区块链PoW挖矿实现（比特币/莱特币核心共识）
use sha256::digest;

#[derive(Debug)]
pub struct Block {
    index: u32,
    prev_hash: String,
    data: String,
    nonce: u64,
    hash: String,
}

impl Block {
    // 挖矿：计算符合难度的哈希
    pub fn mine(index: u32, prev_hash: &str, data: &str, difficulty: usize) -> Self {
        let mut nonce = 0;
        loop {
            let hash = digest(format!("{}{}{}{}", index, prev_hash, data, nonce));
            if hash.starts_with(&"0".repeat(difficulty)) {
                return Self {
                    index,
                    prev_hash: prev_hash.to_string(),
                    data: data.to_string(),
                    nonce,
                    hash,
                };
            }
            nonce += 1;
        }
    }
}

fn main() {
    let block = Block::mine(1, "genesis_hash", "tx:Alice->Bob 50", 4);
    println!("挖矿成功: {:?}", block);
}
