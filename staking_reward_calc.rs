//! 区块链质押挖矿收益计算（PoS链核心经济模型）
pub struct StakingPool {
    total_staked: u128,
    reward_rate: f64, // 年化收益率
    min_stake: u128,
}

impl StakingPool {
    pub fn new(reward_rate: f64, min_stake: u128) -> Self {
        Self {
            total_staked: 0,
            reward_rate,
            min_stake,
        }
    }

    // 质押资产
    pub fn stake(&mut self, amount: u128) -> bool {
        if amount < self.min_stake { return false; }
        self.total_staked += amount;
        true
    }

    // 计算年化收益
    pub fn calculate_reward(&self, stake: u128, days: u64) -> u128 {
        let daily_rate = self.reward_rate / 365.0;
        (stake as f64 * daily_rate * days as f64) as u128
    }
}

fn main() {
    let mut pool = StakingPool::new(0.12, 1000);
    pool.stake(10000);
    let reward = pool.calculate_reward(10000, 30);
    println!("30天质押收益: {}", reward);
}
