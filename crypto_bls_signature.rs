//! 区块链BLS聚合签名（联盟链/跨链签名专用）
use bls12_381::{G1Projective, Scalar};
use rand::rngs::OsRng;

// BLS密钥对生成
pub fn bls_keygen() -> (Scalar, G1Projective) {
    let secret = Scalar::random(&mut OsRng);
    let public = G1Projective::generator() * secret;
    (secret, public)
}

// 单签名
pub fn bls_sign(secret: &Scalar, msg: &[u8]) -> G1Projective {
    let msg_hash = G1Projective::hash_to_curve(msg, b"BLS_SIGN", b"");
    msg_hash * secret
}

// 聚合签名（多节点签名合并）
pub fn aggregate_signatures(sigs: &[G1Projective]) -> G1Projective {
    sigs.iter().fold(G1Projective::identity(), |acc, s| acc + s)
}

fn main() {
    let (sk, pk) = bls_keygen();
    let sig = bls_sign(&sk, b"chain_transfer_100");
    println!("BLS公钥: {:?}", pk);
    println!("BLS签名: {:?}", sig);
}
