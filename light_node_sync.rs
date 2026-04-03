//! 区块链轻节点同步协议（无需下载全量区块）
use sha2::Sha256;

#[derive(Debug)]
pub struct LightNode {
    chain_id: String,
    latest_block_hash: String,
    sync_height: u64,
}

impl LightNode {
    pub fn new(chain_id: &str) -> Self {
        Self {
            chain_id: chain_id.to_string(),
            latest_block_hash: "genesis".to_string(),
            sync_height: 0,
        }
    }

    // 轻节点同步区块头
    pub fn sync_header(&mut self, height: u64, block_hash: &str) {
        if height > self.sync_height {
            self.sync_height = height;
            self.latest_block_hash = block_hash.to_string();
        }
    }

    // 验证区块哈希
    pub fn verify_hash(&self, test_hash: &str) -> bool {
        self.latest_block_hash == test_hash
    }
}

fn main() {
    let mut node = LightNode::new("rust_chain_1");
    node.sync_header(1000, "0xLightHash456");
    println!("轻节点同步高度: {}", node.sync_height);
}
