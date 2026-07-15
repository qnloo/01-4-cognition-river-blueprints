# P-RZZH-050-QNLOO-2026-v2: 逻辑连续性真诚审计专利样板间 (Rust 实现)

本样板间完整实现了 **《一种基于逻辑连续性的多智能体交互真诚度审计系统及方法》** 专利中的“需求-方案-行为-效果”时序因果对齐校验、本地 TEE 隐私脱敏签名与消磁擦除、指数滑动窗口审计积分模型，以及边缘限制网关的协议栈阶梯时延惩戒和财务冻结联动控制。

## 🏛️ 一、 专利权利要求映射说明

| 专利保护点 | 对应代码位置 | 物理安全与清算作用 |
| :--- | :--- | :--- |
| **独立权利要求 1 / 4 (时序四阶段因果对齐系统及方法)** | `src/logic_actor.rs`、`src/sincerity_audit.rs`、`src/gateway_proxy.rs` | 1. 验证需求-方案余弦相似对齐与本地动作-反馈时序因果自洽；2. 链上提交 TEE 密码学证明并用 EWMA 更新权重积分；3. 三级梯度控制物理网关时延与财务提取权。 |
| **从属权利要求 2 (自相矛盾与逻辑断裂判定)** | `src/main.rs` 的异常路径逻辑断裂测试部分 | 智能体在声称方案无效果的情况下，没有提供任何本地设备动作指纹，且依然频繁调用该方案，直接判定发生逻辑断裂并给予极低判定分值。 |
| **从属权利要求 3 (EWMA 指数衰减滑动窗口模型)** | `src/sincerity_audit.rs` 的 `submit_sincerity_proof` 核心公式 | 以增量写入方式计算最新审计积分 $W_t = 0.85 \cdot W_{t-1} + 0.15 \cdot S_t$，保证近期行为的高权重和长期记录记忆的平衡。 |
| **独立权利要求 7 / 8 / 10 (本地沙箱隐私脱敏证明与签名)** | `src/logic_actor.rs` 的 `generate_tee_proof` | 1. 在本地沙箱硬件 TEE 隔离内存中完成因果判定，擦除物理隐私参数明文；2. 用硬件私钥混入 Nonce 生成加密签名包。 |
| **从属权利要求 5 (网关协议栈物理阶梯时延注入)** | `src/gateway_proxy.rs` 的 `enforce_access_policy` | 当信誉等级处于欺诈（Fraudulent）状态时，在套接字协议握手层强制注入 $3.5\text{s} \sim 12\text{s}$ 的阶梯延时拦截白嫖。 |
| **从属权利要求 6 / 9 (并发控制与跨合约联动冻结)** | `src/sincerity_audit.rs` 和 `src/main.rs` 场景 4 | 1. 引入并发控制乐观锁版本号以防止多方异步提包冲突；2. 自动跨合约联动分账系统，对欺诈节点实施资金清算冻结。 |

## 🚀 二、 如何本地编译并运行验证

确保本地已安装 Rust 环境 (含 `cargo`)，在当前目录下执行：

```bash
# 运行实证测试用例（余弦对齐验证、TEE脱敏签名、防重放乐观锁、EWMA积分更新、网关阶梯时延惩戒、跨合约财务冻结）
cargo run --release
```

项目依赖 `serde` & `serde_json` (序列化支持) 与 `sha2` (哈希算法校验)。

---

# P-RZZH-050-QNLOO-2026-v2: Logical Continuity Sincerity Audit Patent Blueprint (Rust Implementation)

This blueprint provides a reference implementation of the "Demand-Proposal-Action-Outcome" temporal causal alignment check, local TEE data desensitization signature and memory scrubbing, Exponentially Weighted Moving Average (EWMA) sliding audit score model, and edge-intercept gateway protocol-level tiered latency penalty and financial freeze joint control defined in the patent **"A Multi-agent Interaction Sincerity Audit System and Method Based on Logical Continuity"**.

## 🏛️ I. Patent Claims Mapping Reference

| Patent Claim / Protection Point | Corresponding Code Location | Physical Security & Clearing Objective |
| :--- | :--- | :--- |
| **Independent Claims 1 / 4 (4-Phase Causal Alignment)** | `src/logic_actor.rs`, `src/sincerity_audit.rs` & `src/gateway_proxy.rs` | 1. Verifies Cosine Similarity of Demand-Proposal and causal consistency of Action-Feedback; 2. Computes dynamic reputation score via EWMA; 3. Implements 3-tiered edge latency gating and financial freeze controls. |
| **Dependent Claim 2 (Logical Breakage Identification)** | Logical anomaly test vectors in `src/main.rs` | Identifies cases where agents report failure of a proposal yet call it high-frequency without PLC action payloads, dropping scores to intercept loopholes. |
| **Dependent Claim 3 (EWMA Sliding Score Formula)** | Core algorithm inside `submit_sincerity_proof` in `src/sincerity_audit.rs` | Applies $W_t = 0.85 \cdot W_{t-1} + 0.15 \cdot S_t$ to prioritize recent metrics while balancing historical weight. |
| **Independent Claims 7 / 8 / 10 (Local TEE Sandbox Desensitization)** | `generate_tee_proof` in `src/logic_actor.rs` | 1. Obtains causal proofs inside secure TEE enclaves to strip sensitive parameter plaintexts; 2. Combines private keys and Nonces to sign proofs. |
| **Dependent Claim 5 (Gating Latency Injector)** | `enforce_access_policy` in `src/gateway_proxy.rs` | Injects physical socket-level handshake delays ranging from 3.5s to 12s to deter free-riders under Fraudulent profiles. |
| **Dependent Claims 6 / 9 (Joint Control & Concurrency Guard)** | `src/sincerity_audit.rs` and Scenario 4 in `src/main.rs` | 1. Uses concurrency lock versions to block overlapping state updates; 2. Cross-contractually triggers cascading billing modules to freeze delinquent accounts. |

## 🚀 II. Local Compilation & Verification Guide

Ensure you have the Rust environment (with `cargo` installed), run the following in the project root:

```bash
# Run the verification cases (cosine alignment, TEE desensitization, optimistic locking, EWMA updates, latency gating, and financial freezes)
cargo run --release
```

Dependencies include `serde` & `serde_json` (JSON serialization) and `sha2` (cryptographic hash).
