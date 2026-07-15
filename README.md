# 千年鹿认知体系 · 体系技术蓝图仓库｜QNLOO Cognition System · System Technical Blueprints

## 1. 本仓库定位 / Repository Profile

- **性质**：全球官方唯一《认知之河》六项核心发明专利的参考实现与开源样板间仓库。作为体系技术蓝图的官方交付载体，本仓库用于公开专利方案的标准化工程实现，供全球开发者学习、验证及合规参照。

- **Nature**: The world's official reference implementation and open-source blueprint repository for the six core invention patents of the *River of Cognition*. As the official delivery vehicle for the system's technical blueprints, this repository publicly provides standardized engineering implementations of the patented solutions for global developers to study, verify, and use as compliance reference.

- **资产形态 / Asset Format**

    - **源码**：以 Rust 语言编写的高可读性、高安全性参考实现，完整展示专利说明书所披露技术方案的工程落地形态。

    - **Source Code**: Highly readable and secure reference implementations written in Rust, fully demonstrating the engineering implementation forms of the technical solutions disclosed in the patent specifications.

- **交付节奏**：遵循“发布即交付”的核心理念，本仓库所有六项专利样板间的源码文件、目录结构及许可证协议已于 **2026年7月15日正式全量交付并开源**。全球开发者即可实时克隆、编译并运行全部实现样例。

- **Delivery Cadence**: Adhering to the core principle of "Release as Delivery," the source code, directory structures, and license agreements for all six patent blueprints have been **fully delivered and open-sourced on July 15, 2026**. Global developers can instantly clone, compile, and execute all reference implementations.

⚠️ **开发者合规与认知导流 / Developer Compliance & Theory Base**

> 运行、研究或商业适配本仓源码前，建议先行研读 **[00-QNLOO-SPL-Licenses](https://github.com/QNLOO/00-QNLOO-SPL-Licenses)** 仓库内的《主权价值协议白皮书（Sovereign Value Protocol Whitepaper）》，以获取完整的技术原理图谱与合规并网指引。

> Before compiling or commercially adapting this repository, it is highly recommended to study the *Sovereign Value Protocol Whitepaper* in **[00-QNLOO-SPL-Licenses](https://github.com/QNLOO/00-QNLOO-SPL-Licenses)** to acquire the complete legal and theoretical map.

---

## 2. 资产总览 / Asset Overview

**说明**：本仓库为《认知之河》全套六项核心发明专利的官方参考实现，每项专利对应一个独立的 Rust 样板间工程。所有代码即刻物理就绪。

**Description**: This repository is the official reference implementation for all six core invention patents of the *River of Cognition*, with each patent corresponding to an independent Rust blueprint project. All blueprints are fully operational.


### 2.1 六项专利样板间清单 / Six Patent Blueprints

| 样板间目录 / Blueprint Directory | 对应专利 / Patent | 核心功能 / Core Function | 状态 / Status |
| :--- | :---: | :--- | :---: |
| `P-01-Three-Layer-Separation-Architecture/` | P-01 | 三层物理分离、TEE消磁、防重放<br>Three-layer separation, TEE erasure, anti-replay | 已交付 / Delivered |
| `P-02-Standardized-Cognitive-Protocol-Packaging/` | P-02 | 级联哈希、自适应转译、隐蔽水印<br>Cascading hash, adaptive translation, covert watermark | 已交付 / Delivered |
| `P-03-Local-Privacy-Right-Confirmation-and-Synchronization/` | P-03 | 本地指纹确权、Loopback隔离、增量同步<br>Local fingerprinting, loopback isolation, incremental sync | 已交付 / Delivered |
| `P-04-Cascading-Revenue-Auto-Settlement/` | P-04 | 18代分账、DFS环检测、拉式提款<br>18-gen settlement, DFS cycle detection, pull payment | 已交付 / Delivered |
| `P-05-Logic-Continuity-Sincerity-Audit/` | P-05 | 四维因果审计、EWMA积分、梯度惩戒<br>4D causal audit, EWMA score, tiered penalty | 已交付 / Delivered |
| `P-06-Outcome-Based-Pricing-and-Settlement-System/` | P-06 | 三层网关、效果验证、容量分流<br>3-tier gateway, outcome verification, capacity routing | 已交付 / Delivered |

> 每个样板间目录均为独立的 Rust 工程，包含完整的 `Cargo.toml` 及可直接编译运行的全部源码文件。全局 `Cargo.toml` 位于仓库根目录。
> 
> Each blueprint directory is an independent Rust project, containing a complete `Cargo.toml` and all source files ready for compilation and execution. The global `Cargo.toml` is located in the repository root.


### 2.2 代码模块与专利权利要求映射 / Code-to-Patent Mapping

**说明**：下表列出六项专利对应的官方参考实现代码模块及其核心功能定位，供全球开发者按图索骥，精准验证每项专利权利要求的工程实现。

**Description**: The table below maps the official reference implementation code modules to their respective patents and core functions, enabling global developers to precisely verify the engineering implementation of each patent claim.

| 核心代码模块 / Core Code Module | 所属专利 / Patent | 功能定位 / Function | 对应核心权利要求 / Core Claims |
| :--- | :---: | :--- | :--- |
| `cognition.rs` | P-01 | TEE只读隧道与物理内存消磁机制<br>TEE logic tunnel & memory degaussing | 权利要求1、2、4、7<br>Claims 1, 2, 4, 7 |
| `main.rs` | P-01 | 合规会话审计与网络重置防护<br>Compliance session audit & network reset | 权利要求1、5<br>Claims 1, 5 |
| `compiler.rs` | P-02 | 引擎自适应自编译与零宽隐藏数字水印<br>Adaptive compilation & ZWSP injection | 权利要求3、4、6<br>Claims 3, 4, 6 |
| `main.rs` | P-02 | 级联哈希校验及词级出站逻辑熵审计断言<br>Cascading hash verify & output entropy audit | 权利要求1、2、5、8<br>Claims 1, 2, 5, 8 |
| `local_zone.rs` | P-03 | 本地指纹提取与 Loopback IPC 向量检索硬隔离<br>Vector local IP retrieval & Loopback firewall | 权利要求1、3、5<br>Claims 1, 3, 5 |
| `sandbox.rs` | P-03 | 隔离推理沙箱与主动 TCP RST 熔断机制<br>TSB enclave & TCP RST interface | 权利要求1、6、7<br>Claims 1, 6, 7 |
| `contract.rs` | P-04 | 18级代际限制与20年失效重定向合约<br>18-level cutoff & 20-year backfeed contract | 权利要求1、2、3、5<br>Claims 1, 2, 3, 5 |
| `withdrawal_module.rs` | P-04 | 拉式资金结算机制（防止重入攻击）<br>Pull-payment CEI protection | 权利要求2、4、6<br>Claims 2, 4, 6 |
| `logic_actor.rs` | P-05 | 系统输入意图对齐与动态滑动 EWMA 积分评级<br>Intent alignment check & EWMA calculation | 权利要求1、4、8、10<br>Claims 1, 4, 8, 10 |
| `gateway_proxy.rs` | P-05 | 边缘网关物理时延注入阶梯限流惩戒<br>Tiered network latency injection penalty | 权利要求1、4、5<br>Claims 1, 4, 5 |
| `sincerity_audit.rs` | P-05 | 链上连续审计验证与跨合约冻结安全联动<br>Auditor contract & cross-contract asset freeze | 权利要求1、3、6、7、9<br>Claims 1, 3, 6, 7, 9 |
| `gateway.rs` | P-06 | 游程状态转换计费阶梯网关<br>3-Tier payment gateway routing | 权利要求1、4、5<br>Claims 1, 4, 5 |
| `sandbox.rs` | P-06 | 飞地缺陷率改善测试与未达标退款豁免机制<br>Outcome check & escrow refund exemption | 权利要求1、4、8<br>Claims 1, 4, 8 |
| `routing_contract.rs` | P-06 | 同源版权 Royalty 划款与服务节点溢出路由<br>Royalty distribution & load split routing | 权利要求1、2、3、6、7、9<br>Claims 1, 2, 3, 6, 7, 9 |

---

## 3. 许可与设计规范 / License & Specifications

- **许可架构 / Licensing**: 本仓库代码资产严格采用 **QNLOO-SPL v1.1**（商用及特许权协议）与 **Apache 2.0**（源码宽松开源）的双层隔离许可架构。项目根目录已置备标准的“四文件布局”（`LICENSE`, `LICENSE-APACHE`, `LICENSE-QNLOO-SPL`, `PATENTS`），请合规参照。
- **Rust 源码规范**: 全仓采用 Rust 语言，严格遵循其社区标准命名与目录结构，便于全球开发者直接编译、审查及独立验证。

- **Licensing**: Code assets in this repository are strictly governed by the dual-layer isolation architecture of **QNLOO-SPL v1.1** (commercial & charter) and **Apache 2.0** (source code). Standard "Four-File Layout" (`LICENSE`, `LICENSE-APACHE`, `LICENSE-QNLOO-SPL`, `PATENTS`) is deployed in the root directory.
- **Rust Source Specification**: The entire repository adopts the Rust language, strictly following its community standard naming and directory structure, making it easy for global developers to compile and verify.

---

## 4. 官方生态入口 / Official Ecosystem Links

整个千年鹿认知体系由五大核心板块构成，覆盖规则范式、方法论与工具载体、理论体系、技术专利、工程实现五大层级，各板块底层逻辑完全同构。

The entire QNLOO cognition system consists of five core modules, covering five layers: rule paradigm, methodology & tool carrier, theoretical system, technical patents, and engineering implementation. All modules share the identical underlying logic.

| 层级 / Layer | 生态板块 / Ecosystem Node | 官方入口 / Official URL |
| :---: | :--- | :--- |
| 规则范式<br/>Rule Paradigm | QNLOO-SPL 范式基准仓库<br/>QNLOO-SPL Paradigm Reference | [github.com/QNLOO/00-QNLOO-SPL-Licenses](https://github.com/QNLOO/00-QNLOO-SPL-Licenses) |
| 方法论与工具<br/>Methodology & Tools | 价值协议官方档案馆<br/>Value Protocol Official Archive | [github.com/QNLOO/01-1-value-protocols](https://github.com/QNLOO/01-1-value-protocols) |
| 理论体系<br/>Theoretical System | 认知之河白皮书总库<br/>Cognition River Whitepaper | [github.com/QNLOO/01-2-cognition-river](https://github.com/QNLOO/01-2-cognition-river) |
| 技术专利<br/>Technical Patents | 技术专利与合规档案库<br/>Patents & Compliance Archive | [github.com/QNLOO/01-3-cognition-river-patents](https://github.com/QNLOO/01-3-cognition-river-patents) |
| 工程实现<br/>Engineering Implementation | 体系技术蓝图仓库<br/>System Technical Blueprints | [github.com/QNLOO/01-4-cognition-river-blueprints](https://github.com/QNLOO/01-4-cognition-river-blueprints) |

---

## 5. 官方音乐专辑 / Official Music Album

千年鹿认知体系专属声音资产，与白皮书叙事完全同步发布。它是用声音写成的生命史诗，记录了个人认知成长从温柔孕育到淬炼成金、从觉醒到开创的完整英雄旅程。每一首曲目，都对应着白皮书叙事中的一个关键节点。

Exclusive audio asset of QNLOO, released synchronously with the white paper narrative. It is an epic of life written with sound, recording the complete heroic journey of individual cognitive growth from gentle gestation to refinement into gold, and from awakening to forging new paths. Each track corresponds to a key node in the white paper's narrative.

| 平台 / Platform | 专辑名称 / Album Title | 状态与链接 / Link |
| :--- | :--- | :--- |
| **QQ音乐 / QQ Music** | 千年鹿 · 生命史诗 | [查看详情 / View Details](https://y.qq.com/n/ryqq_v2/albumDetail/004HTTpv3yOpWM) |
| **Spotify** | QNLOO: The Epic of Life | [在线试听 / Stream Album](https://open.spotify.com/album/4kZkrAVCECuUTG1uu5sOL1) |

---

## 6. 联系与对接 / Contact & Inquiries

| 规划维度 | 对接规范与格式要求 |
| :--- | :--- |
| **对接邮箱** | **contact@qnloo.com** |
| **邮件主题** | `[您自定义的简明主题]` |
| **邮件正文** | 1. 您的机构/姓名<br/>2. 意图说明<br/>3. 所属机构官网地址<br/>4. 联系方式 |
| **注意事项** | 本邮箱仅用于初步意向沟通与信息分发，不构成任何商业承诺或合同要约。 |

<br/>

| Category | Requirements & Format |
| :--- | :--- |
| **Contact Email** | **contact@qnloo.com** |
| **Subject** | `[Your customized concise subject]` |
| **Body** | 1. Your institution/name<br/>2. Statement of intent<br/>3. Official website of your institution<br/>4. Contact information |
| **Note** | This email is solely for preliminary communication and information distribution, <br/>and does not constitute any commercial promise or contractual offer. |

---

## 7. 官方社区与社交媒体 / Official Community & Social Media

| 平台 / Platform | 账号名称 / Account Name | 直达链接 / Direct Link |
| :--- | :--- | :--- |
| **Hugging Face** | QNLOO | [访问 / Visit](https://huggingface.co/QNLOO) |
| **GitHub** | QNLOO | [访问 / Visit](https://github.com/QNLOO) |
| **Product Hunt** | @qnloo | [访问 / Visit](https://www.producthunt.com/@qnloo) |
| **X (Twitter)** | @QNLOO2026 | [访问 / Visit](https://x.com/QNLOO2026) |
| **Reddit** | u/QNLOO | [访问 / Visit](https://www.reddit.com/user/QNLOO/) |
| **知乎 / Zhihu** | qnloo | [访问 / Visit](https://www.zhihu.com/people/qnloo) |

---

## 8. 外部权限 / Contribution Policy

**“发布即交付”——每一份交付成果都不是征求意见稿，而是构建者深思熟虑之后的完整思想交付。** 因此，本仓库为官方对外只读归档仓库，不接收任何外部提交的PR、Issue及修改贡献。

官方将持续进行贝叶斯更新——主动发现问题、验证假设、寻求更优路径，从而自主决定交付成果的迭代方向与节奏。外部反馈将作为辅助参考，但迭代的最终决策与执行主体始终是官方自身。所有内容的迭代、释义与版本定义权属归官方所有。

**"Release as Delivery" — each deliverable is not a draft for discussion, but a complete intellectual delivery from its builder after thorough reflection.** Therefore, this is an official public read-only archive. No external PRs, issues or modifications are accepted.

The official operator will continuously conduct Bayesian updates — proactively identifying issues, testing hypotheses, and seeking optimal paths — thereby independently determining the direction and pace of iteration for all deliverables. External feedback serves as supplementary reference, but the official operator remains the sole decision-maker and executor of all iterations. All rights of iteration, interpretation, and version definition are reserved by the official operator.

---

## 9. 克隆与下载说明 (Git LFS) / Clone & Download Instructions (Git LFS)

本仓库的 PDF 及多媒体交付物采用 **Git LFS (Large File Storage)** 进行版本管理。

*   **直接网页下载**：点击网页右上角的 **Code -> Download ZIP**，下载的压缩包中已包含**完整、可以直接阅读的 PDF 文件**。
*   **使用命令行克隆**：
    1.  请确保本地已安装 Git LFS（安装与下载指引请参考 [git-lfs.github.com](https://git-lfs.github.com/)）。
    2.  安装后在终端运行一次初始化命令：
        ```bash
        git lfs install
        ```
    3.  使用 `git clone` 克隆仓库。如果克隆下来的 PDF 显示为文本指针，可在仓库目录下运行以下命令拉取真实文件：
        ```bash
        git lfs pull
        ```

This repository uses **Git LFS (Large File Storage)** to manage PDF and multimedia deliverables.

*   **Web Download**: Click **Code -> Download ZIP** in the top-right corner. The downloaded ZIP archive contains the **fully readable PDF deliverables**.
*   **Command-Line Clone**:
    1.  Ensure Git LFS is installed on your local machine (refer to [git-lfs.github.com](https://git-lfs.github.com/)).
    2.  Run the initialization command once:
        ```bash
        git lfs install
        ```
    3.  Run `git clone` to copy the repository. If the cloned PDFs appear as pointer text files, pull the actual files by running:
        ```bash
        git lfs pull
        ```

---

**© 2026 千年鹿 QNLOO**

**厦门千年鹿文化科技有限公司 / Xiamen QNLOO Culture & Technology Co., Ltd.**
