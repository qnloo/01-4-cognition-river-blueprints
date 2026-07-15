# P-RZZH-040-QNLOO-2026-v2: 多级收益自动分账专利样板间 (Rust 实现)

本样板间完整实现了 **《一种基于智能合约的认知资产多级收益自动分账系统及方法》** 专利中的多路径级联清分模型、18级依赖代数截断、多父节点指纹相似度分成、防重放Nonces、并发乐观锁版本校验，以及Checks-Effects-Interactions拉式安全划账。

## 🏛️ 一、 专利权利要求映射说明

| 专利保护点 | 对应代码位置 | 物理安全与清算作用 |
| :--- | :--- | :--- |
| **独立权利要求 1 / 4 (三级分账与状态机转换系统及方法)** | `src/contract.rs`中的整个数据结构和流转控制 | 1. 递归构建树状拓扑并硬限依赖代数限制；2. 执行 75%/10%/15% 非对称级联拉式分配记账；3. 结合时间戳控制保护期状态转化。 |
| **从属权利要求 2 (18级深度硬截断)** | `src/contract.rs`中的`register_protocol`内`calculate_depth` | 硬性截断子协议至根元协议的递归继承代数为 18 级，防范栈溢出与恶性爆 Gas 攻击。 |
| **从属权利要求 3 (非对称三级分成比例)** | `src/contract.rs`中的`execute_billing`计算部分 | 规定孙协议自留 75%，父代 10%，祖父代 15%（且祖父代分成比例固定高于父代）。 |
| **从属权利要求 4 / 9 / 10 (相似度权重与沙箱防重放触发)** | `src/contract.rs`中的`execute_billing`与`register_protocol` | 1. 读取各父节点指纹相似度进行非对称权重拨付；2. 校验 Nonces 防范重放攻击。 |
| **从属权利要求 5 (Checks-Effects-Interactions 安全提款防线)** | `src/contract.rs`中的`claim_payout` | 严格遵守 `Checks -> Effects -> Interactions` 顺序：先校验、再将合约内余额清零、最后执行划转，阻断外部回环重入套利。 |
| **从属权利要求 6 (并发控制乐观锁)** | `src/contract.rs`中的`execute_billing`与`ProtocolNode::version` | 对协议节点执行并发版本校验，若并发调用时 expected_version 不一致则回滚，防止状态失步冲突。 |
| **独立权利要求 7 / 8 (合约端清算与沙箱端触发方法)** | `src/contract.rs`与`src/main.rs` | 实证模拟逻辑行为体在隔离沙箱完成效果核验后，携带 nonce 向去中心化确权账本请求 execute_billing 的联调控制流。 |

## 🚀 二、 如何本地编译并运行验证

确保本地已安装 Rust 环境 (含 `cargo`)，在当前目录下执行：

```bash
# 运行实证测试用例（三级分成级联记账、Checks-Effects-Interactions提款防御、并发乐观锁、Nonce防重放、20年到期反哺及18级深度硬限熔断）
cargo run --release
```

项目依赖 `serde` & `serde_json` (序列化支持) 与 `sha2` (哈希校验)。

---

# P-RZZH-040-QNLOO-2026-v2: Multi-Tier Automatic Settlement Patent Blueprint (Rust Implementation)

This blueprint provides a reference implementation of the multi-path cascading clearing model, 18-generation dependency algebra truncation, multi-parent fingerprint similarity splits, anti-replay Nonces, optimistic concurrency lock validation, and Checks-Effects-Interactions pull-based withdrawal security defined in the patent **"A Multi-tier Automatic Settlement System and Method for Cognitive Assets Based on Smart Contracts"**.

## 🏛️ I. Patent Claims Mapping Reference

| Patent Claim / Protection Point | Corresponding Code Location | Physical Security & Clearing Objective |
| :--- | :--- | :--- |
| **Independent Claims 1 / 4 (3-Tier Settlement & State Machine)** | The data structures and control flow in `src/contract.rs` | 1. Recursively constructs tree topologies; 2. Executes 75%/10%/15% asymmetric cascading pull split payouts; 3. Controls protection state transition via block timestamps. |
| **Dependent Claim 2 (18-level Tree Truncation)** | `calculate_depth` inside `register_protocol` in `src/contract.rs` | Stops recursive inheritances strictly at 18 generations from Gen 1 to defend against stack overflows and gas exhaustion attacks. |
| **Dependent Claim 3 (Asymmetric 75/10/15 splits)** | Computation module in `execute_billing` in `src/contract.rs` | Allocates 75% for grandchild, 10% for parent, and 15% for grandparent (ensuring grandparent's share remains higher than parent's). |
| **Dependent Claims 4 / 9 / 10 (Similarity Audit & Replay Block)** | `execute_billing` & `register_protocol` in `src/contract.rs` | 1. Queries parent similarity metrics to allocate weights; 2. Enforces unique nonces to deny transaction replays. |
| **Dependent Claim 5 (Reentrancy Guard via Checks-Effects-Interactions)** | `claim_payout` in `src/contract.rs` | Enforces Checks -> Effects -> Interactions: validates balances, clears state to zero, and triggers actual transfer to reject recursive callback exploits. |
| **Dependent Claim 6 (Optimistic Concurrency Lock)** | `execute_billing` and `ProtocolNode::version` in `src/contract.rs` | Compares estimated and actual block versions, rolls back state upon mismatch to deny overlapping writes. |
| **Independent Claims 7 / 8 (Contract Clearance & Sandbox Trigger)** | `src/contract.rs` and `src/main.rs` | Simulates the orchestration flow where local agents, post-sandbox audit validation, dispatch nonce-locked execute_billing requests to the ledger. |

## 🚀 II. Local Compilation & Verification Guide

Ensure you have the Rust environment (with `cargo` installed), run the following in the project root:

```bash
# Run the verification cases (3-tier payouts, CEI withdrawal, optimistic locking, Nonces, 20-year backfeed, and 18-gen cutoff)
cargo run --release
```

Dependencies include `serde` & `serde_json` (JSON serialization) and `sha2` (HMAC cryptographic hash).
