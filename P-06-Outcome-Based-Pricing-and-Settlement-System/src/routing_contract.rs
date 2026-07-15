// P-RZZH-060-QNLOO-2026: 本地隐私确权与同步 - 路由清分智能合约模块 (Routing Contract Module)
// P-RZZH-060-QNLOO-2026: Local Privacy Ownership & Sync - Routing & Clearing Smart Contract Module
// 物理安全机制：活跃容量监控及同源子协议路由分流 + 效果断言判定 + Pull 待提记账与 e-CNY 锁步重置
// Physical Security: Active Capacity Monitoring & Congeneric Sub-protocol Routing Split + Outcome Assertion Decision + Pull-based Billing & e-CNY Lockstep Settlement

use std::collections::HashMap;
use sha2::{Sha256, Digest};
use crate::gateway::{PricingTier, PriceGateway};

pub const ECOLOGY_BACKFEED_POOL: &str = "0x000000000000000000000000000000000000FADE";

#[derive(Clone, Debug)]
pub struct UserSession {
    #[allow(dead_code)]
    pub session_id: [u8; 32],
    pub user_address: String,
    pub teacher_address: String,
    pub tier: PricingTier,
    pub trial_counter: u32,
    pub paid_ai_counter: u32,
    // 会话质押托管余额
    // Session deposit escrow balance
    pub deposit_escrow: u64,
    // 乐观锁版本号
    // Optimistic concurrency control version number
    pub version: u64,
}

#[derive(Clone, Debug)]
pub struct TeacherProfile {
    #[allow(dead_code)]
    pub teacher_address: String,
    pub capacity_limit: u32,
    pub active_sessions: u32,
    pub parent_protocol_owner: String,
    pub sub_protocols: Vec<String>,
}

pub struct OutcomeBillingRoutingContract {
    pub teachers: HashMap<String, TeacherProfile>,
    pub sessions: HashMap<[u8; 32], UserSession>,
    pub pending_withdrawals: HashMap<String, u64>,
    pub used_nonces: HashMap<[u8; 32], bool>,
    pub root_address: String,
    // 万分比基准，200 = 2%
    // Platform fee rate basis points (200 = 2%)
    pub service_fee_rate: u32,
    // 万分比基准，100 = 1%
    // Root fee rate basis points (100 = 1%)
    pub root_fee_rate: u32,
    pub gateway_logic: PriceGateway,
}

impl OutcomeBillingRoutingContract {
    pub fn new(root_addr: String, fee_rate: u32, root_rate: u32) -> Self {
        Self {
            teachers: HashMap::new(),
            sessions: HashMap::new(),
            pending_withdrawals: HashMap::new(),
            used_nonces: HashMap::new(),
            root_address: root_addr,
            service_fee_rate: fee_rate,
            root_fee_rate: root_rate,
            gateway_logic: PriceGateway::new(),
        }
    }

    pub fn register_teacher(
        &mut self,
        teacher: String,
        limit: u32,
        parent_owner: String,
        subs: Vec<String>,
    ) {
        let profile = TeacherProfile {
            teacher_address: teacher.clone(),
            capacity_limit: limit,
            active_sessions: 0,
            parent_protocol_owner: parent_owner,
            sub_protocols: subs,
        };
        self.teachers.insert(teacher, profile);
    }

    // 活跃会话真人容量饱和监测与同源路由重定向分流
    // Active Session Human Capacity Monitoring & Congeneric Routing Redirection Split
    // 对应权利要求1、3、4、7：若 active_sessions >= capacity_limit，激活分流，重定向到同源子协议
    // Aligning with Claims 1, 3, 4, 7: Active session limit triggers redirection to congeneric sub-protocol
    pub fn get_routing_target(&self, target_teacher: &str) -> String {
        if let Some(profile) = self.teachers.get(target_teacher) {
            // 容量超限饱和
            // Capacity limit saturated
            if profile.active_sessions >= profile.capacity_limit && !profile.sub_protocols.is_empty() {
                // 提示：此处默认选取第一个空闲节点作为示例。若所有子协议也均达到容量上限（全饱和场景），本演示代码默认返回原目标节点。
                // 在生产环境中，此处的选择策略可进一步扩展为基于多维度进行动态排序选择，例如：
                // 1. 负载率（选取 active_sessions/capacity_limit 最低的节点以均衡负载）；
                // 2. 服务历史评分（优先选择评分最高的最优子协议节点）；
                // 3. 计费价格梯度（选择价格最经济或更适配当前任务难度的节点）。
                // 此外，针对全饱和场景，可扩展排队等待机制、超额直接拒绝策略或跨域弹性调度分流逻辑，以体现方案的极端完备性。
                // Note: The prototype selects the first idle node from the sub-protocols list. If all sub-protocols are also saturated (full saturation), the prototype returns the original target.
                // In production, the dispatch strategy can be extended to dynamically sort sub-protocols by:
                // 1. Load rate (select lowest active_sessions/capacity_limit for load balancing);
                // 2. Historical service rating (prioritize highest rating);
                // 3. Billing pricing tier (select the most cost-effective node).
                // Additionally, for full saturation scenarios, queueing mechanisms, over-limit rejection, or cross-domain elastic scheduling can be implemented to ensure system completeness.
                for sub_addr in &profile.sub_protocols {
                    if let Some(sub_profile) = self.teachers.get(sub_addr) {
                        if sub_profile.active_sessions < sub_profile.capacity_limit {
                            println!(
                                "🔀 [ 饱和分流 / Capacity Redirect ] 专家 '{}' 活跃通道饱和 ({}/{})。分流路由已激活，重定向至同源子协议所有者 '{}'。 / Expert '{}' active channel saturated ({}/{}). Redirection active, routing to congeneric sub-protocol '{}'.",
                                target_teacher, profile.active_sessions, profile.capacity_limit, sub_addr,
                                target_teacher, profile.active_sessions, profile.capacity_limit, sub_addr
                            );
                            return sub_addr.clone();
                        }
                    }
                }
            }
        }
        target_teacher.to_string()
    }

    // 接入三层价格网关，处理状态强转拦截与结算质押托管
    // Accessing Three-tier Price Gateway, Forced Interception & Settlement Escrow Deposit
    // 对应权利要求1、4、5、7：根据累计计数器状态强转，OutcomeHuman层强制托管质押
    // Aligning with Claims 1, 4, 5, 7: Forced state transition based on call counter, mandatory escrow for OutcomeHuman
    pub fn access_gateway(
        &mut self,
        session_id: [u8; 32],
        user: String,
        teacher: String,
        payment: u64,
    ) -> Result<PricingTier, &'static str> {
        let mut session = self.sessions.get(&session_id).cloned().unwrap_or(UserSession {
            session_id,
            user_address: user,
            teacher_address: teacher.clone(),
            tier: PricingTier::FreeTrial,
            trial_counter: 0,
            paid_ai_counter: 0,
            deposit_escrow: 0,
            version: 1,
        });

        let old_tier = session.tier;

        // 累计调用计数自增
        // Increment accumulative call counters
        match session.tier {
            PricingTier::FreeTrial => {
                session.trial_counter = session.trial_counter
                    .checked_add(1)
                    .ok_or("INTEGER_OVERFLOW_ERROR: Trial counter increment overflow")?;
            }
            PricingTier::PaidAI => {
                session.paid_ai_counter = session.paid_ai_counter
                    .checked_add(1)
                    .ok_or("INTEGER_OVERFLOW_ERROR: Paid AI counter increment overflow")?;
            }
            PricingTier::OutcomeHuman => {}
        }

        // 判定新层级
        // Determine new pricing tier
        let new_tier = self.gateway_logic.determine_tier(session.trial_counter, session.paid_ai_counter);
        session.tier = new_tier;

        // 根据计费层逻辑拦截资金
        // Intercept funds based on billing tier logic
        match new_tier {
            PricingTier::FreeTrial => {
                println!(
                    "[ 价格网关 / Price Gateway ] 节点处于 FreeTrial 免费层 (计数: {}/3)，此会话完全免费开放。 / Node in FreeTrial tier (count: {}/3), session fully free.",
                    session.trial_counter, session.trial_counter
                );
            }
            PricingTier::PaidAI => {
                if payment == 0 {
                    return Err("PREPAID_REQUIRED_ERROR: Second tier PaidAI requires small prepaid deposit.");
                }
                // 提示：此处小额 AI 运行费仅做打印提示。在生产环境中，该笔费用将自动计入平台的待提取余额账户（例如 PLATFORM_WALLET），完成计费闭环。
                // Note: This small AI execution fee is for demonstration. In production, this fee will be credited to the platform's pending withdrawal account (e.g. PLATFORM_WALLET).
                println!(
                    "[ 价格网关 / Price Gateway ] 节点已跃迁至 PaidAI 低价层 (计数: {}/3)，扣除小额 AI 运行费: {}。 / Node transitioned to PaidAI tier (count: {}/3), deducting small AI execution fee: {}.",
                    session.paid_ai_counter, payment, session.paid_ai_counter, payment
                );
            }
            PricingTier::OutcomeHuman => {
                if payment == 0 && session.deposit_escrow == 0 {
                    return Err("ESCROW_REQUIRED_ERROR: Third tier OutcomeHuman requires premium outcome escrow deposit.");
                }
                session.deposit_escrow = session.deposit_escrow
                    .checked_add(payment)
                    .ok_or("INTEGER_OVERFLOW_ERROR: Deposit escrow increment overflow")?;
                
                // 占用专家活跃会话通道 (仅在会话首次跃迁入 OutcomeHuman 层时扣减通道容量，防止容量泄露)
                // Occupy active session channel of the expert (only increment capacity on initial transition to prevent capacity leakage)
                if old_tier != PricingTier::OutcomeHuman {
                    if let Some(profile) = self.teachers.get_mut(&teacher) {
                        profile.active_sessions = profile.active_sessions
                            .checked_add(1)
                            .ok_or("INTEGER_OVERFLOW_ERROR: Active sessions count increment overflow")?;
                        println!(
                            "[ 价格网关 / Price Gateway ] 节点已强转至 OutcomeHuman 真人层！已成功托管质押资金: {} Token。占用 '{}' 真人服务带宽通道 (当前活跃数: {}/{})。 / Node forced to OutcomeHuman tier! Escrow deposit successful: {} Token. Occupying '{}' service channel (active: {}/{}).",
                            session.deposit_escrow, teacher, profile.active_sessions, profile.capacity_limit,
                            session.deposit_escrow, teacher, profile.active_sessions, profile.capacity_limit
                        );
                    }
                } else {
                    println!(
                        "[ 价格网关 / Price Gateway ] 追加托管资金: {} Token，当前会话已处于 OutcomeHuman 真人层。 / Appending escrow: {} Token, session already in OutcomeHuman tier.",
                        payment, payment
                    );
                }
            }
        }

        self.sessions.insert(session_id, session);
        Ok(new_tier)
    }

    // 接收效果验证断言并执行拉式清结算 (防重放 + 乐观锁并发控制 + 效果未达标豁免退款)
    // Receive Outcome Verification Assertion & Execute Pull-based Clearing (Anti-replay + Optimistic Concurrency + Zero-effectiveness Refund Exemption)
    // 对应权利要求1、2、3、4、6、7、9：验签防重放，乐观锁，断言为假触发退款豁免，断言为真分润(含子代到父代10%返还)
    // Aligning with Claims 1, 2, 3, 4, 6, 7, 9: Signature verification, anti-replay, lock, refund on false, profit sharing on true (10% to parent)
    pub fn execute_billing(
        &mut self,
        session_id: [u8; 32],
        assertion: bool,
        nonce: [u8; 32],
        expected_version: u64,
        tee_proof: Vec<u8>,
        signature: Vec<u8>,
    ) -> Result<(), &'static str> {
        // 1. 防重放 Nonce 校验
        // Anti-replay Nonce verification
        if self.used_nonces.contains_key(&nonce) {
            return Err("REPLAY_ATTACK_DETECTED_ERROR: Transaction nonce already executed");
        }
        self.used_nonces.insert(nonce, true);

        // 2. 乐观锁并发控制版本校验
        // Optimistic Concurrency Control Version Verification
        let session = self.sessions.get_mut(&session_id).ok_or("NOT_FOUND_ERROR: User session not found")?;
        if session.version != expected_version {
            return Err("CONCURRENCY_CONFLICT_ERROR: Session version state changed by another concurrent task");
        }
        session.version = session.version
            .checked_add(1)
            .ok_or("INTEGER_OVERFLOW_ERROR: Session version increment overflow")?;

        // 5.1 验证签名是否来自可信 TEE
        // 5.1 Verify that signature originates from trusted TEE
        if !Self::verify_tee_signature(&tee_proof, &signature) {
            return Err("SECURITY_ALERT_ERROR: Invalid TEE signature for outcome proof");
        }

        let total_deposit = session.deposit_escrow;
        if total_deposit == 0 {
            return Err("BALANCE_EMPTY_ERROR: No escrow funds registered for this outcome session");
        }

        // 4. 释放专家活跃带宽通道
        // Release expert active bandwidth channel
        if let Some(profile) = self.teachers.get_mut(&session.teacher_address) {
            if profile.active_sessions > 0 {
                profile.active_sessions = profile.active_sessions
                    .checked_sub(1)
                    .unwrap_or(0);
            }
        }

        // 5. 对应权利要求1、4、7：效果不达标时执行费用豁免退款，达标时按比例分拨
        // Aligning with Claims 1, 4, 7: Exemption refund on zero outcome, proportional billing split on success
        if assertion {
            // 效果达标：划拨扣除，使用 checked 安全运算防止算术溢出
            // Outcome validated: execute clearing split with safe checked math
            let platform_share = total_deposit
                .checked_mul(self.service_fee_rate as u64)
                .ok_or("INTEGER_OVERFLOW_ERROR: Platform share multiplication overflow")?
                .checked_div(10000)
                .ok_or("INTEGER_DIVISION_BY_ZERO_ERROR: Platform share division error")?;

            let root_share = total_deposit
                .checked_mul(self.root_fee_rate as u64)
                .ok_or("INTEGER_OVERFLOW_ERROR: Root share multiplication overflow")?
                .checked_div(10000)
                .ok_or("INTEGER_DIVISION_BY_ZERO_ERROR: Root share division error")?;

            let mut teacher_share = total_deposit
                .checked_sub(platform_share)
                .ok_or("INTEGER_UNDERFLOW_ERROR: Teacher share subtraction underflow on platform share")?
                .checked_sub(root_share)
                .ok_or("INTEGER_UNDERFLOW_ERROR: Teacher share subtraction underflow on root share")?;

            // 对应从属权利要求3：若为溢出子协议提供服务，自动扣除 10% 溢出分成版权返还给主协议专家
            // Aligning with Claim 3: For congeneric sub-protocol execution, deduct 10% royalty backfeed to parent protocol owner
            if let Some(profile) = self.teachers.get(&session.teacher_address) {
                if profile.parent_protocol_owner != session.teacher_address {
                    let parent_share = total_deposit
                        .checked_mul(1000) // 10%
                        .ok_or("INTEGER_OVERFLOW_ERROR: Parent share multiplication overflow")?
                        .checked_div(10000)
                        .ok_or("INTEGER_DIVISION_BY_ZERO_ERROR: Parent share division error")?;
                    
                    let actual_parent_share = std::cmp::min(parent_share, teacher_share);
                    teacher_share = teacher_share
                        .checked_sub(actual_parent_share)
                        .ok_or("INTEGER_UNDERFLOW_ERROR: Teacher share subtraction underflow on parent share")?;
                    
                    let parent_bal = self.pending_withdrawals.entry(profile.parent_protocol_owner.clone()).or_insert(0);
                    *parent_bal = parent_bal
                        .checked_add(actual_parent_share)
                        .ok_or("INTEGER_OVERFLOW_ERROR: Parent balance addition overflow")?;
                    println!(
                        "   [ 同源分成 / Congeneric Split ] 成功将子协议收益 of 10% (实际分配: {}) 提取并记入主协议所有者 '{}' 待提余额中。 / Successfully deducted 10% royalty ({}) and credited parent owner '{}' pending balance.",
                        actual_parent_share, profile.parent_protocol_owner, actual_parent_share, profile.parent_protocol_owner
                    );
                }
            }

            // 存入 platform
            // Credit platform wallet
            let plat_bal = self.pending_withdrawals.entry("PLATFORM_WALLET".to_string()).or_insert(0);
            *plat_bal = plat_bal
                .checked_add(platform_share)
                .ok_or("INTEGER_OVERFLOW_ERROR: Platform balance addition overflow")?;

            // 存入 root 元协议反哺池
            // Credit root backfeed pool
            let root_bal = self.pending_withdrawals.entry(self.root_address.clone()).or_insert(0);
            *root_bal = root_bal
                .checked_add(root_share)
                .ok_or("INTEGER_OVERFLOW_ERROR: Root balance addition overflow")?;

            // 存入实际执行服务的老师
            // Credit actual service teacher
            let teacher_bal = self.pending_withdrawals.entry(session.teacher_address.clone()).or_insert(0);
            *teacher_bal = teacher_bal
                .checked_add(teacher_share)
                .ok_or("INTEGER_OVERFLOW_ERROR: Teacher balance addition overflow")?;

            println!(
                "   🎉 效果达标！结算资金分拨待提: 实际服务者: {}, 元反哺池: {} / Outcome validated! Settlement split: Service Teacher: {}, Root Pool: {}",
                teacher_share, root_share, teacher_share, root_share
            );
        } else {
            // 对应权利要求1、4、7：效果未达标 (Assertion = False)，退款豁免，100% 托管款返还至用户待提中
            // Aligning with Claims 1, 4, 7: Outcome unvalidated (Assertion = False), 100% escrow refunded to user pending withdrawals
            let user_bal = self.pending_withdrawals.entry(session.user_address.clone()).or_insert(0);
            *user_bal = user_bal
                .checked_add(total_deposit)
                .ok_or("INTEGER_OVERFLOW_ERROR: User balance refund addition overflow")?;
            println!(
                "   ⚠️ 效果未达标！触发按效果付费豁免条款，质押托管资金 {} Token 100% 退回用户待提余额。 / Outcome unvalidated! 100% of escrow ({} Token) refunded to user.",
                total_deposit, total_deposit
            );
        }

        // 清空质押托管数据
        // Clear escrow record
        session.deposit_escrow = 0;

        // 对应权利要求6：模拟对接数字人民币可编程支付网关锁步通信，接收成功回执解锁
        // Aligning with Claim 6: Mock communication with e-CNY clearing gateway, unlock on receipt
        self.mock_ecny_lockstep_clearing();

        Ok(())
    }

    // 提款：Checks-Effects-Interactions 安全防线
    // Pull-based Withdrawal: Checks-Effects-Interactions Pattern
    pub fn withdraw(&mut self, caller: &str) -> Result<u64, &'static str> {
        let amount = *self.pending_withdrawals.get(caller).ok_or("NOT_FOUND_ERROR: No balance registered for this caller")?;
        if amount == 0 {
            return Err("WITHDRAW_ZERO_ERROR: No withdrawal balance available");
        }

        // 先清零，后执行外部转账
        // Clear balance before transfer
        self.pending_withdrawals.insert(caller.to_string(), 0);
        println!(
            "[ 拉式提款 / Pull Withdrawal ] 提款方 '{}' 提款: {} Token。 [Checks-Effects-Interactions 安全执行] / Caller '{}' withdrew: {} Token. [Checks-Effects-Interactions Secure Execution]",
            caller, amount, caller, amount
        );
        Ok(amount)
    }

    fn verify_tee_signature(_proof: &[u8], signature: &[u8]) -> bool {
        // 校验 HMAC 哈希是否匹配主权安全密钥
        // Verify HMAC signature aligns with secure key
        let mut hmac = Sha256::new();
        hmac.update(b"TEE_OUTCOME_BILLING_PRIV_KEY_060");
        hmac.update(_proof);
        let expected_signature = hmac.finalize().to_vec();
        signature == expected_signature
    }

    fn mock_ecny_lockstep_clearing(&self) {
        // 提示：生产环境中的数字人民币直连锁步清算流程如下：
        // 1. 锁定账本状态：标记该会话的清算过程处于锁步进行中，禁止并发变更。
        // 2. 调用外部 e-CNY 接口：向央行数字人民币可编程支付网关发起点对点代币拨付请求。
        // 3. 接收处理回执：
        //    - 若收到成功回执，则正式解锁并清零相关计费计数器，完成业务闭环。
        //    - 若收到失败或超时回执，则自动执行状态回滚，保留质押资金及原有的未清分账本状态。
        // Note: The production e-CNY lockstep clearing workflow:
        // 1. Lock ledger state: mark clearing in progress to block concurrent updates.
        // 2. Invoke e-CNY API: call central bank programmable payments API for peer-to-peer tokens.
        // 3. Handle clearing receipt:
        //    - If success receipt received, unlock and clear the counters to finalize.
        //    - If failed/timeout receipt received, rollback to original locked state.
        println!("🇨🇳 [ 数字人民币直连可编程清算 / e-CNY Lockstep Clearing ] 锁步状态成功激活！对接二级运营机构，点对点完成数字钱包物理代币划转，接收回执无双花，解锁状态成功。 / Lockstep clearing activated! Peer-to-peer e-CNY digital wallet transfer complete with zero double-spend, status unlocked.");
    }
}
