// P-RZZH-050-QNLOO-2026: 本地隐私确权与同步 - 链上真诚审计智能合约模块 (Audit Contract Module)
// P-RZZH-050-QNLOO-2026: Local Privacy Confirmation & Sync - On-chain Sincerity Audit Contract Module
// 物理安全机制：签名验签防重放 + EWMA 指数移动平均得分计算 + 三级梯度惩戒状态机控制
// Physical Security: Signature validation & anti-replay + EWMA score computation + 3-tier gradient penalty state machine

use std::collections::HashMap;
use sha2::{Sha256, Digest};

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SincerityState {
    Trusted,
    Suspicious,
    Fraudulent,
}

#[derive(Clone, Debug)]
pub struct ActorProfile {
    #[allow(dead_code)]
    pub actor_address: String,
    // 当前真诚度得分，默认 100
    // Current sincerity score, default 100
    pub sincerity_weight: u32,
    pub state: SincerityState,
    #[allow(dead_code)]
    pub last_update_time: u64,
    pub public_key: Vec<u8>,
    // 乐观锁版本号
    // Optimistic lock version number
    pub version: u64,
    // 连续欺诈判定次数，用于网关层动态阶梯式时延惩戒
    // Accumulate consecutive fraud counts for gateway tiered latency penalty calculation
    pub consecutive_fraud_count: u32,
}

pub struct LogicContinuityContract {
    pub actors: HashMap<String, ActorProfile>,
    pub used_nonces: HashMap<[u8; 32], bool>,
}

impl LogicContinuityContract {
    pub fn new() -> Self {
        Self {
            actors: HashMap::new(),
            used_nonces: HashMap::new(),
        }
    }

    pub fn register_actor(&mut self, actor: String, pub_key: Vec<u8>) -> Result<(), &'static str> {
        // 提示：在生产环境中，需要对公钥进行设备证书链（如 TEE Attestation）的合法身份认证，防止恶意节点注册伪造的公钥。
        // Note: In production, the public key must undergo device-certificate chain validation (e.g., TEE Attestation) to prevent malicious actors from registering forged public keys.
        if self.actors.contains_key(&actor) {
            return Err("REGISTRATION_ERROR: Actor already registered");
        }
        let profile = ActorProfile {
            actor_address: actor.clone(),
            sincerity_weight: 100,
            state: SincerityState::Trusted,
            last_update_time: 0,
            public_key: pub_key,
            version: 1,
            consecutive_fraud_count: 0,
        };
        self.actors.insert(actor, profile);
        Ok(())
    }

    // 获取行为体动态审计得分与梯度状态及连续欺诈计数
    // Retrieve actor's dynamic audit score, reputation tier state, and consecutive fraud count
    // 对应从属权利要求5、6：跨智能合约分账提取权限校验联动与动态阶梯时延惩戒
    // Aligning with Claims 5 & 6: Cross-contract verification linkage of billing withdrawal rights and dynamic tiered latency penalties
    pub fn get_actor_state(&self, actor: String) -> Result<(u32, SincerityState, u32), &'static str> {
        let profile = self.actors.get(&actor).ok_or("NOT_FOUND_ERROR: Actor profile not found")?;
        Ok((profile.sincerity_weight, profile.state, profile.consecutive_fraud_count))
    }

    // 提交 TEE 审计判定结果并更新信誉积分
    // Submit TEE audit proof and update reputation score
    // 对应权利要求1、4、7、9：签名验签，防重放，版本乐观锁，EWMA滑动窗口，状态机转移
    // Aligning with Claims 1, 4, 7, 9: Signature validation, anti-replay, optimistic locking, EWMA sliding window, state machine transition
    pub fn submit_sincerity_proof(
        &mut self,
        actor: String,
        consistency_score: u32,
        nonce: [u8; 32],
        expected_version: u64,
        proof_data: Vec<u8>,
        signature: Vec<u8>,
    ) -> Result<u32, &'static str> {
        // 1. 防重放 Nonce 校验
        // 1. Anti-replay Nonce check
        if self.used_nonces.contains_key(&nonce) {
            return Err("REPLAY_ATTACK_ERROR: Nonce has already been validated");
        }
        self.used_nonces.insert(nonce, true);

        // 2. 获取行为体 Profile 并进行乐观锁校验
        // 2. Retrieve Actor Profile and perform optimistic lock validation
        let profile = self.actors.get_mut(&actor).ok_or("NOT_FOUND_ERROR: Actor profile not found")?;
        if profile.version != expected_version {
            return Err("CONCURRENCY_ERROR: Profile state version mismatch");
        }
        profile.version = profile.version
            .checked_add(1)
            .ok_or("INTEGER_OVERFLOW_ERROR: Profile version increment overflow")?;

        // 3. 校验签名真实性 (使用预置的 TEE 私钥派生公钥逻辑模拟)
        // 3. Validate signature authenticity (simulated via TEE pre-shared keys)
        if !Self::verify_tee_signature(&profile.public_key, &proof_data, &signature) {
            return Err("SECURITY_ALERT_ERROR: Cryptographic signature mismatch! Rejected.");
        }

        if consistency_score > 100 {
            return Err("INVALID_SCORE_ERROR: Consistency score cannot exceed 100");
        }

        // 4. EWMA 动态更新积分权重
        // 4. EWMA dynamic score weight update
        // 对应权利要求3：计算模型 W_t = \alpha * W_{t-1} + (1 - \alpha) * S_t，设 alpha = 0.85
        // 提示：此处采用纯整数安全运算替代浮点数，以避免智能合约环境下的浮点精度不一致与误差风险。
        // Aligning with Claim 3: EWMA model W_t = \alpha * W_{t-1} + (1 - \alpha) * S_t, with alpha = 0.85
        // Note: Safe integer arithmetic is used here instead of floating-point math to prevent precision and consistency discrepancies in smart contract environments.
        let current_weight = profile.sincerity_weight;
        let term_a = current_weight
            .checked_mul(85)
            .ok_or("INTEGER_OVERFLOW_ERROR: Sincerity weight multiplication overflow")?;
        let term_b = consistency_score
            .checked_mul(15)
            .ok_or("INTEGER_OVERFLOW_ERROR: Consistency score multiplication overflow")?;
        let next_weight = term_a
            .checked_add(term_b)
            .ok_or("INTEGER_OVERFLOW_ERROR: Sincerity score addition overflow")?
            .checked_div(100)
            .ok_or("INTEGER_DIVISION_BY_ZERO_ERROR: Division error in weight calculation")?;
        profile.sincerity_weight = next_weight;

        // 5. 三级梯度状态机转换
        // 5. 3-tier gradient state machine transition
        // 对应权利要求1、4：更新节点等级并联动网关时延及分账状态
        // Aligning with Claims 1 & 4: Update node tier and cascade to gateway latency/billing state
        if next_weight >= 80 {
            profile.state = SincerityState::Trusted;
            // 恢复至可信状态，清零连续欺诈计数以解除时延惩戒
            // Restored to Trusted state, reset consecutive fraud count to lift latency penalty
            profile.consecutive_fraud_count = 0;
        } else if next_weight >= 50 {
            profile.state = SincerityState::Suspicious;
            // 恢复至可疑状态，同样清零连续计数，代表节点表现好转
            // Restored to Suspicious state, reset consecutive count indicating node behavior improved
            profile.consecutive_fraud_count = 0;
        } else {
            profile.state = SincerityState::Fraudulent;
            profile.consecutive_fraud_count = profile.consecutive_fraud_count
                .checked_add(1)
                .unwrap_or(u32::MAX);
        }

        Ok(next_weight)
    }

    // 模拟签名校验
    // Simulate signature verification
    fn verify_tee_signature(_pub_key: &[u8], message: &[u8], signature: &[u8]) -> bool {
        // 校验 HMAC 哈希是否匹配主权安全密钥
        // Validate if HMAC matches sovereign secure key
        let mut hmac = Sha256::new();
        hmac.update(b"ACTOR_TEE_SECURE_PRIVATE_KEY_202");
        hmac.update(message);
        let expected_signature = hmac.finalize().to_vec();
        signature == expected_signature
    }
}
