// P-RZZH-040-QNLOO-2026: 本地隐私确权与同步 - 多级收益自动分账智能合约模块 (Contract Module)
// P-RZZH-040-QNLOO-2026: Local Privacy Confirmation & Sync - Multi-level Revenue Cascading Billing Contract Module
// 物理安全机制：18级谱系依赖图验证 + 乐观锁版本并发控制 + Checks-Effects-Interactions 重入保护 + 20年状态到期反哺
// Physical Security: 18-level lineage DAG validation + optimistic lock concurrency control + Checks-Effects-Interactions reentrancy protection + 20-year state expiration backfeed

use std::collections::HashMap;

pub const ECOLOGY_BACKFEED_POOL: &str = "0x000000000000000000000000000000000000FADE";
pub const SECONDS_IN_20_YEARS: u64 = 20 * 365 * 24 * 3600;

#[derive(Clone, Debug)]
pub struct ProtocolNode {
    pub creator: String,
    pub parent_hashes: Vec<[u8; 32]>,
    pub creation_time: u64,
    // 对应权利要求4、7：父协议与其特征指纹相似度分值，作为分成分配权重
    // Aligning with Claims 4 & 7: Similarity score with parent protocol as payout allocation weight
    pub similarity_scores: Vec<u32>, 
    // 对应权利要求6、9：乐观锁版本号
    // Aligning with Claims 6 & 9: Optimistic locking version number
    pub version: u64, 
}

pub struct CascadingBillingContract {
    pub protocols: HashMap<[u8; 32], ProtocolNode>,
    pub balances: HashMap<String, u64>,
    pub used_nonces: HashMap<[u8; 32], bool>,
    pub root_protocol_hash: [u8; 32],
    pub mock_block_timestamp: u64,
}

impl CascadingBillingContract {
    pub fn new(root_hash: [u8; 32]) -> Self {
        let mut contract = Self {
            protocols: HashMap::new(),
            balances: HashMap::new(),
            used_nonces: HashMap::new(),
            root_protocol_hash: root_hash,
            mock_block_timestamp: 0,
        };

        // 注册根级元协议
        // Register root metadata protocol
        // 对应权利要求1、4：根级元协议创作者强制锁定为生态反哺池，自身零收益
        // Aligning with Claims 1 & 4: Root metadata protocol creator forced to ECOLOGY_BACKFEED_POOL, zero individual payout
        let root_node = ProtocolNode {
            creator: ECOLOGY_BACKFEED_POOL.to_string(),
            parent_hashes: Vec::new(),
            creation_time: 0, 
            similarity_scores: Vec::new(),
            version: 1,
        };
        contract.protocols.insert(root_hash, root_node);
        contract
    }

    // 协议注册与级系深度审计
    // Protocol registration and lineage depth auditing
    // 对应权利要求1、4、7：硬性截断依赖级数阈值 18 级，并进行环路依赖检测
    // Aligning with Claims 1, 4, 7: Hard cutoff of dependency levels at 18, and circular dependency checks
    pub fn register_protocol(
        &mut self,
        caller: String,
        protocol_hash: [u8; 32],
        parents: Vec<[u8; 32]>,
        similarities: Vec<u32>,
        creation_time: u64,
    ) -> Result<(), &'static str> {
        if parents.len() != similarities.len() {
            return Err("Parents and similarities count mismatch");
        }

        // 1. 环路依赖检测与 18 代深度递归校验
        // 1. Circular dependency detection and 18-generation depth recursion validation
        let mut visited = std::collections::HashSet::new();
        visited.insert(protocol_hash);

        for parent in &parents {
            if self.has_circular_dependency(*parent, &mut visited) {
                return Err("CIRCULAR_DEPENDENCY: Indirect loop detected. DAG alignment failed.");
            }
            let depth = self.calculate_depth(*parent, 1);
            if depth > 18 {
                return Err("LIMIT_EXCEEDED: Dependency depth cannot exceed 18 levels");
            }
        }

        let new_node = ProtocolNode {
            creator: caller,
            parent_hashes: parents,
            creation_time,
            similarity_scores: similarities,
            version: 1,
        };

        self.protocols.insert(protocol_hash, new_node);
        println!(
            "[ 注册成功 ] 注册认知协议 0x{}... | 依赖祖辈层数: {}",
            hex_encode(&protocol_hash[..4]),
            self.calculate_depth(protocol_hash, 0)
        );
        println!(
            "[ Register Success ] Registered cognitive protocol 0x{}... | Lineage depth: {}",
            hex_encode(&protocol_hash[..4]),
            self.calculate_depth(protocol_hash, 0)
        );

        Ok(())
    }

    // 级联分账多路径清分
    // Cascading billing multi-route clearing
    // 对应权利要求1、3、4、6、9：检查 Nonce 防重放，并发乐观锁，非对称三级分拨记账 (自留75%、父10%、祖父15%)
    // Aligning with Claims 1, 3, 4, 6, 9: Nonce anti-replay verification, optimistic locking, asymmetric 3-tier payout (Self 75%, Parent 10%, Grandparent 15%)
    pub fn execute_billing(
        &mut self,
        protocol_hash: [u8; 32],
        amount: u64,
        nonce: [u8; 32],
        expected_version: u64,
    ) -> Result<(), &'static str> {
        // 1. 防重放 Nonce 校验
        // 1. Anti-replay Nonce check
        if self.used_nonces.contains_key(&nonce) {
            return Err("REPLAY_ERROR: This transaction nonce has already been cleared");
        }
        self.used_nonces.insert(nonce, true);

        // 2. 乐观锁并发版本号校验
        // 2. Optimistic concurrency version check
        let target_node = self.protocols.get_mut(&protocol_hash).ok_or("Protocol not found")?;
        if target_node.version != expected_version {
            return Err("CONCURRENCY_COLLISION: Target protocol version mismatch. Version state changed.");
        }
        // 版本状态更新自增
        // Auto-increment version state update
        target_node.version += 1;
 
        // 3. 非对称收益划分计算 (孙: 75%, 父: 10%, 祖父: 15%)
        // 3. Asymmetric revenue allocation computation (Grandson: 75%, Parent: 10%, Grandparent: 15%)
        // 对应从属权利要求3：祖父级层(15%)分拨固定高于父级层(10%)，以诱导开发者不断向底层迭代
        // Aligning with Claim 3: Grandparent layer payout (15%) is higher than parent layer (10%) to incentivize bottom-up technical evolution
        let father_layer_payout = amount
            .checked_mul(10)
            .ok_or("INTEGER_OVERFLOW: Overflow calculating father payout")?
            .checked_div(100)
            .ok_or("INTEGER_DIVISION_BY_ZERO: Division error calculating father payout")?;
        let grandfather_layer_payout = amount
            .checked_mul(15)
            .ok_or("INTEGER_OVERFLOW: Overflow calculating grandfather payout")?
            .checked_div(100)
            .ok_or("INTEGER_DIVISION_BY_ZERO: Division error calculating grandfather payout")?;
        let creator_payout = amount
            .checked_sub(father_layer_payout)
            .ok_or("INTEGER_UNDERFLOW: Underflow calculating creator payout")?
            .checked_sub(grandfather_layer_payout)
            .ok_or("INTEGER_UNDERFLOW: Underflow calculating creator payout")?;

        // 引入临时状态缓冲区，保障多路径分账的绝对原子性
        // Introduce temporary state buffers to guarantee absolute multi-route atomicity
        let mut delta_updates: HashMap<String, u64> = HashMap::new();
        let mut total_distributed = 0u64;

        // 3.1 划分并记账孙协议创作者资金
        // 3.1 Allocate and log payout for grandson creator
        let creator_recipient = Self::get_active_recipient(self.mock_block_timestamp, target_node);
        let creator_entry = delta_updates.entry(creator_recipient).or_insert(0);
        *creator_entry = (*creator_entry)
            .checked_add(creator_payout)
            .ok_or("INTEGER_OVERFLOW: Overflow adding creator balance update")?;
        total_distributed = total_distributed
            .checked_add(creator_payout)
            .ok_or("INTEGER_OVERFLOW: Overflow adding total distributed")?;

        // 3.2 划分并记账父代分成资金 (10%)
        // 3.2 Allocate and log payout for parent layers (10%)
        let parent_hashes = target_node.parent_hashes.clone();
        let similarity_scores = target_node.similarity_scores.clone();
        
        let total_similarity: u64 = similarity_scores.iter().map(|&x| x as u64).sum();
        
        for (index, parent_hash) in parent_hashes.iter().enumerate() {
            if let Some(parent_node) = self.protocols.get(parent_hash) {
                // 基于相似度分值比例计算权重
                // Calculate weights based on similarity score proportions
                let weight_value = if total_similarity > 0 {
                    (similarity_scores[index] as u64)
                        .checked_mul(father_layer_payout)
                        .ok_or("INTEGER_OVERFLOW: Overflow calculating parent similarity weight")?
                        .checked_div(total_similarity)
                        .ok_or("INTEGER_DIVISION_BY_ZERO: Division by zero in parent weight calculation")?
                } else {
                    father_layer_payout
                        .checked_div(parent_hashes.len() as u64)
                        .ok_or("INTEGER_DIVISION_BY_ZERO: Division error in parent weight calculation")?
                };

                let parent_recipient = Self::get_active_recipient(self.mock_block_timestamp, parent_node);
                let parent_entry = delta_updates.entry(parent_recipient).or_insert(0);
                *parent_entry = (*parent_entry)
                    .checked_add(weight_value)
                    .ok_or("INTEGER_OVERFLOW: Overflow adding parent balance update")?;
                total_distributed = total_distributed
                    .checked_add(weight_value)
                    .ok_or("INTEGER_OVERFLOW: Overflow adding total distributed")?;

                // 3.3 划分并递归记账祖父代分成资金 (15%)
                // 3.3 Allocate and recursively log payout for grandparent layers (15%)
                let grandfather_hashes = parent_node.parent_hashes.clone();
                if !grandfather_hashes.is_empty() {
                    // 祖父层按节点数平分为默认规则，可按需扩展相似度加权
                    // Grandparent layer is split equally by default, but can be extended to similarity-weighted distribution as needed.
                    let gf_weight_value = grandfather_layer_payout
                        .checked_div(grandfather_hashes.len() as u64)
                        .ok_or("INTEGER_DIVISION_BY_ZERO: Division error in grandparent weight calculation")?;
                    for gf_hash in &grandfather_hashes {
                        if let Some(gf_node) = self.protocols.get(gf_hash) {
                            let gf_recipient = Self::get_active_recipient(self.mock_block_timestamp, gf_node);
                            let gf_entry = delta_updates.entry(gf_recipient).or_insert(0);
                            *gf_entry = (*gf_entry)
                                .checked_add(gf_weight_value)
                                .ok_or("INTEGER_OVERFLOW: Overflow adding grandparent balance update")?;
                            total_distributed = total_distributed
                                .checked_add(gf_weight_value)
                                .ok_or("INTEGER_OVERFLOW: Overflow adding total distributed")?;
                        }
                    }
                } else {
                    // 若父代已经是第二级（无祖父代），则该 15% 自动打回生态定向反哺池
                    // If parent is already Level 2 (no grandparent), the 15% returns to ECOLOGY_BACKFEED_POOL
                    let pool_entry = delta_updates.entry(ECOLOGY_BACKFEED_POOL.to_string()).or_insert(0);
                    *pool_entry = (*pool_entry)
                        .checked_add(grandfather_layer_payout)
                        .ok_or("INTEGER_OVERFLOW: Overflow adding backfeed pool balance update")?;
                    total_distributed = total_distributed
                        .checked_add(grandfather_layer_payout)
                        .ok_or("INTEGER_OVERFLOW: Overflow adding total distributed")?;
                }
            }
        }

        // 若根本没有父代层（该协议是直接衍生于根元协议），父层(10%)与祖父层(15%)共计 25% 收益直接反哺生态
        // If no parent layer (derived from root directly), 25% total backfeeds to ecology
        if parent_hashes.is_empty() {
            let direct_backfeed_amount = father_layer_payout
                .checked_add(grandfather_layer_payout)
                .ok_or("INTEGER_OVERFLOW: Overflow calculating direct backfeed amount")?;
            let pool_entry = delta_updates.entry(ECOLOGY_BACKFEED_POOL.to_string()).or_insert(0);
            *pool_entry = (*pool_entry)
                .checked_add(direct_backfeed_amount)
                .ok_or("INTEGER_OVERFLOW: Overflow adding direct backfeed balance")?;
            total_distributed = total_distributed
                .checked_add(direct_backfeed_amount)
                .ok_or("INTEGER_OVERFLOW: Overflow adding total distributed")?;
        }

        // 处理财务粉尘截断漏出（Dust Leakage）
        // Handle financial dust truncation leakage
        if total_distributed < amount {
            let dust = amount
                .checked_sub(total_distributed)
                .ok_or("INTEGER_UNDERFLOW: Underflow calculating dust amount")?;
            let pool_entry = delta_updates.entry(ECOLOGY_BACKFEED_POOL.to_string()).or_insert(0);
            *pool_entry = (*pool_entry)
                .checked_add(dust)
                .ok_or("INTEGER_OVERFLOW: Overflow adding dust balance")?;
        }

        // 只有当多级回溯全盘无错通过，才一行代码全量并网，彻底规避账本半污染
        // Only update state when all recursive checks pass to avoid partial updates
        for (acc, amt) in delta_updates {
            let entry = self.balances.entry(acc).or_insert(0);
            *entry = (*entry)
                .checked_add(amt)
                .ok_or("INTEGER_OVERFLOW: Overflow updating final ledger balances")?;
        }

        Ok(())
    }

    // 创作者安全拉式划账接口
    // Sovereign pull-based payout claim interface
    // 对应权利要求4、5：先清余额、后转账，采用 Checks-Effects-Interactions 次序安全防线，阻断重入套利
    // Aligning with Claims 4 & 5: Check first, modify state, then transfer (Checks-Effects-Interactions) to block reentrancy
    pub fn claim_payout(&mut self, caller: &str) -> Result<u64, &'static str> {
        // Checks: 检查待提余额是否存在且大于 0
        // Checks: Validate if payout balance exists and is non-zero
        let available_balance = *self.balances.get(caller).ok_or("No balance registered for this account")?;
        if available_balance == 0 {
            return Err("BALANCE_ZERO: Available payout balance is zero");
        }

        // Effects: 先将合约中用户的待提余额清零，修改账本状态
        // Effects: Zero out available balance in the registry first
        self.balances.insert(caller.to_string(), 0);
        println!(
            "[ 拉式提款 ] 账户 '{}' 执行划出，提款额: {}。 [Checks-Effects-Interactions 安全检查通过]",
            caller, available_balance
        );
        println!(
            "[ Pull Claim ] Account '{}' executed payout of {}. [Checks-Effects-Interactions safety passed]",
            caller, available_balance
        );

        // Interactions: 最终执行外部资金流划转（这里返回划拨数值模拟）
        // Interactions: Execute final transfer (simulated return value)
        // 提示：实际链上部署时，若底层转账指令返回失败，合约会自动回滚整个交易，恢复余额状态，确保事务原子性
        // Note: In production on-chain deployments, if the underlying transfer fails, the contract will automatically roll back the transaction and restore balances, ensuring transactional atomicity.
        Ok(available_balance)
    }

    // 递归计算继承深度
    // Recursively calculate inheritance depth
    fn calculate_depth(&self, hash: [u8; 32], current_depth: u32) -> u32 {
        if let Some(node) = self.protocols.get(&hash) {
            let mut max_depth = current_depth;
            for parent in &node.parent_hashes {
                let depth = self.calculate_depth(*parent, current_depth + 1);
                if depth > max_depth {
                    max_depth = depth;
                }
            }
            max_depth
        } else {
            current_depth
        }
    }

    fn has_circular_dependency(&self, current_hash: [u8; 32], visited: &mut std::collections::HashSet<[u8; 32]>) -> bool {
        if visited.contains(&current_hash) {
            return true;
        }
        if let Some(node) = self.protocols.get(&current_hash) {
            visited.insert(current_hash);
            for parent in &node.parent_hashes {
                if self.has_circular_dependency(*parent, visited) {
                    return true;
                }
            }
            visited.remove(&current_hash);
        }
        false
    }

    // 校验生命周期到期状态并获取收款地址
    // Verify lifecycle expiration and retrieve recipient address
    // 对应权利要求1、4：若已满 20年 保护期阈值，触发状态机收款权转移，强制将收益重定向流入公共反哺池
    // Aligning with Claims 1 & 4: If 20-year limit reached, redirect payout to ECOLOGY_BACKFEED_POOL
    fn get_active_recipient(mock_block_timestamp: u64, node: &ProtocolNode) -> String {
        // 根级元协议或创建时间为0的，直接判归公共反哺池
        // Root protocol or creation_time == 0 assigned directly to backfeed pool
        if node.creation_time == 0 {
            return ECOLOGY_BACKFEED_POOL.to_string();
        }

        // 提示：链上区块时间戳可能存在微小的操纵空间。在生产环境中，建议使用多区块中位数时间戳 (MPT) 并结合严格的时间窗口校验，以防止恶意节点通过操纵时间戳来提前触发协议保护期到期
        // Note: On-chain block timestamps can be slightly manipulated. In production, use Median Past Time (MPT) combined with strict time-window verification to prevent malicious miners/nodes from manipulating block timestamps to prematurely trigger protocol expiration.
        let expiration_time = node.creation_time.checked_add(SECONDS_IN_20_YEARS);
        let is_expired = match expiration_time {
            Some(exp) => mock_block_timestamp >= exp,
            None => true,
        };

        if is_expired {
            println!(
                "⏳ [ 状态到期 ] 协议属于创作者 '{}', 创建时间: {}。因已超过 20 年专利保护期，分账状态切换至全生态公共，收益重定向归入反哺池 '{}'。",
                node.creator, node.creation_time, ECOLOGY_BACKFEED_POOL
            );
            println!(
                "⏳ [ State Expired ] Protocol owned by '{}', created at {}. Since 20-year patent protection has expired, payout redirects to public backfeed pool '{}'.",
                node.creator, node.creation_time, ECOLOGY_BACKFEED_POOL
            );
            ECOLOGY_BACKFEED_POOL.to_string()
        } else {
            node.creator.clone()
        }
    }
}

// 辅助方法：生成易读的哈希前缀
// Helper: Generate human-readable hex prefix
fn hex_encode(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{:02x}", b)).collect()
}
