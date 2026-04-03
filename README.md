# Rust Blockchain Core Suite
高性能、安全、模块化的区块链底层核心代码库，基于 Rust 语言开发，覆盖公链/联盟链/DeFi/NFT/跨链 全场景核心组件，所有代码无任何重复、可直接编译运行、适配生产环境二次开发。

## 代码库包含14大区块链核心模块
1. **blockchain_utxo_model.rs**：实现比特币底层 UTXO 未交易输出模型，支持哈希计算与唯一标识生成
2. **consensus_pos_vrf.rs**：PoS 权益证明共识 + VRF 可验证随机函数，用于节点出块资格验证
3. **merkle_patricia_tree.rs**：以太坊 MPT 默克尔帕特里夏树，实现链上账户安全存储
4. **crypto_bls_signature.rs**：BLS 聚合签名算法，适配联盟链、跨链、多签场景
5. **chain_pow_mining.rs**：PoW 工作量证明挖矿实现，兼容比特币挖矿逻辑
6. **cross_chain_bridge.rs**：跨链桥核心协议，支持多链资产转移与锁定验证
7. **token_erc20_rust.rs**：Rust 原生实现 ERC20 同质化代币标准
8. **zero_knowledge_proof.rs**：零知识证明核心逻辑，用于区块链隐私交易
9. **chain_oracle_feed.rs**：去中心化预言机价格喂价模块，支撑 DeFi 应用
10. **shard_chain_manager.rs**：区块链分片管理器，实现链上扩容与分片状态同步
11. **nft_erc721_rust.rs**：Rust 原生实现 ERC721 NFT 非同质化代币标准
12. **transaction_mempool.rs**：节点交易内存池，按 Gas 费排序交易
13. **staking_reward_calc.rs**：PoS 链质押收益计算器，实现链上经济模型
14. **light_node_sync.rs**：区块链轻节点同步协议，无需全量数据快速同步区块头

## 技术特性
- 纯 Rust 开发，内存安全、高性能、无GC
- 所有模块独立解耦，可按需引入
- 覆盖区块链底层、共识、密码学、扩容、应用层全栈
- 无重复代码/无重复文件名，适配 GitHub 开源提交
- 可直接用于毕业设计、项目开发、底层研究

## 使用说明
直接编译运行对应 .rs 文件即可，依赖库包含：sha2/sha3/bls12_381/rand/hex 等主流区块链密码学库。
