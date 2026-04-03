//! 区块链交易内存池（节点未打包交易缓存）
use std::collections::BinaryHeap;

#[derive(Debug, Eq, PartialEq)]
pub struct Transaction {
    hash: String,
    gas_price: u64,
    size: u32,
}

impl Ord for Transaction {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.gas_price.cmp(&other.gas_price)
    }
}

impl PartialOrd for Transaction {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

pub struct Mempool {
    txs: BinaryHeap<Transaction>,
}

impl Mempool {
    pub fn new() -> Self {
        Self { txs: BinaryHeap::new() }
    }

    // 添加交易到内存池
    pub fn add_tx(&mut self, hash: &str, gas_price: u64, size: u32) {
        self.txs.push(Transaction {
            hash: hash.to_string(),
            gas_price,
            size,
        });
    }

    // 获取最高手续费交易
    pub fn pop_highest_gas(&mut self) -> Option<Transaction> {
        self.txs.pop()
    }
}

fn main() {
    let mut pool = Mempool::new();
    pool.add_tx("0xTx123", 50, 256);
    let best_tx = pool.pop_highest_gas();
    println!("优先打包交易: {:?}", best_tx);
}
