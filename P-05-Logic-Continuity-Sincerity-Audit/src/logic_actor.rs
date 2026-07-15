// P-RZZH-050-QNLOO-2026: 本地隐私确权与同步 - 逻辑行为体沙箱模块 (Logic Actor Sandbox Module)
// P-RZZH-050-QNLOO-2026: Local Privacy Confirmation & Sync - Logic Actor Sandbox Module
// 物理安全机制：主权沙箱内余弦相似度比对 + 本地因果检验 + TEE 脱敏签名 + 内存彻底消磁
// Physical Security: Cosine similarity check in sovereign sandbox + local causal verification + TEE desensitized signature + thorough memory scrubbing
// 依赖说明：本模块依赖外部 `rand` crate (例如 rand = "0.8") 用于内存消磁的随机电荷中和覆盖。在生产环境中，应直接调用硬件 TEE 的真随机数发生器 (TRNG)。
// Dependency: This module depends on the external `rand` crate (e.g. rand = "0.8") for random charge neutralization in memory scrubbing. In production, the TEE's hardware True Random Number Generator (TRNG) should be invoked directly.

use sha2::{Sha256, Digest};

pub struct LogicActorSandbox {
    pub actor_private_key: [u8; 32],
    pub actor_public_key: Vec<u8>,
}

impl LogicActorSandbox {
    pub fn new() -> Self {
        let mut key = [0u8; 32];
        key.copy_from_slice(b"ACTOR_TEE_SECURE_PRIVATE_KEY_202");
        Self {
            actor_private_key: key,
            actor_public_key: b"ACTOR_TEE_SECURE_PUBLIC_KEY_2026".to_vec(),
        }
    }

    // 计算需求特征向量 V_D 与方案特征向量 V_P 的余弦相似度
    // Calculate cosine similarity between Demand Vector V_D and Proposal Vector V_P
    // 对应权利要求1、4：余弦相似度计算与语义对齐校验
    // Aligning with Claims 1 & 4: Cosine similarity calculation and semantic alignment check
    pub fn cos_similarity(&self, v1: &[f32], v2: &[f32]) -> f32 {
        if v1.len() != v2.len() || v1.is_empty() {
            return 0.0;
        }
        let mut dot_product = 0.0;
        let mut norm_a = 0.0;
        let mut norm_b = 0.0;

        for i in 0..v1.len() {
            dot_product += v1[i] * v2[i];
            norm_a += v1[i] * v1[i];
            norm_b += v2[i] * v2[i];
        }

        let denominator = norm_a.sqrt() * norm_b.sqrt();
        if denominator < f32::EPSILON {
            return 0.0;
        }

        dot_product / denominator
    }

    // 本地评估物理动作与效果反馈之间的因果关系
    // Evaluate local causal relationship between physical actions and outcome feedback
    // 对应权利要求1、4、8：对动作哈希与效果特征比对进行本地因果关系判定
    // Aligning with Claims 1, 4, 8: Local causal relationship determination via action hash and outcome feature mapping
    // 提示：在生产环境中，完整的因果判定实现需要通过动作哈希查询其在上链或本地存证时的绝对时间戳 T_action，
    // 并校验其与效果反馈捕获时间戳 T_effect 之间的差值是否满足 |T_effect - T_action| <= θ_time_window (时序窗口阈值)。
    // 若超出时序窗口，或未找到对应的动作哈希，则判定为发生逻辑断裂。
    // Note: In production, the causal determination requires querying the absolute timestamp T_action of the action on-chain or locally via the action hash,
    // and validating that the difference with the outcome feedback capture timestamp T_effect satisfies |T_effect - T_action| <= θ_time_window.
    // If the difference exceeds the temporal window or no matching action hash is found, it is judged as a logical disconnection.
    pub fn local_causal_verify(&self, action_hash: &[u8; 32], outcome_metrics: &[f32]) -> u32 {
        // 模拟提取本地敏感物理参数明细进行判定
        // Simulate extraction of local sensitive physical parameters for evaluation
        let sum: f32 = outcome_metrics.iter().sum();
        let hash_weight = action_hash[0] as f32 / 255.0;
        
        let score = if sum > 0.1 && hash_weight > 0.0 {
            // 因果自洽性极高
            // Extremely high causal self-consistency
            95
        } else {
            // 发生逻辑自洽断裂
            // Logic self-consistency breakage occurred
            10
        };
        score
    }

    // 在 TEE 中生成脱敏审计证明与密码学签名，并在完成后对内存区执行消磁擦除
    // Generate desensitized audit proof and cryptographic signature in TEE, and scrub memory upon completion
    // 对应权利要求1、4、8、10：清除物理敏感参数，生成脱敏布尔证明与防重放 Nonce 签名
    // Aligning with Claims 1, 4, 8, 10: Scrub physical sensitive parameters, generate desensitized boolean proof and anti-replay Nonce signature
    pub fn generate_tee_proof(
        &self,
        consistency_score: u32,
        nonce: [u8; 32],
    ) -> (Vec<u8>, Vec<u8>) {
        // 1. 脱敏处理：将敏感的具体工业数据或操作明细清除，只输出抽象脱敏判定证明
        // 1. Desensitization: Strip sensitive specific industrial data or operation details, output abstract boolean proof only
        let mut proof_data = Vec::new();
        proof_data.extend_from_slice(b"PROOF_CONSISTENCY_TRUE_SCORE:");
        proof_data.push(consistency_score as u8);
        proof_data.extend_from_slice(&nonce);

        // 2. 使用硬件内部受保护的私钥对脱敏数据生成密码学签名 (使用 HMAC 模拟签名)
        // 2. Generate cryptographic signature over desensitized data using hardware-protected private key (simulated using HMAC)
        let mut hmac = Sha256::new();
        hmac.update(&self.actor_private_key);
        hmac.update(&proof_data);
        let signature = hmac.finalize().to_vec();

        // 3. 对应权利要求8：在向外部发送前，必须在 TEE 物理内存隔离区中对底层明文数据执行消磁清空
        // 3. Aligning with Claim 8: Before outbound transmission, must scrub and demagnetize underlying plaintext data within TEE isolated memory
        self.secure_erase_sandbox_secrets();

        (proof_data, signature)
    }

    // 内存消磁
    // Secure memory demagnetization and scrubbing
    fn secure_erase_sandbox_secrets(&self) {
        // 模拟对 TEE 内部临时装载敏感数据的堆物理内存进行 3 遍擦除覆写
        // Simulate 3-pass overwrite scrubbing on TEE internal heap memory temporarily storing sensitive data
        let mut sensitive_buffer = vec![0xABu8; 256];
        let ptr = sensitive_buffer.as_mut_ptr();
        let size = sensitive_buffer.len();

        unsafe {
            // Pass 1: 物理内存覆写全 0x00，使用 write_volatile 阻止编译器死商店消除（DSE）
            // Pass 1: Overwrite memory with 0x00 using write_volatile to prevent Dead Store Elimination (DSE)
            for i in 0..size {
                ptr.add(i).write_volatile(0x00);
            }
            // Pass 2: 物理内存覆写全 0xFF
            // Pass 2: Overwrite memory with 0xFF
            for i in 0..size {
                ptr.add(i).write_volatile(0xFF);
            }
            // Pass 3: 写入强密码学伪随机噪声，中和电荷
            // Pass 3: Write cryptographically secure pseudo-random noise to neutralize charges
            let mut rng = rand::thread_rng();
            let slice = std::slice::from_raw_parts_mut(ptr, size);
            rand::RngCore::fill_bytes(&mut rng, slice);
            for i in 0..size {
                ptr.add(i).write_volatile(slice[i]);
            }
            
            // 物理清刷各级 CPU 高速缓存行以中和硅片电荷残留
            // Physically flush CPU cache lines across all levels to neutralize residual silicon charges
            #[cfg(target_arch = "x86_64")]
            for offset in (0..size).step_by(64) {
                std::arch::x86_64::_mm_clflush(ptr.add(offset));
            }
            #[cfg(target_arch = "aarch64")]
            for offset in (0..size).step_by(64) {
                std::arch::asm!(
                    "dc cvac, {0}",
                    "dsb sy",
                    in(reg) ptr.add(offset),
                    options(nostack, preserves_flags)
                );
            }
        }
    }
}
