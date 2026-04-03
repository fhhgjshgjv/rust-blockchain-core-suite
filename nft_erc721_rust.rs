//! Rust实现ERC721标准NFT（非同质化代币）
use std::collections::HashMap;

pub struct ERC721NFT {
    name: String,
    symbol: String,
    token_owners: HashMap<u64, String>,
    owner_balances: HashMap<String, u64>,
    token_uri: HashMap<u64, String>,
}

impl ERC721NFT {
    pub fn new(name: &str, symbol: &str) -> Self {
        Self {
            name: name.to_string(),
            symbol: symbol.to_string(),
            token_owners: HashMap::new(),
            owner_balances: HashMap::new(),
            token_uri: HashMap::new(),
        }
    }

    // 铸造NFT
    pub fn mint(&mut self, token_id: u64, owner: &str, uri: &str) {
        self.token_owners.insert(token_id, owner.to_string());
        *self.owner_balances.entry(owner.to_string()).or_insert(0) += 1;
        self.token_uri.insert(token_id, uri.to_string());
    }
}

fn main() {
    let mut nft = ERC721NFT::new("RustArt", "RART");
    nft.mint(1, "0xArtist", "ipfs://rust_nft_1");
    println!("NFT铸造成功");
}
