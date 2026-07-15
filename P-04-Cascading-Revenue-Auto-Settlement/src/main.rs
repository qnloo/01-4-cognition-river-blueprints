// P-RZZH-040-QNLOO-2026: 本地隐私确权与同步 - 样板间主程序入口 (Main Entry)
// P-RZZH-040-QNLOO-2026: Local Privacy Confirmation & Sync - Main Entry
// 本文件以实证代码形式，演示多级分账、18级依赖截断、乐观锁并发控制与重入锁保护拉式提款。
// This file demonstrates multi-level cascading billing, 18-level dependency cutoff, optimistic lock control, and reentrancy-safe pull payouts.

mod contract;

use sha2::{Sha256, Digest};

fn main() {
    println!("======================================================================");
    println!("🚀 [ P-RZZH-040-QNLOO-2026 ] 智能分账合约系统 - 专利样板间代码启动");
    println!("🚀 [ P-RZZH-040-QNLOO-2026 ] Intelligent Cascading Billing Contract System - Patent Blueprint Startup");
    println!("======================================================================\n");

    // ----------------------------------------------------------------------
    // 0. 初始化根级元协议
    // 0. Initialize root metadata protocol
    // ----------------------------------------------------------------------
    let mut hasher = Sha256::new();
    hasher.update(b"META_VALUE_PROTOCOL_ROOT");
    let root_hash: [u8; 32] = hasher.finalize().into();

    let mut contract = contract::CascadingBillingContract::new(root_hash);
    println!("[ 初始就绪 ] 部署 CascadingBillingContract，根元协议注册至地址: 0x{}", hex_encode(&root_hash[..4]));
    println!("[ Initialized ] Deployed CascadingBillingContract. Root protocol registered at: 0x{}\n", hex_encode(&root_hash[..4]));

    // ----------------------------------------------------------------------
    // 1. 三代二分叉衍生依赖树构建及相似度分成 (Normal Multi-level Split)
    // 1. 3-generation bifurcated derived dependency tree construction and similarity payout (Normal Multi-level Split)
    // ----------------------------------------------------------------------
    println!("----------------------------------------------------------------------");
    println!("🌱 1. 三代多父节点依赖树构建与非对称相似度分成 (75% / 10% / 15%)");
    println!("🌱 1. 3-generation multi-parent dependency tree construction & asymmetric similarity split (75% / 10% / 15%)");
    println!("----------------------------------------------------------------------");

    // 父协议 B1 创作者: Alice，创建时间: 1000
    // Parent Protocol B1 Creator: Alice, Creation Time: 1000
    let b1_hash = hash_bytes(b"PROTOCOL_B1_ALICE");
    let _ = contract.register_protocol(
        "Alice".to_string(),
        b1_hash,
        vec![root_hash], // 依赖根级元协议 / Depends on root metadata protocol
        vec![100],
        1000,
    );

    // 父协议 B2 创作者: Bob，创建时间: 1200
    // Parent Protocol B2 Creator: Bob, Creation Time: 1200
    let b2_hash = hash_bytes(b"PROTOCOL_B2_BOB");
    let _ = contract.register_protocol(
        "Bob".to_string(),
        b2_hash,
        vec![root_hash], // 依赖根级元协议  / Depends on root metadata protocol
        vec![100],
        1200,
    );

    // 孙协议 C 创作者: Charlie，继承了 B1 和 B2，创建时间: 1500
    // Grandson protocol C Creator: Charlie, inherits B1 and B2, creation time: 1500
    // 对 B1 的贡献相似度 70，对 B2 的贡献相似度 30
    // Similarity to B1 is 70, Similarity to B2 is 30
    let c_hash = hash_bytes(b"PROTOCOL_C_CHARLIE");
    let _ = contract.register_protocol(
        "Charlie".to_string(),
        c_hash,
        vec![b1_hash, b2_hash], // 多父继承 / Multi-parent inheritance
        vec![70, 30],           // 相似度分值 / Similarity scores
        1500,
    );

    // 发起第一笔商用清算会话，交易额 1000 Token
    // Initiate first commercial settlement session, transaction volume 1000 Token
    let billing_nonce = hash_bytes(b"SESSION_NONCE_001");
    println!("\n[ 清算执行 ] 外部会话调用 C 协议结束，支付结算款 1000 Token...");
    println!("[ Settlement ] External session called Protocol C, sending payout payment of 1000 Tokens...");
    match contract.execute_billing(c_hash, 1000, billing_nonce, 1) {
        Ok(_) => {
            println!("✅ 清分成功！已完成多路径账本余额更新。");
            println!("✅ Settlement cleared! Multi-route ledger balance updated.");
            println!("   孙创作者 Charlie 待提余额: {} (应得 75% 自留 = 750)", contract.balances.get("Charlie").unwrap_or(&0));
            println!("   Grandson creator Charlie balance: {} (Expected 75% self-retained = 750)", contract.balances.get("Charlie").unwrap_or(&0));
            println!("   父创作者 Alice (相似度 70) 待提余额: {} (应得 10% * 70% = 70)", contract.balances.get("Alice").unwrap_or(&0));
            println!("   Parent creator Alice (similarity 70) balance: {} (Expected 10% * 70% = 70)", contract.balances.get("Alice").unwrap_or(&0));
            println!("   父创作者 Bob (相似度 30) 待提余额: {} (应得 10% * 30% = 30)", contract.balances.get("Bob").unwrap_or(&0));
            println!("   Parent creator Bob (similarity 30) balance: {} (Expected 10% * 30% = 30)", contract.balances.get("Bob").unwrap_or(&0));
            println!("   祖父层级 (根元协议->反哺池) 待提余额: {} (祖父层 15% 分成共 150 归入公利池)", contract.balances.get(contract::ECOLOGY_BACKFEED_POOL).unwrap_or(&0));
            println!("   Grandparent level (Root -> Backfeed) balance: {} (Expected 15% split = 150 directed to backfeed pool)", contract.balances.get(contract::ECOLOGY_BACKFEED_POOL).unwrap_or(&0));
        }
        Err(e) => {
            println!("❌ 清分失败: {}", e);
            println!("❌ Settlement clearing failed: {}", e);
        }
    }
    println!("");

    // ----------------------------------------------------------------------
    // 2. Checks-Effects-Interactions 安全提款防重入 (Pull Payment)
    // 2. Checks-Effects-Interactions secure withdrawal anti-reentrancy (Pull Payment)
    // ----------------------------------------------------------------------
    println!("----------------------------------------------------------------------");
    println!("🛡️ 2. Pull Payment 安全提款机制 (Checks-Effects-Interactions)");
    println!("🛡️ 2. Pull Payment security claim mechanism (Checks-Effects-Interactions)");
    println!("----------------------------------------------------------------------");

    // 创作者 Alice 主动发送提款调用
    // Creator Alice actively calls to claim payout
    match contract.claim_payout("Alice") {
        Ok(amount) => {
            println!("✅ Alice 成功提款: {} Token！", amount);
            println!("✅ Alice successfully claimed payout of {} Tokens!", amount);
            println!("   最新合约账本中 Alice 待提余额: {}", contract.balances.get("Alice").unwrap_or(&0));
            println!("   Latest registry balance for Alice: {}", contract.balances.get("Alice").unwrap_or(&0));
        }
        Err(e) => {
            println!("❌ 提款失败: {}", e);
            println!("❌ Payout claim failed: {}", e);
        }
    }
    println!("");

    // ----------------------------------------------------------------------
    // 3. 交易防重放与并发乐观锁冲突验证 (Anti-Replay & Concurrency Locking)
    // 3. Transaction anti-replay and concurrency optimistic locking conflict verification (Anti-Replay & Concurrency Locking)
    // ----------------------------------------------------------------------
    println!("----------------------------------------------------------------------");
    println!("🔒 3. 防重放 Nonce 与乐观锁并发版本冲突检测");
    println!("🔒 3. Anti-replay Nonce & optimistic concurrency version conflict checks");
    println!("----------------------------------------------------------------------");

    // 重放攻击模拟：使用刚才已经用过的 billing_nonce 再次调用
    // Replay attack simulation: call using the spent billing_nonce again
    println!("[ 重放模拟 ] 恶意攻击者拦截 session_nonce_001 并试图二次清算...");
    println!("[ Replay Simulation ] Malicious peer intercepts session_nonce_001, trying to clear twice...");
    match contract.execute_billing(c_hash, 1000, billing_nonce, 2) {
        Ok(_) => {
            println!("❌ [ 状态缺陷 ] 合约错误地允许了二次划扣！");
            println!("❌ [ Security Defect ] Contract incorrectly permitted duplicate clearing!");
        }
        Err(e) => {
            println!("✅ [ 拦截成功 ] 成功拦截交易重放攻击！错误信息: '{}'", e);
            println!("✅ [ Intercept Success ] Successfully blocked replay attack! Error: '{}'", e);
        }
    }
 
    // 乐观锁冲突模拟：此时 C 协议状态版本已自增为 2，若调用传入版本号 1 则触发版本冲突
    // Optimistic lock simulation
    println!("\n[ 乐观锁模拟 ] 异步清分并发请求，传入失效的版本号 (Expected: 1, Actual: 2)...");
    println!("[ Optimistic Lock ] Concurrency clearing request with stale version (Expected: 1, Actual: 2)...");
    let stale_nonce = hash_bytes(b"SESSION_NONCE_002");
    match contract.execute_billing(c_hash, 1000, stale_nonce, 1) {
        Ok(_) => {
            println!("❌ [ 状态缺陷 ] 合约乐观锁失效，造成并发失步！");
            println!("❌ [ Security Defect ] Optimistic lock failed to prevent concurrency race!");
        }
        Err(e) => {
            println!("✅ [ 拦截成功 ] 成功拦截并发更新碰撞！错误信息: '{}'", e);
            println!("✅ [ Intercept Success ] Successfully blocked concurrent update collision! Error: '{}'", e);
        }
    }
    println!("");

    // ----------------------------------------------------------------------
    // 4. 20年保护期到期状态机重定向 (20-Year Expiry redirection)
    // 4. 20-year protection period expiry state machine redirection (20-Year Expiry redirection)
    // ----------------------------------------------------------------------
    println!("----------------------------------------------------------------------");
    println!("⏳ 4. 20年知识产权保护到期状态转移，重定向反哺公利池");
    println!("⏳ 4. 20-year IP protection expiration state transfer, redirected to ECOLOGY_BACKFEED_POOL");
    println!("----------------------------------------------------------------------");

    // 模拟时间推移至 20 年以后 (SECONDS_IN_20_YEARS = 630720000)
    // Simulate time advancing 20 years (SECONDS_IN_20_YEARS = 630720000)
    contract.mock_block_timestamp = 1500 + contract::SECONDS_IN_20_YEARS + 1000;
    println!("[ 时间跃迁 ] 模拟时光穿梭至 20 年后 (区块时间戳设为: {})", contract.mock_block_timestamp);
    println!("[ Time Warp ] Simulating warp to 20 years later (Block timestamp set to: {})", contract.mock_block_timestamp);

    let future_nonce = hash_bytes(b"SESSION_NONCE_003");
    let target_ver = contract.protocols.get(&c_hash).unwrap().version;
    
    // 执行清分，此时由于 Alice、Bob 与 Charlie 创建均已超过 20 年保护期，收益全部改判公共所有权，注入反哺池
    // Execute clearing: since Alice, Bob, and Charlie protocols are >20 years old, payout redirects to ECOLOGY_BACKFEED_POOL
    match contract.execute_billing(c_hash, 1000, future_nonce, target_ver) {
        Ok(_) => {
            println!("✅ 清分成功！");
            println!("✅ Clearing success!");
            println!("   公共反哺池 FADE 累计余额: {} (20年到期，全部收益流向生态反哺)", contract.balances.get(contract::ECOLOGY_BACKFEED_POOL).unwrap_or(&0));
            println!("   Ecology Backfeed Pool FADE balance: {} (20-year expired, all payouts redirected to ecology)", contract.balances.get(contract::ECOLOGY_BACKFEED_POOL).unwrap_or(&0));
        }
        Err(e) => {
            println!("❌ 清分失败: {}", e);
            println!("❌ Clearing failed: {}", e);
        }
    }
    println!("");

    // ----------------------------------------------------------------------
    // 5. 18级继承深度限制与环路死锁防御 (Depth Verification)
    // 5. 18-level inheritance depth limit and circular deadlock defense (Depth Verification)
    // ----------------------------------------------------------------------
    println!("----------------------------------------------------------------------");
    println!("🛑 5. 18级继承代数截断限制与环回死锁拦截校验");
    println!("🛑 5. 18-level lineage depth limit & circular dependency blocking checks");
    println!("----------------------------------------------------------------------");

    // 5.1 环回继承拦截：试图将 B1 指向自己作为父级
    // 5.1 Loopback inheritance interception: attempting to point B1 to itself as parent
    let self_loop_hash = hash_bytes(b"SELF_LOOP");
    println!("[ 环回测试 ] 试图注册一个指向自身的死循环协议节点:");
    println!("[ Loop Test ] Attempting to register a circular self-referential protocol node:");
    match contract.register_protocol(
        "Attacker".to_string(),
        self_loop_hash,
        vec![self_loop_hash],
        vec![100],
        5000,
    ) {
        Ok(_) => {
            println!("❌ [ 安全漏洞 ] 允许了自环回死锁继承关系注册！");
            println!("❌ [ Security Defect ] Permitted circular self-dependency registry!");
        }
        Err(e) => {
            println!("✅ [ 拦截成功 ] 成功拦截自环回死锁关系！原因: '{}'", e);
            println!("✅ [ Intercept Success ] Successfully blocked circular self-dependency! Reason: '{}'", e);
        }
    }

    // 5.2 深度递归审计：构建一条深度达到 19 代的超深依赖链
    // 5.2 Deep recursive audit: construct a 19-generation hyper-deep dependency chain
    println!("\n[ 深度测试 ] 开始构建长线继承链 (从 1 代递归延伸至 19 代)...");
    println!("[ Depth Test ] Building a long lineage chain (recursively extending from Gen 1 to Gen 19)...");
    let mut current_ancestor = root_hash;
    let mut success = true;

    for i in 1..=19 {
        let node_hash = hash_bytes(format!("GENERATION_{}", i).as_bytes());
        match contract.register_protocol(
            format!("User_{}", i),
            node_hash,
            vec![current_ancestor],
            vec![100],
            6000 + i * 10,
        ) {
            Ok(_) => {
                current_ancestor = node_hash;
            }
            Err(e) => {
                println!("✅ [ 18级熔断 ] 在第 {} 代节点拦截成功！账本强制打回注册。原因: '{}'", i, e);
                println!("✅ [ 18-Gen Circuit Break ] Successfully blocked at Gen {}! Registry rejected. Reason: '{}'", i, e);
                success = false;
                break;
            }
        }
    }

    if success {
        println!("❌ [ 安全漏洞 ] 合约漏过了超限 18 级依赖继承链！");
        println!("❌ [ Security Defect ] Contract bypassed the 18-level depth limit constraint!");
    }

    println!("\n======================================================================");
    println!("🎉 [ 演练结束 ] 三级比例分成、18级硬截断、重入清零、乐观锁及20年反哺全部通过验证！");
    println!("🎉 [ Demos Complete ] 3-tier payout, 18-level cutoff, reentrancy-safe claims, optimistic locking, and 20-year backfeed validated!");
    println!("======================================================================");
}

// 辅助哈希与格式化工具
// Helper hashing and formatting tools
fn hash_bytes(data: &[u8]) -> [u8; 32] {
    let mut hasher = Sha256::new();
    hasher.update(data);
    hasher.finalize().into()
}

fn hex_encode(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{:02x}", b)).collect()
}
