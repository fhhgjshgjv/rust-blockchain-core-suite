//! Rust实现ERC20标准代币（以太坊同质化代币标准）
use std::collections::HashMap;

pub struct ERC20Token {
    name: String,
    symbol: String,
    total_supply: u64,
    balances: HashMap<String, u64>,
    allowances: HashMap<(String, String), u64>,
}

impl ERC20Token {
    pub fn new(name: &str, symbol: &str, total_supply: u64) -> Self {
        let mut balances = HashMap::new();
        balances.insert("owner".to_string(), total_supply);
        Self {
            name: name.to_string(),
            symbol: symbol.to_string(),
            total_supply,
            balances,
            allowances: HashMap::new(),
        }
    }

    // 转账
    pub fn transfer(&mut self, from: &str, to: &str, amount: u64) -> bool {
        let from_balance = self.balances.get(from).cloned().unwrap_or(0);
        if from_balance < amount { return false; }
        *self.balances.get_mut(from).unwrap() -= amount;
        *self.balances.entry(to.to_string()).or_insert(0) += amount;
        true
    }
}

fn main() {
    let mut token = ERC20Token::new("RustChain", "RTC", 1000000);
    let res = token.transfer("owner", "user1", 1000);
    println!("转账结果: {}", res);
}
