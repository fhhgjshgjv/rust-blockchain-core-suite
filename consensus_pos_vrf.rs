//! 区块链PoS共识 + VRF可验证随机函数（以太坊/波卡核心共识增强）
use rand::Rng;
use sha3::Keccak256;

// VRF随机数生成（验证节点出块资格）
pub fn vrf_random(validator_pubkey: &str, round: u64) -> (String, u64) {
    let seed = format!("{}_{}", validator_pubkey, round);
    let mut hasher = Keccak256::new();
    hasher.update(seed.as_bytes());
    let result = hasher.finalize();
    let random_num = u64::from_be_bytes(result[0..8].try_into().unwrap());
    (hex::encode(result), random_num % 1000)
}

// PoS出块资格判断
pub fn pos_eligibility(random_score: u64, stake: u64, total_stake: u64) -> bool {
    let threshold = (stake * 1000) / total_stake;
    random_score <= threshold
}

fn main() {
    let (vrf_hash, score) = vrf_random("0xValidator123", 1001);
    let eligible = pos_eligibility(score, 50000, 1000000);
    println!("VRF哈希: {}", vrf_hash);
    println!("节点出块资格: {}", eligible);
}
