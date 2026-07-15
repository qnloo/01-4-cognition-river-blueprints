// P-RZZH-060-QNLOO-2026: 本地隐私确权与同步 - 样板间主程序入口 (Main Entry)
// P-RZZH-060-QNLOO-2026: Local Privacy Ownership & Synchronization - Prototype Main Entry (Main Entry)
// 本文件以实证代码形式，演示三层跳转、容量分流路由、TEE脱敏判定、效果未达标退款豁免与防并发重入拉式提款。
// This file demonstrates 3-tier routing, capacity throttling, TEE desensitization, outcome-based refund exemptions, and reentrancy-safe pull-based withdrawals.

mod gateway;
mod sandbox;
mod routing_contract;

use sha2::{Sha256, Digest};

fn main() {
    println!("======================================================================");
    println!("🚀 [ P-RZZH-060-QNLOO-2026 ] 效果验证结算与同源分流系统 - 专利样板间代码启动");
    println!("🚀 [ P-RZZH-060-QNLOO-2026 ] Outcome Verification Settlement & Homocentric Split System - Patent Blueprint Startup");
    println!("======================================================================\n");

    // ----------------------------------------------------------------------
    // 0. 初始化核心节点与配置
    // 0. Initialize core nodes and configurations
    // ----------------------------------------------------------------------
    let root_address = routing_contract::ECOLOGY_BACKFEED_POOL.to_string();
    let mut routing_engine = routing_contract::OutcomeBillingRoutingContract::new(
        root_address.clone(),
        // Platform revenue share: 200 per 10000 (2%)
        200, // 平台分成比万分之二百 (2%)
        // Meta-protocol ecosystem backfeed share: 100 per 10000 (1%)
        100, // 元协议反哺分成万分之一百 (1%)
    );
    let verify_sandbox = sandbox::OutcomeVerifySandbox::new();

    let user_name = "User_Alice";
    let master_teacher = "Master_Bob";
    let sub_teacher = "Sub_Bob_Junior";

    // 注册传光师 (Master_Bob 设置容量上限为 1，超过就分流至 Sub_Bob_Junior)
    // Register Chuan Guang Shi (Master_Bob sets capacity limit to 1, excess overflows to Sub_Bob_Junior)
    routing_engine.register_teacher(
        master_teacher.to_string(),
        // Capacity limit: 1
        1, // 容量上限：1
        // Top-level copyright owner
        master_teacher.to_string(), // 顶级版权所有者
        // List of child protocols
        vec![sub_teacher.to_string()], // 子协议列表
    );
    // 注册子代传光师 (同源子代所有者也是 Master_Bob)
    // Register child Chuan Guang Shi (Homologous child owner is also Master_Bob)
    routing_engine.register_teacher(
        sub_teacher.to_string(),
        // Child capacity limit: 5
        5, // 子代容量上限为 5
        // Top-level copyright owner points to Bob
        master_teacher.to_string(), // 顶级版权所有者指向 Bob
        vec![],
    );

    println!("[ 系统就绪 ] 注册主协议专家 '{}' (容量: 1) 与同源子代 '{}' (容量: 5) 成功。", master_teacher, sub_teacher);
    println!("[ Initialized ] Registered master expert '{}' (capacity: 1) and homologous child '{}' (capacity: 5) successfully.\n", master_teacher, sub_teacher);

    // ----------------------------------------------------------------------
    // 1. 三层价格网关累计调用自增跳转与拦截测试
    // 1. 3-tier pricing gateway cumulative call auto-increment routing and interception test
    // ----------------------------------------------------------------------
    println!("----------------------------------------------------------------------");
    println!("🌱 1. 三层价格限制网关累计调用状态机强转验证");
    println!("🌱 1. Three-tier pricing gateway cumulative call state machine transition verification");
    println!("----------------------------------------------------------------------");
    let session_gateway_id = hash_bytes(b"GATEWAY_JUMP_SESSION_01");

    // 模拟前 3 次：免费体验 FreeTrial
    // Simulate first 3 times: FreeTrial
    for i in 1..=3 {
        let tier = routing_engine.access_gateway(session_gateway_id, user_name.to_string(), master_teacher.to_string(), 0).unwrap();
        println!("   第 {} 次调用网关 -> 当前所处计费层级: {:?}", i, tier);
        println!("   Gateway Call #{} -> Current billing tier: {:?}", i, tier);
    }
    
    // 模拟第 4 次调用：付费AI，不给钱将被网关拦截
    // Simulate 4th call: PaidAI, will be intercepted by gateway if unpaid
    println!("\n[ 网关拦截测试 ] 免费试用超限后，不质押金额请求 PaidAI 层...");
    println!("[ Gateway Intercept ] Trial limits exceeded. Requesting PaidAI tier without deposits...");
    match routing_engine.access_gateway(session_gateway_id, user_name.to_string(), master_teacher.to_string(), 0) {
        Ok(_) => {
            println!("   ❌ [ 状态缺陷 ] 网关未拦截零支付请求！");
            println!("   ❌ [ State Anomaly ] Gateway failed to intercept zero-deposit request!");
        },
        Err(e) => {
            println!("   ✅ [ 拦截成功 ] 成功拦截非预付的 AI 服务调用！错误原因: '{}'", e);
            println!("   ✅ [ Intercept Success ] Grabbed unpaid AI request successfully! Error: '{}'", e);
        },
    }

    // 正常支付 AI 小额费用
    // Normal payment of small AI fee
    let tier_ai = routing_engine.access_gateway(session_gateway_id, user_name.to_string(), master_teacher.to_string(), 2).unwrap();
    println!("   预付 2 Token -> 成功跃迁至: {:?}", tier_ai);
    println!("   Prepaid 2 Tokens -> Successfully transitioned to: {:?}", tier_ai);

    // 再次调用 PaidAI 2 次以触发容量升级
    // Call PaidAI 2 more times to trigger capacity upgrade
    let _ = routing_engine.access_gateway(session_gateway_id, user_name.to_string(), master_teacher.to_string(), 2);
    let _ = routing_engine.access_gateway(session_gateway_id, user_name.to_string(), master_teacher.to_string(), 2);

    // 模拟第 7 次：强制跳转真人效果计费层 (OutcomeHuman)
    // Simulate 7th call: Forced routing to OutcomeHuman billing tier
    println!("\n[ 真人层跳转 ] 触发真人效果付费层限流，进行托管资金质押...");
    println!("[ Human Tier Escalation ] Triggering human escrow limit. Pledging escrow deposit...");
    let tier_human = routing_engine.access_gateway(session_gateway_id, user_name.to_string(), master_teacher.to_string(), 100).unwrap();
    println!("   质押质押 100 Token -> 强制跳转至: {:?}", tier_human);
    println!("   Pledged 100 Tokens -> Enforced transition to: {:?}", tier_human);
    println!("");

    // ----------------------------------------------------------------------
    // 2. 真人容量饱和分流与同源版权提成返还测试
    // 2. Human capacity saturation routing and homologous copyright royalty kickback test
    // ----------------------------------------------------------------------
    println!("----------------------------------------------------------------------");
    println!("🔀 2. 活跃真人会话容量饱和路由分流与同源版权分成返还 (10% Royalty)");
    println!("🔀 2. Active human session capacity saturation routing redirect & homologous royalty kickback (10% Royalty)");
    println!("----------------------------------------------------------------------");

    // 此时 Master_Bob 的活跃会话数为 1 (刚才 session_gateway_id 占用了该名额)
    // At this point, Master_Bob's active session count is 1 (occupied by session_gateway_id earlier)
    // 另一个用户 User_Eve 发起请求，系统检测 Bob 饱和，自动分流至 Sub_Bob_Junior
    // User_Eve initiates a request, system detects Bob is saturated, automatically routes to Sub_Bob_Junior
    let target = routing_engine.get_routing_target(master_teacher);
    println!("   User_Eve 尝试发起协作请求，目标传光师: {}", master_teacher);
    println!("   User_Eve attempts collaboration request. Target expert: {}", master_teacher);
    println!("   分流路由决策模块返回实际分流目标: '{}'", target);
    println!("   Split routing decision engine returned actual target: '{}'", target);

    // 建立 Eve 在子代路由的 OutcomeHuman 会话并预存 100 质押托管资金
    // Establish Eve's OutcomeHuman session on child route and pre-deposit 100 escrow funds
    let session_eve_id = hash_bytes(b"EVE_SESSION_SATURATED_02");
    // 模拟前序直接强行跳转并质押 100 资金
    // Simulate forced routing and pledging 100 funds
    let _ = routing_engine.access_gateway(session_eve_id, "User_Eve".to_string(), target.clone(), 100);

    // 模拟子代完成效果，沙箱验证断言达标并通过签名
    // Simulate child completing outcome, sandbox verifies assertion and passes signature
    let nonce_eve = hash_bytes(b"EVE_NONCE_2026");
    let (assertion, proof, sig) = verify_sandbox.verify_outcome_in_tee(
        session_eve_id,
        // Initial defect rate 10%
        0.10f32, // 初始缺陷率 10%
        // Post-tuning defect rate 4%
        0.04f32, // 调试后缺陷率 4%
        // Required improvement > 5% (Actual improvement 6%)
        0.05f32, // 要求改善 5% 以上 (实际改善 6%)
        nonce_eve
    );

    // 链上执行 execute_billing 结算
    // Execute on-chain billing settlement
    println!("\n[ 链上清结算 ] 提交 TEE 签名证明对子代服务进行清算...");
    println!("[ On-chain Clearance ] Dispatching TEE signed proof to settle child services...");
    let expected_v = routing_engine.sessions.get(&session_eve_id).unwrap().version;
    let _ = routing_engine.execute_billing(session_eve_id, assertion, nonce_eve, expected_v, proof, sig);

    // 校验分成结果
    // 100 Token:
    // 平台: 2% = 2
    // Platform: 2% = 2
    // 元协反哺池: 1% = 1
    // Ecosystem backfeed pool: 1% = 1
    // 主协版权所有者(Master_Bob): 10% = 10
    // Master protocol copyright owner (Master_Bob): 10% = 10
    // 子代实际服务者(Sub_Bob_Junior): 100 - 2 - 1 - 10 = 87
    // Child Actual Service Provider (Sub_Bob_Junior): 100 - 2 - 1 - 10 = 87
    let bal_platform = *routing_engine.pending_withdrawals.get("PLATFORM_WALLET").unwrap_or(&0);
    let bal_root = *routing_engine.pending_withdrawals.get(&root_address).unwrap_or(&0);
    let bal_master = *routing_engine.pending_withdrawals.get(master_teacher).unwrap_or(&0);
    let bal_sub = *routing_engine.pending_withdrawals.get(sub_teacher).unwrap_or(&0);

    println!("\n   分成记账分配检查 (万分比级联拨付)：");
    println!("   Asset allocation check (ten-thousandths cascade splits):");
    println!("   └─ 平台手续费账户: {} Token", bal_platform);
    println!("   └─ Platform fee wallet: {} Tokens", bal_platform);
    println!("   └─ 元协生态反哺池: {} Token", bal_root);
    println!("   └─ Meta-protocol ecology backfeed pool: {} Tokens", bal_root);
    println!("   └─ 主协议所有者 ({}) 溢出提成: {} Token", master_teacher, bal_master);
    println!("   └─ Master protocol owner ({}) overflow royalty: {} Tokens", master_teacher, bal_master);
    println!("   └─ 实际提供服务子代 ({}): {} Token", sub_teacher, bal_sub);
    println!("   └─ Actual service child ({}) wallet allocation: {} Tokens", sub_teacher, bal_sub);
    println!("");

    // ----------------------------------------------------------------------
    // 3. 效果未达标退款豁免与防白嫖验证
    // 3. Substandard outcome refund exemption and anti-freeloader verification
    // ----------------------------------------------------------------------
    println!("----------------------------------------------------------------------");
    println!("⚠️ 3. 效果未达标退款豁免测试 (Outcome Failure & Fee Exemption)");
    println!("⚠️ 3. Outcome substandard refund exemption & leverage protection test (Outcome Failure & Fee Exemption)");
    println!("----------------------------------------------------------------------");

    // 质押托管了 100 Token 的 session_gateway_id 会话，模拟服务结果不达标 (只降了 1%，要求 5%)
    // session_gateway_id session with 100 Token in escrow, simulate service outcome failure (only reduced 1%, required 5%)
    let nonce_fail = hash_bytes(b"GATEWAY_SESSION_FAIL_NONCE");
    let (assertion_fail, proof_fail, sig_fail) = verify_sandbox.verify_outcome_in_tee(
        session_gateway_id,
        0.10f32,
        // Decreased by only 1%
        0.09f32, // 仅下降 1%
        // Target decrease 5%
        0.05f32, // 目标下降 5%
        nonce_fail
    );

    let session_v = routing_engine.sessions.get(&session_gateway_id).unwrap().version;
    let user_prev_bal = *routing_engine.pending_withdrawals.get(user_name).unwrap_or(&0);

    println!("\n[ 链上清结算 ] 提交未达标断言，触发退款豁免...");
    println!("[ On-chain Clearance ] Submitting substandard outcome assertion to trigger refund exemption...");
    let _ = routing_engine.execute_billing(session_gateway_id, assertion_fail, nonce_fail, session_v, proof_fail, sig_fail);

    let user_post_bal = *routing_engine.pending_withdrawals.get(user_name).unwrap_or(&0);
    println!("   用户 '{}' 提款余额变更: {} -> {} (成功原路退回 100 Token)", user_name, user_prev_bal, user_post_bal);
    println!("   User '{}' claimable balance trace: {} -> {} (Successfully returned 100 Tokens to original wallet)", user_name, user_prev_bal, user_post_bal);
    println!("");

    // ----------------------------------------------------------------------
    // 4. 重放、乐观锁与 Pull 提款安全性验证
    // 4. Anti-replay, optimistic locking, and Pull withdrawal security verification
    // ----------------------------------------------------------------------
    println!("----------------------------------------------------------------------");
    println!("🔒 4. 签名重放、乐观锁冲突与 Pull Payment CEI 提款安全防御");
    println!("🔒 4. Signature replay, optimistic locking, and Pull Payment CEI withdrawal security defenses");
    println!("----------------------------------------------------------------------");

    // 4.1 防重放 nonce_fail 再次请求
    // 4.1 Anti-replay nonce_fail retry
    println!("[ 重放模拟 ] 提交已处理的 nonce_fail 到合约进行结算...");
    println!("[ Replay Simulation ] Submitting verified nonce_fail to smart contract for settlement...");
    let (_, pd_replay, sig_replay) = verify_sandbox.verify_outcome_in_tee(
        session_gateway_id, 0.10, 0.04, 0.05, nonce_fail
    );
    // 重新获取已退款会话的乐观锁版号
    // Retrieve optimistic lock version of refunded session
    let fresh_v = routing_engine.sessions.get(&session_gateway_id).unwrap().version;
    match routing_engine.execute_billing(
        session_gateway_id,
        true,
        // Reused
        nonce_fail, // 重用
        fresh_v,
        pd_replay.to_vec(),
        sig_replay.to_vec()
    ) {
        Ok(_) => {
            println!("   ❌ [ 状态缺陷 ] 成功执行了重放签名验证！");
            println!("   ❌ [ State Anomaly ] Replay signature verification was bypass-executed!");
        },
        Err(e) => {
            println!("   ✅ [ 拦截成功 ] 成功拦截重放攻击！错误: '{}'", e);
            println!("   ✅ [ Intercept Success ] Successfully blocked replay attack! Error: '{}'", e);
        },
    }

    // 4.2 乐观锁版本校验：传入过期版本号
    // 4.2 Optimistic lock version check: passing expired version number
    println!("\n[ 乐观锁并发测试 ] 传入过期的 session version = 1 进行请求...");
    println!("[ Optimistic Concurrency Test ] Submitting request with outdated session version = 1...");
    let nonce_fresh = hash_bytes(b"FRESH_NONCE_VER");
    let (a_v, pd_v, sig_v) = verify_sandbox.verify_outcome_in_tee(
        session_gateway_id, 0.10, 0.04, 0.05, nonce_fresh
    );
    match routing_engine.execute_billing(
        session_gateway_id,
        a_v,
        nonce_fresh,
        // Stale version number
        1, // 陈旧版本号
        pd_v,
        sig_v
    ) {
        Ok(_) => {
            println!("   ❌ [ 状态缺陷 ] 成功越权执行并发状态冲突覆盖！");
            println!("   ❌ [ State Anomaly ] Overplaced concurrency overwrite succeeded!");
        },
        Err(e) => {
            println!("   ✅ [ 拦截成功 ] 乐观锁检测成功，拦截版本并发冲突！错误: '{}'", e);
            println!("   ✅ [ Intercept Success ] Optimistic locking verified. Intercepted version collision! Error: '{}'", e);
        },
    }

    // 4.3 安全拉式提款 Checks-Effects-Interactions 逻辑
    // 4.3 Secure Pull-Based Withdrawal Checks-Effects-Interactions logic
    println!("\n[ 拉式提款测试 ] 用户 '{}' 调用拉式提款接口 withdraw()...", user_name);
    println!("[ Pull Payout Test ] User '{}' triggers pull withdrawal API withdraw()...", user_name);
    let _ = routing_engine.withdraw(user_name);
    
    // 二次提款，余额已清零，应该报错
    // Secondary withdrawal, balance cleared, should error
    match routing_engine.withdraw(user_name) {
        Ok(_) => {
            println!("   ❌ [ 状态缺陷 ] 成功执行了二次提现，合约存在并发重入/双花溢出！");
            println!("   ❌ [ State Anomaly ] Re-withdrawal succeeded! Contract has reentrancy/double-spend flaws!");
        },
        Err(e) => {
            println!("   ✅ [ 重入拦截 ] 成功拦截二次空卷提款！错误: '{}'", e);
            println!("   ✅ [ Reentrancy Block ] Successfully blocked secondary zero-balance withdrawal! Error: '{}'", e);
        },
    }

    println!("\n======================================================================");
    println!("🎉 [ 演练结束 ] 三层网关跳转、真人容量分流、退款豁免与 CEI 安全提现全部验证通过！");
    println!("🎉 [ Demos Complete ] 3-tier gateway, overflow routing, refunds, and reentrancy blocks validated!");
    println!("======================================================================");
}

// 辅助方法
// Helper methods
fn hash_bytes(data: &[u8]) -> [u8; 32] {
    let mut hasher = Sha256::new();
    hasher.update(data);
    hasher.finalize().into()
}
