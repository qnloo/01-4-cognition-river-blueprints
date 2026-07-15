# P-RZZH-010-QNLOO-2026-v2: 三层分离系统架构专利样板间 (Rust 实现)

本样板间完整实现了 **《一种基于三层分离的多智能体系统运行系统及方法》** 专利中的核心软硬件分离逻辑与四大物理防御边界。

## 🏛️ 一、 专利权利要求映射说明

| 专利保护点 | 对应代码位置 | 物理安全作用 |
| :--- | :--- | :--- |
| **独立权利要求 1 / 8 (三层数据调度)** | `src/cognition.rs`中的`run_cortex_inference_session` | 串联资产层只读隧道获取、认知层聚合熵计算、因果签名及销毁全流程。 |
| **从属权利要求 3 (三遍物理覆写消磁)** | `src/cognition.rs`中的`physical_scrub` | 强力覆写内存空间 (0x00 -> 0xFF -> 随机数) 并调用架构级缓存清刷，防止侧信道冷启动提取。 |
| **从属权利要求 4 (指令防重放指纹公式)** | `src/cognition.rs`中的`calculate_logic_entropy` & `generate_causal_output_hash` | 基于 Shannon 熵的控制指令量化，结合微秒级物理时序特征计算 HMAC 出生证明。 |
| **独立权利要求 5 (语义暴食拦截与熔断)** | `src/execution.rs`中的`receive_and_execute` | 校验缓冲区限额 (Theta_max)，若溢出则启动网络重置。 |
| **从属权利要求 6 (特异 TCP RST 报文)** | `src/execution.rs`中的`emit_forensic_rst_signal` | 在熔断发生时，强行构造带有窗口值字段为 `0xABCD` 的报文广播取证指纹。 |
| **独立权利要求 7 (只读资产层管理)** | `src/asset.rs`中的`request_read_tunnel` & `recycle_tunnel_memory` | 校验签名权限，通过只读物理隧道单向映射切片，完成后实施中和消退。 |

## 🚀 二、 如何本地编译并运行验证

确保本地已安装 Rust 环境 (含 `cargo`)，在当前目录下执行：

```bash
# 运行实证测试用例（正常流程、语义暴食拦截、重放拦截、超时熔断）
cargo run --release
```

项目依赖 `rand` (随机数生成)、`sha2` 与 `hmac` (因果序哈希校验)。已进行跨 CPU 架构兼容适配（支持 x86_64 及 Apple Silicon/ARM 架构的 Cache Flush 模拟）。

---

# P-RZZH-010-QNLOO-2026-v2: Three-Layer Separation System Patent Blueprint (Rust Implementation)

This blueprint provides a reference implementation of the core hardware-software separation logic and four physical defense boundaries defined in the patent **"A Run-time System and Method for Multi-agent Systems Based on Three-layer Separation"**.

## 🏛️ I. Patent Claims Mapping Reference

| Patent Claim / Protection Point | Corresponding Code Location | Physical Security Objective |
| :--- | :--- | :--- |
| **Independent Claims 1 / 8 (Three-layer Data Orchestration)** | `run_cortex_inference_session` in `src/cognition.rs` | Orchestrates read-only tunnel recovery, rule-cognition entropy calculation, causal HMACs, and secure memory purges. |
| **Dependent Claim 3 (3-Pass Physical Overwrite Degaussing)** | `physical_scrub` in `src/cognition.rs` | Volatile-overwrites memory blocks (0x00 -> 0xFF -> random) and flushes CPU cache lines to prevent cold-boot extraction. |
| **Dependent Claim 4 (Anti-Replay Instruction Fingerprint)** | `calculate_logic_entropy` & `generate_causal_output_hash` in `src/cognition.rs` | Measures Shannon entropy of control streams coupled with microsecond-level timestamps to generate HMAC certificates. |
| **Independent Claim 5 (Semantic Binge Attack Interception)** | `receive_and_execute` in `src/execution.rs` | Audits instruction payload scale against ceiling limits (Theta_max) and triggers immediate network shutdown. |
| **Independent Claim 6 (Specific TCP RST Forensic Frame)** | `emit_forensic_rst_signal` in `src/execution.rs` | Constructs special TCP RST packets with window size 0xABCD to broadcast forensic signatures upon meltdown. |
| **Independent Claim 7 (Isolated Asset Layer Management)** | `request_read_tunnel` & `recycle_tunnel_memory` in `src/asset.rs` | Validates cortex credentials, opens single-directional read-only mapping tunnels, and neutralizes residual charges post-read. |

## 🚀 II. Local Compilation & Verification Guide

Ensure you have the Rust environment (with `cargo` installed), run the following in the project root:

```bash
# Run the physical defense verification suite (compliant session, semantic binge blocking, anti-replay, TTL timeout)
cargo run --release
```

Dependencies include `rand` (CSPRNG), `sha2`, and `hmac` (causal sequencing verification). The project has been fully cross-compiled and tested for both x86_64 and Apple Silicon/ARM platforms (with native physical cache line flush instructions natively simulated/supported).
