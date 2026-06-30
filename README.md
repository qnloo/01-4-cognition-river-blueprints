# 千年鹿认知体系 · 体系技术蓝图仓库｜QNLOO Cognition System · System Technical Blueprints

### 1. 本仓库定位 / Repository Profile

- **性质**：全球官方唯一《认知之河》六项核心发明专利的参考实现与开源样板间仓库。作为体系技术蓝图的官方交付载体，本仓库用于公开专利方案的标准化工程实现，供全球开发者学习、验证及合规参照。
- **Nature**: The world's official reference implementation and open-source blueprint repository for the six core invention patents of the *River of Cognition*. As the official delivery vehicle for the system's technical blueprints, this repository publicly provides standardized engineering implementations of the patented solutions for global developers to study, verify, and use as compliance reference.

- **资产形态 / Asset Format**

    - **源码**：以 Rust 语言编写的高可读性、高安全性参考实现，完整展示专利说明书所披露技术方案的工程落地形态。
    - **Source Code**: Highly readable and secure reference implementations written in Rust, fully demonstrating the engineering implementation forms of the technical solutions disclosed in the patent specifications.

- **交付节奏**：遵循“发布即交付”的核心理念，本仓库已完成全部六项专利样板间的代码研发与最终审计。**源码文件将于2026年7月15日正式上传**。当前仓库为预告状态，已公开完整目录结构、说明文档及许可证文件，供全球开发者提前了解样板间架构与代码模块映射关系。
- **Delivery Cadence**: Adhering to the core principle of "Release as Delivery," all six patent blueprint codes have been fully developed and finalized in audit. **The source code files will be officially uploaded on July 15, 2026.** The repository is currently in a preview state, with the complete directory structure, documentation, and license files publicly accessible, allowing global developers to familiarize themselves with the blueprint architecture and code-to-patent mapping in advance.

- **外部权限**：本仓库为官方只读参考实现归档仓库，不接受任何外部PR、Issue及代码贡献。样板间代码仅供学习、验证与合规参照，官方不对外部修改、分发或衍生使用承担任何技术支持和维护义务。本仓库仅对其中的官方源码本体做迭代维护，不管理第三方Fork或衍生版本。
- **Contribution Policy**: This is an official read-only reference implementation archive. No external PRs, issues, or code contributions are accepted. The blueprint codes are provided solely for study, verification, and compliance reference. The official operator assumes no technical support or maintenance obligations for external modifications, distributions, or derivative uses. This repository only iterates and maintains its own official source code and does not manage third-party forks or derivative versions.

- **法律基座**：本仓库全部源码及文档，遵循以 **Apache License 2.0（源码）** 与 **QNLOO-SPL v1.0（专利/文本/品牌）** 为核心的**双资产分层许可架构**。完整范式标准详见官方基准仓库。
- **Legal Base**: All source code and documentation in this repository are governed by a **Dual-Asset Tiered Licensing Architecture** centered on the **Apache License 2.0 (for source code)** and **QNLOO-SPL v1.0 (for patents/text/branding)**. Complete paradigm specifications can be found in the official reference repository.

- 本仓库采用双资产分层许可架构：核心资产适用根目录 LICENSE 对应许可；源代码类资产适用 LICENSE-APACHE；商事运营与价值分配规则适用 LICENSE-QNLOO-SPL。专利授权补充说明详见 PATENTS 文件。

- This repository adopts a dual-asset tiered licensing architecture: core assets are governed by the license corresponding to the root LICENSE file; source code assets are governed by LICENSE-APACHE; commercial operations and value distribution rules are governed by LICENSE-QNLOO-SPL. For patent grant addendum, see the PATENTS file.

---

### 2. 资产总览 / Asset Overview

**说明**：本仓库为《认知之河》全套六项核心发明专利的官方参考实现，每项专利对应一个独立的 Rust 样板间工程。**源码文件将于2026年7月15日正式上传**，当前仓库已公开完整目录结构与代码模块映射关系。

**Description**: This repository is the official reference implementation for all six core invention patents of the *River of Cognition*, with each patent corresponding to an independent Rust blueprint project. **Source code files will be officially uploaded on July 15, 2026.** The complete directory structure and code-to-patent mapping are already publicly accessible.


#### 2.1 六项专利样板间清单 / Six Patent Blueprints

| 样板间目录 / Blueprint Directory | 对应专利 / Patent | 核心功能 / Core Function | 源码 / Source |
| :--- | :---: | :--- | :---: |
| `P-01-Three-Layer-Separation-Architecture/` | P-01 | 三层物理分离、TEE消磁、防重放<br>Three-layer separation, TEE erasure, anti-replay | 2026-07-15 |
| `P-02-Standardized-Cognitive-Protocol-Packaging/` | P-02 | 级联哈希、自适应转译、隐蔽水印<br>Cascading hash, adaptive translation, covert watermark | 2026-07-15 |
| `P-03-Local-Privacy-Right-Confirmation-and-Synchronization/` | P-03 | 本地指纹确权、Loopback隔离、增量同步<br>Local fingerprinting, loopback isolation, incremental sync | 2026-07-15 |
| `P-04-Cascading-Revenue-Auto-Settlement/` | P-04 | 18代分账、DFS环检测、拉式提款<br>18-gen settlement, DFS cycle detection, pull payment | 2026-07-15 |
| `P-05-Logic-Continuity-Sincerity-Audit/` | P-05 | 四维因果审计、EWMA积分、梯度惩戒<br>4D causal audit, EWMA score, tiered penalty | 2026-07-15 |
| `P-06-Outcome-Based-Pricing-and-Settlement-System/` | P-06 | 三层网关、效果验证、容量分流<br>3-tier gateway, outcome verification, capacity routing | 2026-07-15 |

> 每个样板间目录均为独立的 Rust 工程，源码上传后将包含完整的 `Cargo.toml` 及可直接编译运行的全部源码文件。全局 `Cargo.toml` 位于仓库根目录。
> 
> Each blueprint directory is an independent Rust project. Once source code is uploaded, it will contain a complete `Cargo.toml` and all source files ready for compilation and execution. The global `Cargo.toml` is located in the repository root.


#### 2.2 代码模块与专利权利要求映射 / Code-to-Patent Mapping

**说明**：下表列出六项专利对应的官方参考实现代码模块及其核心功能定位，供全球开发者在源码上传后按图索骥，精准验证每项专利权利要求的工程实现。

**Description**: The table below maps the official reference implementation code modules to their respective patents and core functions, enabling global developers to precisely verify the engineering implementation of each patent claim once the source code is uploaded.

| 核心代码模块 / Core Code Module | 所属专利 / Patent | 功能定位 / Function | 对应核心权利要求 / Core Claims |
| :--- | :---: | :--- | :--- |
| `gateway.rs` | P-06 | 三层价格梯度网关<br>Three-Tier Pricing Gradient Gateway | 权利要求1、4、5<br>Claims 1, 4, 5 |
| `sandbox.rs` | P-06 | 本地效果验证TEE沙箱<br>Local Outcome Verification TEE Sandbox | 权利要求1、4、8<br>Claims 1, 4, 8 |
| `routing_contract.rs` | P-06 | 路由清分智能合约<br>Routing & Clearing Smart Contract | 权利要求1、2、3、6、7、9<br>Claims 1, 2, 3, 6, 7, 9 |
| `logic_actor.rs` | P-05 | 逻辑行为体审计沙箱<br>Logic Actor Audit Sandbox | 权利要求1、4、8、10<br>Claims 1, 4, 8, 10 |
| `gateway_proxy.rs` | P-05 | 边缘网关惩戒模块<br>Edge Gateway Penalty Module | 权利要求1、4、5<br>Claims 1, 4, 5 |
| `sincerity_audit.rs` | P-05 | 链上真诚审计合约<br>On-Chain Sincerity Audit Contract | 权利要求1、3、6、7、9<br>Claims 1, 3, 6, 7, 9 |
| `billing_contract.rs` | P-04 | 多级分账智能合约<br>Multi-Level Cascading Billing Smart Contract | 权利要求1、2、3、5<br>Claims 1, 2, 3, 5 |
| `withdrawal_module.rs` | P-04 | 拉式提款安全模块<br>Pull-Based Withdrawal Security Module | 权利要求2、4、6<br>Claims 2, 4, 6 |

---

### 3. 文件设计规范 / File Design Specification

- **Rust 源码**：全仓采用 Rust 语言，严格遵循其社区标准命名与目录结构（如 `Cargo.toml`），便于全球开发者直接编译、审查及独立验证。
- **Rust Source**: The entire repository adopts the Rust language, strictly following its community standard naming and directory structure (e.g., `Cargo.toml`), making it easy for global developers to directly compile, review, and independently verify.

---

### 4. 分级授权使用规范 / Tiered License Terms (QNLOO-SPL v1.0 Compliant)

- **✅ 个人 / 学术 / 非营利场景（永久免费）**
  个人学习、学术研究、非盈利分享交流场景，可永久免费运行、修改、分发本仓库所有源码，无需并网与授权。
- **✅ Personal / Academic / Non-Profit (Permanent Free)**
  All source code in this repository is permanently free to run, modify, and distribute for personal study, academic research, and non-profit communication, with no integration or license application required.

- **✅ 小微商业普惠使用权益**
  年度关联业务营收不超过100万元人民币、完整适配价值协议生态并通过官方一致性校验的小微主体，可免费商用本体系专利技术方案。
- **✅ Small Business Friendly Authorization**
  Small-scale entities with annual relevant revenue ≤ 1M RMB that fully adapt to the Value Protocol ecosystem and pass official consistency verification are eligible for free commercial use of the patented technical solutions.

- **⚠️ 商用合规说明**
  本仓库采用双资产分层许可架构：计算机源代码遵循 Apache License 2.0；专利技术方案、体系逻辑及商业运营行为受 QNLOO-SPL v1.0 约束，并受 PATENTS 文件补充约束。
  所有商业应用场景，若实施本体系专利技术方案开展运营服务，均需遵循 QNLOO-SPL v1.0 范式规范合规使用。官方保留对任何实质性违规行为进行溯源、取证，并依据范式规则发起全球分布式司法维权的权利。
- **⚠️ Commercial Compliance Statement**
  This repository adopts a dual-asset tiered license architecture: computer source code is governed by Apache License 2.0; patented technical solutions, system logic, and commercial operations are governed by QNLOO-SPL v1.0, with additional constraints from the PATENTS file.
  All commercial applications implementing the patented technical solutions of this system for operational services shall comply with the QNLOO-SPL v1.0 paradigm specifications. The official reserves the right to trace, collect evidence, and initiate global distributed judicial enforcement against any substantive violation.

- **🚩 文本释义优先级说明**
  本仓库中英双语文档若存在翻译歧义或解读冲突，**以中文版本为唯一最终释义标准**。
- **🚩 Interpretation Preference Rule**
  In case of any ambiguity or interpretation conflict in bilingual documents, **the Chinese version shall prevail as the sole authoritative standard.**

---

### 5. 官方生态入口 / Official Ecosystem Links

| 生态板块 / Ecosystem Node | 官方入口 / Official URL |
| :--- | :--- |
| QNLOO-SPL 范式基准仓库 / QNLOO-SPL Paradigm Reference | [github.com/QNLOO/00-QNLOO-SPL-Licenses](https://github.com/QNLOO/00-QNLOO-SPL-Licenses) |
| 价值协议官方档案馆 / Value Protocol Official Archive | [github.com/QNLOO/01-1-value-protocols](https://github.com/QNLOO/01-1-value-protocols) |
| 认知之河白皮书总库 / Cognition River Whitepaper | [github.com/QNLOO/01-2-cognition-river](https://github.com/QNLOO/01-2-cognition-river) |
| 技术专利与合规档案库 / Patents & Compliance Archive | [github.com/QNLOO/01-3-cognition-river-patents](https://github.com/QNLOO/01-3-cognition-river-patents) |

---

### 6. 仓库权限与版权声明 / Repository Policy & Copyright

**仓库规范**：本仓库为官方技术蓝图基准仓库，源码仅供学习、验证与合规参照。不接收任何外部代码贡献、修改提交或功能请求。体系所有技术迭代、版本更新权属归官方独家所有。

**Repository Policy**: This is the official technical blueprint reference repository. Source code is provided for study, verification, and compliance reference only. No external code contributions, modification submissions, or feature requests are accepted. All rights of technical iteration and version update are exclusively reserved by the official operator.

**版权声明**：© 2026 千年鹿 QNLOO，遵循 Apache License 2.0（源码）与 QNLOO-SPL v1.0（专利/文本/品牌）双许可规范发布，保留全部官方权属。

**Copyright Notice**: © 2026 QNLOO. Released under the dual-license of Apache License 2.0 (source code) and QNLOO-SPL v1.0 (patents/text/branding). All official rights reserved.
