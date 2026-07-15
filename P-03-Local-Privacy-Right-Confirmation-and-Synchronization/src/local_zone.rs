// P-RZZH-030-QNLOO-2026: 本地隐私确权与同步 - 本地安全区模块 (Local Security Zone)
// P-RZZH-030-QNLOO-2026: Local Privacy Confirmation & Sync - Local Security Zone Module
// 物理安全机制：本地切片向量指纹计算 + 语法偏差水印植入 + Loopback 仅本地 IPC 绑定
// Physical Security: Local slice vector fingerprint calculation + syntactic drift watermarking + local Loopback-only IPC binding

use sha2::{Sha256, Digest};
use std::net::IpAddr;

#[derive(Clone, Debug)]
pub struct LocalTextChunk {
    pub text_content: String,
    pub embedding_vector: Vec<f32>,
}

pub struct HashFingerprint {
    pub hash_id: [u8; 32],
    pub timestamp: u64,
}

pub struct LocalVectorDb {
    // 向量库监听绑定的物理网卡地址
    // Physical network interface address bound to the vector database listener
    pub listen_address: IpAddr,
    // 是否仅限制 IPC (进程间通信) 通道访问
    // Whether access is strictly limited to local IPC (Inter-Process Communication)
    #[allow(dead_code)]
    pub local_ipc_only: bool,
    // 本地存储的向量-明文关联库
    // Locally stored vector-plaintext association storage
    storage: std::collections::HashMap<String, LocalTextChunk>,
}

impl LocalTextChunk {
    pub fn new(text: &str, vector: Vec<f32>) -> Self {
        Self {
            text_content: text.to_string(),
            embedding_vector: vector,
        }
    }

    // 本地计算级联哈希指纹并植入特征语法水印
    // Compute cascading hash fingerprint and inject syntactic drift watermark locally
    // 对应权利要求2、6、8：指纹公式 H_chunk = SHA256(v_emb || TextContent || Salt) + 基于哈希二进制位注入语法偏差水印
    // Aligning with Claims 2, 6, 8: Fingerprint formula H_chunk = SHA256(v_emb || TextContent || Salt) + watermark injection based on hash bits
    pub fn generate_fingerprint(&self, salt: &[u8]) -> (HashFingerprint, String) {
        let mut hasher = Sha256::new();
        
        // 1. 级联向量矩阵 data
        // 1. Concatenate vector embedding data
        for val in &self.embedding_vector {
            hasher.update(&val.to_be_bytes());
        }
        
        // 2. 级联文本本体内容
        // 2. Concatenate raw text content
        hasher.update(self.text_content.as_bytes());
        
        // 3. 级联本地非对称加密签名盐值 (Salt)
        // 3. Concatenate local asymmetric cryptographic signature salt
        hasher.update(salt);
        
        let calculated_hash: [u8; 32] = hasher.finalize().into();
        
        let start = std::time::SystemTime::now();
        let since_the_epoch = start.duration_since(std::time::UNIX_EPOCH).expect("Time went backwards");
        
        // 4. 基于哈希二进制序列种子，动态微调句式语法或特殊空白符植入偏差水印
        // 4. Dynamically tweak grammar or insert blank spaces based on hash binary seed
        // 对应权利要求6：利用哈希指纹位特征改变切片同义词概率或空白符排布
        // Claim 6: Leverage hash bits to alter synonym probabilities or blank space layout
        let mut watermarked_text = self.text_content.clone();
        let seed = calculated_hash[0];
        
        if seed % 2 == 0 {
        // 在文本特定标点处植入零宽空格 '\u{200B}'
        // Implant Zero-Width Space '\u{200B}' at specific punctuation marks
            if let Some(pos) = watermarked_text.find('。') {
                let byte_idx = pos + '。'.len_utf8();
                watermarked_text.insert_str(byte_idx, "\u{200B}"); 
            } else {
                watermarked_text.push_str("\u{200B}");
            }
            println!("[ 本地切片 ] 成功基于哈希种子植入语法偏差水印 (Code: ZWSP-Align)");
            println!("[ Local Chunk ] Successfully injected syntactic drift watermark based on hash seed (Code: ZWSP-Align)");
        }
        
        let fingerprint = HashFingerprint {
            hash_id: calculated_hash,
            timestamp: since_the_epoch.as_secs(),
        };

        (fingerprint, watermarked_text)
    }
}

impl LocalVectorDb {
    pub fn new() -> Self {
        // 对应权利要求4：向量检索引擎强行绑定于本地 Loopback 地址 127.0.0.1
        // Aligning with Claim 4: Vector search engine strictly bound to local Loopback address 127.0.0.1
        Self {
            listen_address: "127.0.0.1".parse().unwrap(),
            local_ipc_only: true,
            storage: std::collections::HashMap::new(),
        }
    }

    pub fn insert_chunk(&mut self, id_key: String, chunk: LocalTextChunk) {
        self.storage.insert(id_key, chunk);
    }

    // 模拟基于 Loopback 本地绑定的安全检索
    // Simulate secure search based on Loopback-bound local socket
    // 对应权利要求4：物理阻断外部网络对向量库的语义检索，仅接收本地进程IPC请求
    // Aligning with Claim 4: Physically block external network semantic queries, accept local IPC requests only
    pub fn query_semantic(
        &self,
        request_ip: IpAddr,
        query_vector: &[f32],
    ) -> Result<Vec<LocalTextChunk>, &'static str> {
        // 验证请求源 IP 是否为 127.0.0.1 Loopback
        // Verify if request source IP is 127.0.0.1 Loopback
        if request_ip != self.listen_address {
            return Err("ACCESS_DENIED: Direct semantic query from external networks is physically blocked by vector DB firewall.");
        }

        println!("[ 本地向量库 ] 校验通过：请求方来自本地 Loopback 地址 127.0.0.1 (IPC 安全通信通道)。");
        println!("[ Local Vector DB ] Validation success: Request source is local Loopback 127.0.0.1 (IPC secure channel).");
        
        // 模拟执行 Top-K 近邻检索（这里直接返回关联的测试切片明文数据）
        // Simulate Top-K nearest neighbor search (returning matched slice plaintext directly here)
        let mut results = Vec::new();
        for chunk in self.storage.values() {
            // 简单相关度阈值判定
            // Simple relevance threshold check
            if !chunk.embedding_vector.is_empty() && query_vector.len() == chunk.embedding_vector.len() {
                results.push(chunk.clone());
            }
        }
        
        Ok(results)
    }
}

pub mod hex {
    pub fn encode(bytes: &[u8]) -> String {
        const HEX_CHARS: &[u8; 16] = b"0123456789abcdef";
        let mut s = String::with_capacity(bytes.len() * 2);
        for &b in bytes {
            s.push(HEX_CHARS[(b >> 4) as usize] as char);
            s.push(HEX_CHARS[(b & 0xf) as usize] as char);
        }
        s
    }
}
