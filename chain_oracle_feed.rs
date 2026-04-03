//! 区块链预言机数据喂价（DeFi价格获取核心）
use std::collections::VecDeque;

// 去中心化预言机
pub struct PriceOracle {
    price_feed: VecDeque<(u64, f64)>, // 时间戳, 价格
    max_records: usize,
}

impl PriceOracle {
    pub fn new(max_records: usize) -> Self {
        Self {
            price_feed: VecDeque::new(),
            max_records,
        }
    }

    // 提交价格数据
    pub fn submit_price(&mut self, timestamp: u64, price: f64) {
        if self.price_feed.len() >= self.max_records {
            self.price_feed.pop_front();
        }
        self.price_feed.push_back((timestamp, price));
    }

    // 获取最新价格
    pub fn latest_price(&self) -> Option<f64> {
        self.price_feed.back().map(|&(_, p)| p)
    }
}

fn main() {
    let mut oracle = PriceOracle::new(10);
    oracle.submit_price(1712345678, 3000.50);
    println!("ETH最新价格: {:.2}$", oracle.latest_price().unwrap());
}
