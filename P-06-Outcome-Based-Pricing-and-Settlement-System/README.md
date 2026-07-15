# P-RZZH-060-QNLOO-2026-v2: 效果验证结算与路由分流专利样板间 (Rust 实现)

本样板间完整实现了 **《一种基于效果验证的动态定价与验证系统及方法》** 专利中的三层价格网关状态机跳转、本地 TEE 效果验证沙箱、真人服务并发会话容量溢出路由分流、同源子代版权分成返还，以及拉式提款 Checks-Effects-Interactions 安全防重入、并发乐观锁版本校验与数字人民币锁步直连接口。

## 🏛️ 一、 专利权利要求映射说明

| 专利保护点 | 对应代码位置 | 物理安全与清算作用 |
| :--- | :--- | :--- |
| **独立权利要求 1 / 4 (效果验证与动态路由清分系统及方法)** | `src/gateway.rs`、`src/sandbox.rs`、`src/routing_contract.rs` | 1. 边缘反代维护免费试用-付费AI-真人效果托管三层状态；2. 本地 TEE 安全评估并输出脱敏证明与签名，擦除隐私；3. 容量监控溢出路由分流。 |
| **从属权利要求 2 (Pull Payment 架构与待提清分)** | `src/routing_contract.rs` 的 `withdraw` 与待提存储 | 清结算时不使用推送模式（Push），只修改账本余额，让外部通过提款交易 Pull，防止合约并发重入与 Gas 爆表。 |
| **从属权利要求 3 (同源版权返还提成)** | `src/routing_contract.rs` 的 `execute_billing` 内分成逻辑 | 子协议被分流路由且效果验证达标后，自动扣减该次交易的 10% 级联上缴给主协议所有者钱包，保护初始专家版权。 |
| **从属权利要求 5 (状态机具体计数限额跳转)** | `src/gateway.rs` 和 `src/routing_contract.rs` 的网关调用 | 规定 $C_{\text{trial}} \le 3$ 免费，随后 $C_{\text{paid}} \le 3$ 预存，超限后强行拦截并跳转至 `OutcomeHuman`，要求用户质押全部托管资金。 |
| **从属权利要求 6 / 9 / 10 (数币锁步防双花、防重放与乐观锁并发)** | `src/routing_contract.rs` 和 `src/sandbox.rs` | 1. 签名混入 Nonce 防重放；2. 结算增加 session version 乐观锁校验；3. 锁步数币状态修改并等待回执解锁，防止异步通信双花。 |
| **独立权利要求 7 / 8 (合约端与沙箱端审计生成运行方法)** | `src/routing_contract.rs` 与 `src/sandbox.rs` | 实证模拟本地 TEE 隔离区读取敏感指标并进行消磁清空，签名发送链上，合约验签通过后依据断言状态执行分润或退款豁免。 |

## 🚀 二、 如何本地编译并运行验证

确保本地已安装 Rust 环境 (含 `cargo`)，在当前目录下执行：

```bash
# 运行实证测试用例（三层网关跳转、容量溢出分流、同源版权分成、达标清结算与未达标豁免退款、防重放乐观锁并发与CEI防重入提款）
cargo run --release
```

项目依赖 `serde` & `serde_json` (序列化支持) 与 `sha2` (哈希算法校验)。

---

# P-RZZH-060-QNLOO-2026-v2: Outcome-Based Settlement & Adaptive Routing Patent Blueprint (Rust Implementation)

This blueprint provides a reference implementation of the three-tiered price gateway state machine transition, local TEE outcome validation sandbox, human expert concurrent session overflow routing redirect, homocentric descendant copyright royalty feedback, and secure pull-based payment (Checks-Effects-Interactions) reentrancy prevention, optimistic concurrency lock validation, and lock-step digital currency interface defined in the patent **"A Run-time Valuation and Intercept System and Method for Multi-agent Systems Based on Outcome Verification"**.

## 🏛️ I. Patent Claims Mapping Reference

| Patent Claim / Protection Point | Corresponding Code Location | Physical Security & Clearing Objective |
| :--- | :--- | :--- |
| **Independent Claims 1 / 4 (Outcome Gate & Routing)** | `src/gateway.rs`, `src/sandbox.rs` & `src/routing_contract.rs` | 1. Enforces FreeTrial-PaidAI-OutcomeHuman three-tiered states; 2. Runs TEE-based outcome reviews to desensitize and scrub payload details; 3. Redirects sessions upon queue overflow. |
| **Dependent Claim 2 (Pull-Payment Balance Allocation)** | `withdraw` and balance registry in `src/routing_contract.rs` | Avoids pushing payouts directly; registers balances for users to pull via transaction, preventing loop callback reentrancy and out-of-gas events. |
| **Dependent Claim 3 (Homocentric 10% Copyright Royalty)** | Split mapping inside `execute_billing` in `src/routing_contract.rs` | Taps 10% of sub-agent transaction splits to recursively reward main expert creators, preserving core design rights. |
| **Dependent Claim 5 (Gating Threshold Limits)** | `src/gateway.rs` and execution hooks in `src/routing_contract.rs` | Enforces Trial limits ($C_{\text{trial}} \le 3$) and PaidAI limits ($C_{\text{paid}} \le 3$) to move sessions into escrow-locked status. |
| **Dependent Claims 6 / 9 / 10 (Lock-Step Settlement Guard)** | `src/routing_contract.rs` and `src/sandbox.rs` | 1. Signs nonces to deny message reuse; 2. Enforces session version logic checks; 3. Locks digital e-currency states awaiting verification. |
| **Independent Claims 7 / 8 (Contract Reconciliation & Sandbox Verification)** | `src/routing_contract.rs` and `src/sandbox.rs` | Extracts performance logs inside local TEE sandboxes, signs them using hardware keys, and submits proofs to execute splits or refund exemptions. |

## 🚀 II. Local Compilation & Verification Guide

Ensure you have the Rust environment (with `cargo` installed), run the following in the project root:

```bash
# Run the verification cases (three-tier gateway transitions, capacity overflow routing, copyright royalty, refunds, and reentrancy blocks)
cargo run --release
```

Dependencies include `serde` & `serde_json` (JSON serialization) and `sha2` (HMAC cryptographic hash).
