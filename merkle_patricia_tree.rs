//! 区块链MPT默克尔帕特里夏树（以太坊账户存储核心）
use std::collections::HashMap;
use sha2::Sha256;

#[derive(Default)]
pub struct MerklePatriciaTree {
    nodes: HashMap<String, String>,
    root: String,
}

impl MerklePatriciaTree {
    // 插入键值对
    pub fn insert(&mut self, key: &str, value: &str) {
        let node_hash = self.hash_node(key, value);
        self.nodes.insert(key.to_string(), node_hash.clone());
        self.root = node_hash;
    }

    // 生成节点哈希
    fn hash_node(&self, key: &str, value: &str) -> String {
        let input = format!("{}:{}", key, value);
        let mut hasher = Sha256::new();
        hasher.update(input.as_bytes());
        hex::encode(hasher.finalize())
    }

    // 获取树根哈希
    pub fn root_hash(&self) -> &str {
        &self.root
    }
}

fn main() {
    let mut mpt = MerklePatriciaTree::default();
    mpt.insert("account_0x123", "balance=100ETH");
    println!("MPT根哈希: {}", mpt.root_hash());
}
