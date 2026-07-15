# P-RZZH-030-QNLOO-2026-v2: 本地隐私确权与同步专利样板间 (Rust 实现)

本样板间完整实现了 **《一种基于哈希指纹的认知资产本地确权与增量同步系统及方法》** 专利中的本地离线向量哈希计算、分布式零泄漏上链、Loopback 隔离和会话沙箱物理电荷消磁。

## 🏛️ 一、 专利权利要求映射说明

| 专利保护点 | 对应代码位置 | 物理安全作用 |
| :--- | :--- | :--- |
| **独立权利要求 1 / 7 (三层架构与增量同步方法)** | `src/local_zone.rs`、`src/ledger.rs` 与 `src/sandbox.rs` | 完整定义本地安全区、确权链及安全飞地沙箱的协同数据调度流。 |
| **从属权利要求 2 / 8 (指纹级联哈希计算)** | `src/local_zone.rs`中的`generate_fingerprint` | 公式实现：$H_{\text{chunk}} = \text{SHA256}(\vec{v}_{\text{emb}} \| \text{TextContent} \| \text{Salt})$，并对 SyncPacket 实施完全不含明文的所有权登记。 |
| **从属权利要求 3 / 10 (只读沙箱消磁防冷启动)** | `src/sandbox.rs`中的`secure_erase` | 1. 物理安全飞地挂载只读属性；2. 多遍覆写 (全零 -> 全一 -> 随机数) 并调用架构级缓存清刷，防止侧信道冷启动电荷提取。 |
| **从属权利要求 4 (向量库 Loopback 绑定)** | `src/local_zone.rs`中的`LocalVectorDb::query_semantic` | 向量库强制绑定 127.0.0.1 且限制本地 IPC 访问，拦截一切跨局域网或远程的外网探测。 |
| **从属权利要求 5 (网络连接拦截熔断)** | `src/sandbox.rs`中的`emit_tcp_rst_intercept` | 在会话结束或超时归零时，底层通信栈强制向外发送 TCP RST 报文以抹除通信连接痕迹。 |
| **从属权利要求 6 (指纹种子语法偏差水印)** | `src/local_zone.rs`中的`generate_fingerprint` | 利用哈希指纹特征改变句式空白符分布（植入零宽空格水印），作为产权司法取证的最强闭环支撑。 |
| **独立权利要求 9 (去中心化存证校验)** | `src/ledger.rs`中的`register_fingerprint` | 对 SyncPacket 进行时间戳时序防重放校验，并实现不写入任何原始明文的存证登记。 |

## 🚀 二、 如何本地编译并运行验证

确保本地已安装 Rust 环境 (含 `cargo`)，在当前目录下执行：

```bash
# 运行实证测试用例（语法偏差水印植入、分布式账本防重放上链、Loopback 仅本地 IPC 拦截、只读沙箱 3遍消磁与网络熔断）
cargo run --release
```

项目依赖 `serde` & `serde_json` (数据序列化)、`sha2` (加密哈希) 与 `rand` (防伪随机化)。

---

# P-RZZH-030-QNLOO-2026-v2: Local Privacy Confirmation & Sync Patent Blueprint (Rust Implementation)

This blueprint provides a reference implementation of the local offline vector hash calculation, distributed zero-leakage ledger registry, Loopback isolation, and session sandbox memory charge scrubbing defined in the patent **"A Cognitive Asset Local Privacy Confirmation and Incremental Sync System and Method Based on Hash Fingerprints"**.

## 🏛️ I. Patent Claims Mapping Reference

| Patent Claim / Protection Point | Corresponding Code Location | Physical Security Objective |
| :--- | :--- | :--- |
| **Independent Claims 1 / 7 (Architectural Data Sync Flow)** | `src/local_zone.rs`, `src/ledger.rs` & `src/sandbox.rs` | Defines core interactions between local secure zone, blockchain ledger, and secure enclaves. |
| **Dependent Claims 2 / 8 (Fingerprint Hash Generation)** | `generate_fingerprint` in `src/local_zone.rs` | Implements $H_{\text{chunk}} = \text{SHA256}(\vec{v}_{\text{emb}} \| \text{TextContent} \| \text{Salt})$ to submit secure SyncPackets to remote nodes with zero exposure of raw plaintext. |
| **Dependent Claims 3 / 10 (Read-Only Enclave Memory Scrubbing)** | `secure_erase` in `src/sandbox.rs` | Enforces read-only permissions in TEE enclaves and volatile-scrubs (0x00 -> 0xFF -> random noise) memory pages to deny cold-boot attacks. |
| **Dependent Claim 4 (Vector Local Loopback Protection)** | `LocalVectorDb::query_semantic` in `src/local_zone.rs` | Binds local vector DB instances directly to 127.0.0.1 to reject all remote socket probes. |
| **Dependent Claim 5 (Forced TCP RST Socket Meltdown)** | `emit_tcp_rst_intercept` in `src/sandbox.rs` | Directly broadcasts raw TCP RST packets upon runtime session termination or timeout to destroy TCP link remnants. |
| **Dependent Claim 6 (Syntactic Drift Fingerprint Watermark)** | `generate_fingerprint` in `src/local_zone.rs` | Slightly adapts syntactic white-space distributions based on hash keys to embed invisible watermarks, serving as judicial evidence of ownership. |
| **Independent Claim 9 (Decentralized Timed IP Verification)** | `register_fingerprint` in `src/ledger.rs` | Validates timestamps of incoming SyncPackets to block replay attacks while writing no data except hash fingerprints. |

## 🚀 II. Local Compilation & Verification Guide

Ensure you have the Rust environment (with `cargo` installed), run the following in the project root:

```bash
# Run the verification cases (syntactic watermarking, zero-leak ledger sync, Loopback block, and sandbox degaussing)
cargo run --release
```

Dependencies include `serde` & `serde_json` (JSON serialization), `sha2` (cryptographic hash), and `rand` (anti-forging padding).
