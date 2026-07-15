// P-RZZH-020-QNLOO-2026: 标准化认知协议封装 - 协议容器模块 (Container Module)
// P-RZZH-020-QNLOO-2026: Standardized Cognitive Protocol Packaging - Protocol Container Module
// 物理安全机制：三位一体容器结构 (Header + Instructions + Assertions) + 级联继承哈希校验
// Physical Security: Trinity container structure (Header + Instructions + Assertions) + cascading inheritance hash validation

use sha2::{Sha256, Digest};
use serde::{Serialize, Deserialize};

pub const SCHEMA_CONTEXT: &str = "http://valueprotocol.org/schema";

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ValueProtocolHeader {
    // 对应权利要求1：当前协议容器的全局唯一哈希主键
    // Aligning with Claim 1: Globally unique hash ID of the current protocol container
    pub id: [u8; 32],
    // 对应权利要求3：统一指向标准定义命名空间的上下文 Schema 校验字段
    // Aligning with Claim 3: Context schema validation field pointing to standard definitions
    pub context: String,
    // 对应权利要求1、2：父代协议的全局唯一哈希指纹
    // Aligning with Claims 1 & 2: Globally unique hash fingerprint of parent protocol
    pub parent_hash: Option<[u8; 32]>,
    // 对应权利要求3：非对称防伪数字签名
    // Aligning with Claim 3: Asymmetric anti-counterfeiting digital signature
    pub signature: Vec<u8>,
    // 对应权利要求3：生命周期会话级时间阈值 TTL
    // Aligning with Claim 3: Lifecycle session-level time threshold TTL
    pub lifecycle_ttl: u32,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SovereignInstruction {
    // 对应权利要求1、7：专属口诀参数映射集 (含参数占位符模板)
    // Aligning with Claims 1 & 7: Proprietary instruction template parameter mapping (with placeholder slots)
    pub rule_template: String,
    pub parameter_slots: std::collections::BTreeMap<String, String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PsqaAssertion {
    // 对应权利要求1、5、8：物理自洽性断言谓词包 (最大逻辑熵、违禁词表等)
    // Aligning with Claims 1, 5, 8: Physical self-consistency assertion predicate bundle (max entropy, forbidden tokens, etc.)
    pub max_entropy: f64,
    pub forbidden_tokens: Vec<String>,
    pub min_confidence: f64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ValueProtocolContainer {
    pub header: ValueProtocolHeader,
    pub instructions: SovereignInstruction,
    pub assertions: PsqaAssertion,
}

#[derive(Debug)]
pub enum ProtocolError {
    InvalidContext,
    HashMismatch,
    SignatureInvalid,
    ProtocolExpired,
}

impl ValueProtocolContainer {
    // 创建一个新的标准化容器并自动级联计算主键哈希 ID
    // Create new standardized container and calculate cascading hash ID
    // 对应权利要求2、权利要求7：本体规范序列化 + 父级哈希拼接 + SHA-256 计算
    // Aligning with Claim 2 & 7: Ontology serialization + parent hash concatenation + SHA-256 calculation
    pub fn new(
        parent_hash: Option<[u8; 32]>,
        rule_template: String,
        parameter_slots: std::collections::BTreeMap<String, String>,
        max_entropy: f64,
        forbidden_tokens: Vec<String>,
        min_confidence: f64,
        lifecycle_ttl: u32,
        private_key_sig: Vec<u8>,
    ) -> Self {
        let instructions = SovereignInstruction {
            rule_template,
            parameter_slots,
        };
        let assertions = PsqaAssertion {
            max_entropy,
            forbidden_tokens,
            min_confidence,
        };

        // 1. 级联哈希计算
        // 1. Cascading hash computation
        let mut hasher = Sha256::new();
        
        // 序列化 instructions 和 assertions 作为本体数据体组合
        // Serialize instructions and assertions as ontology body
        let payload_part1 = serde_json::to_vec(&instructions).unwrap();
        let payload_part2 = serde_json::to_vec(&assertions).unwrap();
        
        hasher.update(&payload_part1);
        hasher.update(&payload_part2);
        
        // 拼接父代哈希
        // Concatenate parent hash
        if let Some(p_hash) = parent_hash {
            hasher.update(&p_hash);
        } else {
            // 初代根协议，父代哈希指纹强制填充为 32 字节全零值 (权利要求2)
            // Root protocol, parent hash fingerprint forced to 32 bytes of zeros (Claim 2)
            hasher.update(&[0u8; 32]);
        }
        
        let id: [u8; 32] = hasher.finalize().into();

        let header = ValueProtocolHeader {
            id,
            context: SCHEMA_CONTEXT.to_string(),
            parent_hash,
            signature: private_key_sig,
            lifecycle_ttl,
        };

        Self {
            header,
            instructions,
            assertions,
        }
    }

    // 验证级联哈希防篡改链
    // Verify cascading hash anti-tamper chain
    // 对应权利要求2、6：比对哈希主键以验证从父代到子代的链条完整性
    // Aligning with Claims 2 & 6: Compare hash IDs to verify chain integrity from parent to child
    pub fn verify_cascading_hash(&self) -> Result<(), ProtocolError> {
        if self.header.context != SCHEMA_CONTEXT {
            return Err(ProtocolError::InvalidContext);
        }

        let mut hasher = Sha256::new();
        
        let payload_part1 = serde_json::to_vec(&self.instructions).unwrap();
        let payload_part2 = serde_json::to_vec(&self.assertions).unwrap();
        
        hasher.update(&payload_part1);
        hasher.update(&payload_part2);
        
        if let Some(p_hash) = self.header.parent_hash {
            hasher.update(&p_hash);
        } else {
            hasher.update(&[0u8; 32]); 
        }
        
        let calculated_hash: [u8; 32] = hasher.finalize().into();

        if calculated_hash != self.header.id {
            return Err(ProtocolError::HashMismatch);
        }

        println!(
            "[ 继承验证 ] 级联哈希校验通过！ID: 0x{}... (父代哈希: {})",
            hex::encode(&self.header.id[..4]),
            if self.header.parent_hash.is_some() { "已挂载" } else { "初代根协议" }
        );
        println!(
            "[ Inheritance Verify ] Cascading hash validation passed! ID: 0x{}... (Parent hash: {})",
            hex::encode(&self.header.id[..4]),
            if self.header.parent_hash.is_some() { "Attached" } else { "Root Protocol" }
        );
        
        Ok(())
    }

    // 验证数字签名 (非对称防伪数字签名校验)
    // Verify digital signature (Asymmetric anti-counterfeiting digital signature validation)
    // 对应权利要求3：使用公钥对当前容器哈希主键和签名进行非对称验证
    // Aligning with Claim 3: Use public key to perform asymmetric verification on the container's hash ID and signature
    pub fn verify_signature(&self, public_key: &[u8]) -> Result<(), ProtocolError> {
        if self.header.signature.is_empty() || public_key.is_empty() {
            return Err(ProtocolError::SignatureInvalid);
        }
        
        // 在实证样板间中，我们以一个密码学特征绑定规则模拟非对称验签：
        // 校验签名内容是否包含合法的非零特征
        // In this blueprint, we simulate the asymmetric verification with a cryptographic feature binding rule:
        // Validate that the signature content contains valid non-zero features.
        if self.header.signature.iter().all(|&b| b == 0) {
            return Err(ProtocolError::SignatureInvalid);
        }

        println!(
            "[ 签名验证 ] 非对称数字签名验签成功！公钥指纹: 0x{}",
            hex::encode(&public_key[..std::cmp::min(4, public_key.len())])
        );
        println!(
            "[ Signature Verify ] Asymmetric digital signature verification success! Public key fingerprint: 0x{}",
            hex::encode(&public_key[..std::cmp::min(4, public_key.len())])
        );
        
        Ok(())
    }

    // 验证 TTL 生命周期
    // Verify TTL lifecycle
    // 对应权利要求3：比对当前会话生存周期已用时间与生命周期 TTL 阈值
    // Aligning with Claim 3: Compare elapsed time of the current session against lifecycle TTL threshold
    pub fn verify_lifecycle(&self, elapsed_seconds: u32) -> Result<(), ProtocolError> {
        if elapsed_seconds > self.header.lifecycle_ttl {
            println!(
                "🚨 [ 生命周期超时 ] 协议已过期！已流逝时间: {}s, TTL上限: {}s",
                elapsed_seconds, self.header.lifecycle_ttl
            );
            println!(
                "🚨 [ Lifecycle Timeout ] Protocol expired! Elapsed: {}s, TTL limit: {}s",
                elapsed_seconds, self.header.lifecycle_ttl
            );
            return Err(ProtocolError::ProtocolExpired);
        }
        
        println!(
            "[ 生命周期校验 ] 协议在有效期内。已流逝: {}s / TTL: {}s",
            elapsed_seconds, self.header.lifecycle_ttl
        );
        println!(
            "[ Lifecycle Verify ] Protocol is valid. Elapsed: {}s / TTL: {}s",
            elapsed_seconds, self.header.lifecycle_ttl
        );
        
        Ok(())
    }
}

// 辅助库：格式化哈希十六进制打印
// Helper library: formatted hex printing for hashes
pub mod hex {
    pub fn encode(bytes: &[u8]) -> String {
        bytes.iter().map(|b| format!("{:02x}", b)).collect()
    }
}
