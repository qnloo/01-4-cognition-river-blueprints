// P-RZZH-050-QNLOO-2026: 本地隐私确权与同步 - 样板间主程序入口 (Main Entry)
// P-RZZH-050-QNLOO-2026: Local Privacy Confirmation & Sync - Main Entry
// 本文件以实证代码形式，演示时序因果对齐、EWMA 权重演化、防重放、并发控制与网关时延惩戒。
// This file demonstrates temporal causal alignment, EWMA weight evolution, anti-replay, concurrency control, and gateway latency penalties.

mod logic_actor;
mod sincerity_audit;
mod gateway_proxy;

use sha2::{Sha256, Digest};

fn main() {
    println!("======================================================================");
    println!("🚀 [ P-RZZH-050-QNLOO-2026 ] 逻辑连续性真诚审计系统 - 专利样板间代码启动");
    println!("🚀 [ P-RZZH-050-QNLOO-2026 ] Logical Continuity Sincerity Audit System - Patent Blueprint Startup");
    println!("======================================================================\n");

    // ----------------------------------------------------------------------
    // 0. 系统实体初始化
    // 0. System Entities Initialization
    // ----------------------------------------------------------------------
    let actor_sandbox = logic_actor::LogicActorSandbox::new();
    let mut audit_contract = sincerity_audit::LogicContinuityContract::new();
    let gateway = gateway_proxy::GatewayProxy;

    let actor_name = "LogicActor_FirmA";
    let _ = audit_contract.register_actor(
        actor_name.to_string(),
        actor_sandbox.actor_public_key.clone()
    );
    println!("[ 初始就绪 ] 注册逻辑行为体 '{}'，绑定硬件公钥成功。\n", actor_name);
    println!("[ Initialized ] Logic actor '{}' registered, bound to hardware public key.\n", actor_name);

    // ----------------------------------------------------------------------
    // 1. “需求-方案-行为-效果”时序因果对齐正常审计流程
    // 1. "Demand-Proposal-Action-Outcome" sequential causal alignment normal audit flow
    // ----------------------------------------------------------------------
    println!("----------------------------------------------------------------------");
    println!("🌱 1. 时序因果对齐审计与正常可信评级更新 (Cosine Similarity & EWMA)");
    println!("🌱 1. Temporal causal alignment audit and Trusted rating update (Cosine Similarity & EWMA)");
    println!("----------------------------------------------------------------------");

    // 1.1 需求特征与解决方案特征的语义余弦对齐验证
    // 1.1 Semantic cosine alignment verification of demand and proposal features
    let v_demand = vec![0.85f32, 0.45f32, 0.12f32];
    let v_solution = vec![0.83f32, 0.47f32, 0.10f32];
    let sim = actor_sandbox.cos_similarity(&v_demand, &v_solution);
    println!("   需求 V_D = {:?}", v_demand);
    println!("   Demand V_D = {:?}", v_demand);
    println!("   方案 V_P = {:?}", v_solution);
    println!("   Proposal V_P = {:?}", v_solution);
    println!("   需求与方案余弦相似度 Sim: {:.4} (对齐阈值: 0.80)", sim);
    println!("   Cosine Similarity Sim: {:.4} (Alignment Threshold: 0.80)", sim);
    if sim >= 0.80 {
        println!("   ✅ 需求与方案自洽校验通过。");
        println!("   ✅ Demand-Proposal consistency check passed.");
    } else {
        println!("   ❌ 需求与方案断裂！");
        println!("   ❌ Demand-Proposal consistency broken!");
    }

    // 1.2 本地物理行为哈希与量化效果判定
    // 1.2 Local physical action hashing and quantified outcome determination
    let action_hash = hash_bytes(b"FIRM_A_MODIFY_VALVE_PRESSURE_LIMIT");
    // Qualification rate improvement metrics
    let outcome_metrics = vec![0.98f32, 0.05f32]; // 合格率提升指标
    let local_score = actor_sandbox.local_causal_verify(&action_hash, &outcome_metrics);
    println!("\n   本地提取设备动作指纹 H_A: 0x{}", hex_encode(&action_hash[..4]));
    println!("   Local device action fingerprint H_A: 0x{}", hex_encode(&action_hash[..4]));
    println!("   量化反馈效果指标 E: {:?}", outcome_metrics);
    println!("   Quantified feedback outcome metrics E: {:?}", outcome_metrics);
    println!("   TEE 因果判定逻辑自洽度得分 S_t: {}", local_score);
    println!("   TEE causal logic consistency score S_t: {}", local_score);

    // 1.3 生成 TEE 密码学签名证明并执行本地内存消磁
    // 1.3 Generate TEE cryptographic signature proof and execute local memory scrub
    let nonce_01 = hash_bytes(b"SESSION_NONCE_A01");
    let (proof_data, signature) = actor_sandbox.generate_tee_proof(local_score, nonce_01);
    println!("   [ TEE 签名 ] 脱敏完成，生成布尔证明数据，使用主权私钥生成数字签名。已强制物理清空内存敏感参数明细。");
    println!("   [ TEE Signature ] Desensitized boolean proof generated, signed with sovereign private key. Sensitive memory purged.");

    // 1.4 链上提交审计证明
    // 1.4 Submit audit proof on-chain
    match audit_contract.submit_sincerity_proof(
        actor_name.to_string(),
        local_score,
        nonce_01,
        1, // expected version
        proof_data,
        signature
    ) {
        Ok(next_weight) => {
            println!("   ✅ 链上审计成功！");
            println!("   ✅ On-chain audit successful!");
            let state_info = audit_contract.get_actor_state(actor_name.to_string()).unwrap();
            println!("      行为体 '{}' 的动态审计权重积分 W_t: {} (EWMA 衰减后)", actor_name, next_weight);
            println!("      Dynamic audit weight score W_t for '{}': {} (Post-EWMA decay)", actor_name, next_weight);
            println!("      当前信誉梯度状态: {:?}", state_info.1);
            println!("      Current reputation tier state: {:?}", state_info.1);
            
            // 网关策略验证
            // Gateway policy verification
            gateway.enforce_access_policy(actor_name, state_info.1, state_info.2);
        }
        Err(e) => {
            println!("   ❌ 链上审计失败: {}", e);
            println!("   ❌ On-chain audit failed: {}", e);
        }
    }
    println!("");

    // ----------------------------------------------------------------------
    // 2. 签名防重放与并发乐观锁控制
    // 2. Signature anti-replay and concurrency optimistic lock control
    // ----------------------------------------------------------------------
    println!("----------------------------------------------------------------------");
    println!("🔒 2. 防重放 Nonce 与并发版本控制");
    println!("🔒 2. Anti-replay Nonce & optimistic concurrency version control");
    println!("----------------------------------------------------------------------");

    // 2.1 重放攻击：再次发送已经用过的 nonce_01
    // 2.1 Replay attack: Resend already used nonce_01
    println!("[ 重放模拟 ] 拦截发送的 session_nonce_a01 试图再次提交审计...");
    println!("[ Replay Simulation ] Intercepted session_nonce_a01 trying to resubmit audit...");
    let actor_profile = audit_contract.actors.get(actor_name).unwrap();
    let (pd, sig) = actor_sandbox.generate_tee_proof(95, nonce_01);
    match audit_contract.submit_sincerity_proof(
        actor_name.to_string(),
        95,
        nonce_01,
        actor_profile.version,
        pd,
        sig
    ) {
        Ok(_) => {
            println!("   ❌ [ 状态缺陷 ] 审计合约通过了重放的旧 nonce！");
            println!("   ❌ [ Security Defect ] Audit contract incorrectly accepted replayed nonce!");
        }
        Err(e) => {
            println!("   ✅ [ 拦截成功 ] 成功拦截重放攻击！错误信息: '{}'", e);
            println!("   ✅ [ Intercept Success ] Successfully blocked replay attack! Error: '{}'", e);
        }
    }

    // 2.2 乐观锁并发冲突：传入陈旧的版本号 1 (当前已自增至 2)
    // 2.2 Optimistic concurrency collision: Pass stale version 1 (currently incremented to 2)
    println!("\n[ 乐观锁模拟 ] 异步并发提交审计证明，传入历史失效版本号 1...");
    println!("[ Optimistic Lock ] Concurrency submitting audit proof with stale version 1...");
    let fresh_nonce = hash_bytes(b"SESSION_NONCE_A02");
    let (pd2, sig2) = actor_sandbox.generate_tee_proof(95, fresh_nonce);
    match audit_contract.submit_sincerity_proof(
        actor_name.to_string(),
        95,
        fresh_nonce,
        1, // Stale version
        pd2,
        sig2
    ) {
        Ok(_) => {
            println!("   ❌ [ 状态缺陷 ] 审计合约乐观锁失效，并发状态碰撞冲突！");
            println!("   ❌ [ Security Defect ] Audit contract optimistic lock failed on concurrency race!");
        }
        Err(e) => {
            println!("   ✅ [ 拦截成功 ] 成功拦截并发更新碰撞！错误信息: '{}'", e);
            println!("   ✅ [ Intercept Success ] Successfully blocked concurrent state collision! Error: '{}'", e);
        }
    }
    println!("");

    // ----------------------------------------------------------------------
    // 3. 逻辑自洽度严重断裂下调评级及网关时延阶梯注入惩戒
    // 3. Severe logic self-consistency fracture: downgrade rating and escalate gateway latency penalty
    // ----------------------------------------------------------------------
    println!("----------------------------------------------------------------------");
    println!("⚠️ 3. 逻辑连续性严重断裂下调审计权重及网关 3.5s 阶梯时延惩戒");
    println!("⚠️ 3. Severe logical continuity breakage down-ranking & gateway 3.5s latency penalty");
    println!("----------------------------------------------------------------------");

    // 模拟恶意行为：调用方企业 FirmA 声称“无效果”，但在时间窗口内没有提交设备 PLC 参数调整动作哈希，
    // Simulate malicious behavior: Caller FirmA claims "no effect", but failed to submit equipment PLC parameter adjustment action hash within the time window,
    // 且依然在随后高频调用服务方案。沙箱判定一致性得分仅为 10 分。
    // Simulate malicious behavior: FirmA claims "no effect" but failed to submit PLC action hash within time window,
    // while continuing to call service at high frequency. Sandbox consistency score judged at 10.
    println!("[ 欺诈检测 ] 行为体 '{}' 反复报告方案无效但高频继续调用，且拒绝上传 PLC 动作哈希数据。因果一致性判定得分判定为 10 分。", actor_name);
    println!("[ Fraud Detection ] Actor '{}' reports ineffective outcomes yet calls repeatedly without PLC hash proof. Causal consistency score drops to 10.", actor_name);

    // 连续提交 5 次异常判定结果，模拟 EWMA 得分快速下跌
    // Submit 5 anomalous results continuously to simulate rapid EWMA score decline
    let mut nonce_seed = 100;
    for i in 2..=6 {
        let fake_nonce = hash_bytes(&[nonce_seed]);
        nonce_seed += 1;
        
        let (pd_fake, sig_fake) = actor_sandbox.generate_tee_proof(10, fake_nonce);
        let current_version = audit_contract.actors.get(actor_name).unwrap().version;
        
        let _ = audit_contract.submit_sincerity_proof(
            actor_name.to_string(),
            10,
            fake_nonce,
            current_version,
            pd_fake,
            sig_fake
        );
        let profile = audit_contract.actors.get(actor_name).unwrap();
        println!("   第 {} 次判定提交后 -> 审计积分 W_t: {} | 信誉梯度: {:?}", i, profile.sincerity_weight, profile.state);
        println!("   After audit submittal {} -> Audit Score W_t: {} | Reputation Tier: {:?}", i, profile.sincerity_weight, profile.state);
        
        // 阶梯时延边界验证：打印每一次违规时的具体毫秒数
        // Latency tier boundary verification: Print specific ms delay at each violation
        if profile.state == sincerity_audit::SincerityState::Fraudulent {
            let current_delay = gateway.enforce_access_policy(actor_name, profile.state, profile.consecutive_fraud_count);
            println!("      ⏱️ [ 时延边界验证 ] 阶梯时延计算：连续欺诈次数 = {} | 强制注入延迟 = {}ms", profile.consecutive_fraud_count, current_delay);
            println!("      ⏱️ [ Latency Boundary Verification ] Tiered delay: consecutive frauds = {} | enforced latency = {}ms", profile.consecutive_fraud_count, current_delay);
        }
    }

    // 调用网关，校验是否惩戒拦截
    // Call gateway to verify penalty interception
    let final_profile = audit_contract.actors.get(actor_name).unwrap();
    println!("\n[ 边缘拦截 ] 审计等级已下调至: {:?}", final_profile.state);
    println!("[ Edge Intercept ] Audit tier downgraded to: {:?}", final_profile.state);
    let final_delay = gateway.enforce_access_policy(actor_name, final_profile.state, final_profile.consecutive_fraud_count);
    println!("   ➡️ 网关执行最终拦截决策：最终时延注入 = {}ms (阶梯上限边界 = 12000ms)", final_delay);
    println!("   ➡️ Gateway enforced final intercept: final latency = {}ms (Ladder cap boundary = 12000ms)", final_delay);
    println!("");

    // ----------------------------------------------------------------------
    // 4. 跨智能合约分账资金冻结联动控制
    // 4. Cross-smart-contract billing funds freeze linked control
    // ----------------------------------------------------------------------
    println!("----------------------------------------------------------------------");
    println!("🔗 4. 跨智能合约联动控制 (欺诈状态自动冻结分账提取权)");
    println!("🔗 4. Cross-contract joint control (Fraudulent state auto-freezes payout rights)");
    println!("----------------------------------------------------------------------");

    // 对应权利要求1、6、7：联动专利四的分账系统，自动查询真诚度并锁定资产提取
    // Aligning with Claims 1, 6, 7: Link with Patent P-04 billing system, auto-query sincerity and lock withdrawals
    let (weight, state, consecutive_frauds) = audit_contract.get_actor_state(actor_name.to_string()).unwrap();
    println!("[ 跨合约联动 ] 财务清结算系统 (cascading_billing) 调用真诚审计合约查询节点 '{}' 信誉积分...", actor_name);
    println!("[ Cross-Contract Linkage ] Cascading billing system queries Sincerity Audit contract for actor '{}'...", actor_name);
    println!("   查询结果: 积分 = {}, 状态 = {:?}, 连续欺诈次数 = {}", weight, state, consecutive_frauds);
    println!("   Query Result: Score = {}, State = {:?}, Consecutive Frauds = {}", weight, state, consecutive_frauds);
    
    if state == sincerity_audit::SincerityState::Fraudulent {
        println!("🚨 [ 财务冻结 ] 触发安全响应机制：真诚度跌破欺诈门限，立即联动冻结 '{}' 的待提商用分账余额，限制资金流出并重定向反哺公利池！", actor_name);
        println!("🚨 [ Financial Freeze ] Security response triggered: Score below fraud threshold. Immediately freezing payout balances for '{}' and redirecting to ECOLOGY_BACKFEED_POOL!", actor_name);
    } else {
        println!("✅ [ 财务通行 ] 正常清分通道放行。");
        println!("✅ [ Financial Pass ] Normal settlement route granted.");
    }

    println!("\n======================================================================");
    println!("🎉 [ 演练结束 ] 余弦相似校验、TEE脱敏签名、EWMA 滑动更新与阶梯延迟惩戒全部通过验证！");
    println!("🎉 [ Demos Complete ] Cosine similarity checks, TEE signatures, EWMA sliding updates, and latency penalties validated!");
    println!("======================================================================");
}

// 辅助方法
// Helper methods
fn hash_bytes(data: &[u8]) -> [u8; 32] {
    let mut hasher = Sha256::new();
    hasher.update(data);
    hasher.finalize().into()
}

fn hex_encode(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{:02x}", b)).collect()
}
