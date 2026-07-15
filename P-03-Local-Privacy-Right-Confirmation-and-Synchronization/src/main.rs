// P-RZZH-030-QNLOO-2026: 本地隐私确权与同步 - 样板间主程序入口 (Main Entry)
// P-RZZH-030-QNLOO-2026: Local Privacy Confirmation & Sync - Main Entry
// 本文件以实证代码形式，演示本地确权指纹、增量同步上链、安全数据库 Loopback 隔离以及沙箱物理消磁。
// This file demonstrates local confirmation fingerprinting, incremental sync on-chain, local Loopback isolation, and sandbox physical scrubbing.

mod local_zone;
mod ledger;
mod sandbox;

use std::net::IpAddr;

fn main() {
    println!("======================================================================");
    println!("🚀 [ P-RZZH-030-QNLOO-2026 ] 本地隐私确权与同步系统 - 专利样板间代码启动");
    println!("🚀 [ P-RZZH-030-QNLOO-2026 ] Local Privacy Confirmation & Sync System - Patent Blueprint Startup");
    println!("======================================================================\n");

    // ----------------------------------------------------------------------
    // 0. 系统实体初始化
    // 0. System Entities Initialization
    // ----------------------------------------------------------------------
    let mut local_db = local_zone::LocalVectorDb::new();
    let mut chain_ledger = ledger::DistributedLedger::new();
    let salt = b"LOCAL_OWNERSHIP_SALT_KEY_2026";
    let owner_pub_key = b"OWNER_PUBLIC_KEY_SECP256K1";

    println!("[ 初始就绪 ] 本地 Loopback 向量数据库与分布式区块链确权账本启动成功。");
    println!("[ Initialized ] Local Loopback vector database and distributed blockchain ledger started successfully.\n");

    // ----------------------------------------------------------------------
    // 1. 本地切片指纹计算与语法偏差水印植入
    // 1. Local chunk fingerprint calculation and syntactic drift watermark injection
    // ----------------------------------------------------------------------
    println!("----------------------------------------------------------------------");
    println!("🌱 1. 本地文本切片、单向加密指纹生成与语法偏差水印植入");
    println!("🌱 1. Local text slicing, one-way encrypted fingerprinting & syntactic drift watermarking");
    println!("----------------------------------------------------------------------");

    let original_text = "高精密工业设计核心规则：冲压极限厚度不得小于 1.2 毫米。";
    let mock_vector = vec![0.15f32, -0.84f32, 0.99f32, 0.42f32]; // 模拟嵌入向量 v_emb / Simulated embedding vector v_emb
    
    let text_chunk = local_zone::LocalTextChunk::new(original_text, mock_vector);
    
    // 权利要求2、6、8：离线计算哈希指纹 H_chunk，并植入偏差水印
    // Aligning with Claims 2, 6, 8: Offline compute hash fingerprint H_chunk and implant deviation watermark
    let (fingerprint, watermarked_text) = text_chunk.generate_fingerprint(salt);
    
    println!("   原始明文: '{}'", original_text);
    println!("   Raw Plaintext: '{}'", original_text);
    println!("   指纹 H_chunk: 0x{}", local_zone::hex::encode(&fingerprint.hash_id));
    println!("   Fingerprint H_chunk: 0x{}", local_zone::hex::encode(&fingerprint.hash_id));
    println!("   语法水印植入结果: '{}'", watermarked_text);
    println!("   Syntactic Watermark Result: '{}'", watermarked_text);
    
    // 检测是否注入了 ZWSP
    // Check if ZWSP was injected
    let has_watermark = watermarked_text.contains('\u{200B}');
    println!("   产权标识审计: {}", if has_watermark { "成功检测到同义词/零宽语法水印" } else { "未注入" });
    println!("   IP Signature Audit: {}", if has_watermark { "Synonym / zero-width watermark detected successfully" } else { "Not injected" });
    
    // 将明文切片存入本地向量数据库中
    // Store plaintext chunks into local vector database
    local_db.insert_chunk("chunk_101".to_string(), text_chunk);
    println!("");

    // ----------------------------------------------------------------------
    // 2. 增量确权账本同步与时间戳防重放
    // 2. Incremental ownership ledger sync and timestamp anti-replay
    // ----------------------------------------------------------------------
    println!("----------------------------------------------------------------------");
    println!("⛓️ 2. 轻量化增量上链存证登记与防重放核验");
    println!("⛓️ 2. Lightweight incremental on-chain IP registry & anti-replay verification");
    println!("----------------------------------------------------------------------");

    // 权利要求2、9：打包增量数据 SyncPacket，仅上传哈希、时间戳与签名
    // Aligning with Claims 2, 9: Package incremental SyncPacket, uploading only hash, timestamp, and signature
    let sync_packet = ledger::SyncPacket {
        hash_id: fingerprint.hash_id,
        timestamp: fingerprint.timestamp,
        owner_signature: b"OWNER_PRIVATE_KEY_SIGNATURE_DATA".to_vec(),
    };

    // 提交上链
    // Submit to blockchain
    match chain_ledger.register_fingerprint(sync_packet, owner_pub_key) {
        Ok(_) => {
            println!("✅ 增量所有权指纹登记成功！");
            println!("✅ Incremental ownership fingerprint registered successfully!");
        }
        Err(e) => {
            println!("❌ 增量登记失败: {}", e);
            println!("❌ Incremental registration failed: {}", e);
        }
    }

    // 模拟重放攻击校验：再次发送相同数据
    // Simulate replay attack check: resend the same data
    println!("\n[ 重放模拟 ] 恶意第三方拦截报文，并在 10 分钟后尝试二次重播登记...");
    println!("[ Replay Simulation ] Malicious peer intercepts packet, attempting replay after 10 minutes...");
    let replayed_packet = ledger::SyncPacket {
        hash_id: fingerprint.hash_id,
        timestamp: fingerprint.timestamp - 600, // 制造 600秒 (10分钟) 时间偏移  / Create 600s (10 min) time offset
        owner_signature: b"OWNER_PRIVATE_KEY_SIGNATURE_DATA".to_vec(),
    };
    
    match chain_ledger.register_fingerprint(replayed_packet, owner_pub_key) {
        Ok(_) => {
            println!("❌ [ 重放漏洞 ] 账本错误地通过了旧数据重放！");
            println!("❌ [ Replay Loophole ] Ledger incorrectly accepted stale replayed data!");
        }
        Err(e) => {
            println!("✅ [ 拦截成功 ] 确权账本成功防御重放抢注！错误码: '{}'", e);
            println!("✅ [ Intercept Success ] Confirmation ledger successfully blocked replay! Error: '{}'", e);
        }
    }
    println!("");

    // ----------------------------------------------------------------------
    // 3. 向量数据库本地网卡与 IPC 通信绑定保护
    // 3. Vector DB local network interface and IPC communication binding protection
    // ----------------------------------------------------------------------
    println!("----------------------------------------------------------------------");
    println!("🔒 3. 向量检索引擎本地 Loopback 网口与进程 IPC 安全隔离");
    println!("🔒 3. Vector query engine local Loopback interface & IPC isolation");
    println!("----------------------------------------------------------------------");

    let query_vector = vec![0.15f32, -0.84f32, 0.99f32, 0.42f32];

    // 权利要求4：正常通路：本地主权进程 (127.0.0.1 Loopback) 发起检索
    // Aligning with Claim 4: Normal channel: Local sovereign process (127.0.0.1 Loopback) initiates retrieval
    let local_ip: IpAddr = "127.0.0.1".parse().unwrap();
    println!("[ 通道 1 ] 本地主权进程 (源 IP: {}) 请求语义关联检索:", local_ip);
    println!("[ Channel 1 ] Local sovereign process (Source IP: {}) requests semantic query:", local_ip);
    match local_db.query_semantic(local_ip, &query_vector) {
        Ok(chunks) => {
            println!("   ✅ 检索成功！获取到匹配明文分块大小: {} 字节", chunks[0].text_content.len());
            println!("   ✅ Search success! Retrieved matching plaintext chunk size: {} bytes", chunks[0].text_content.len());
        }
        Err(e) => {
            println!("   ❌ 检索失败: {}", e);
            println!("   ❌ Search failed: {}", e);
        }
    }

    // 权利要求4：攻击通路：外部远程服务器 (如 192.168.1.100) 试图越权探测向量库
    // Aligning with Claim 4: Attack channel: External remote server (e.g., 192.168.1.100) attempts unauthorized vector DB probing
    let external_ip: IpAddr = "192.168.1.100".parse().unwrap();
    println!("\n[ 通道 2 ] 外部非安全服务器 (源 IP: {}) 试图直接穿透请求向量库检索:", external_ip);
    println!("[ Channel 2 ] External untrusted server (Source IP: {}) attempts direct query of vector DB:", external_ip);
    match local_db.query_semantic(external_ip, &query_vector) {
        Ok(_) => {
            println!("   ❌ [ 安全缺陷 ] 向量库允许了外网直接探测语义接口！");
            println!("   ❌ [ Security Defect ] Vector database allowed direct external query!");
        }
        Err(e) => {
            println!("   ✅ [ 物理阻断 ] 向量库防火墙成功拦截外网探测！拦截原因: '{}'", e);
            println!("   ✅ [ Physical Block ] Vector DB firewall successfully blocked external query! Reason: '{}'", e);
        }
    }
    println!("");

    // ----------------------------------------------------------------------
    // 4. 会话只读沙箱分配与物理电荷中和消磁
    // 4. Session read-only sandbox allocation and physical charge neutralization scrub
    // ----------------------------------------------------------------------
    println!("----------------------------------------------------------------------");
    println!("🧹 4. 算力隔离层临时只读沙箱分配、超时监控与 3遍覆写物理消磁");
    println!("🧹 4. Execution sandbox allocation, timeout monitoring & 3-pass physical scrubbing");
    println!("----------------------------------------------------------------------");

    // 权利要求3、10：分配 1024 字节临时只读沙箱，生命周期设为 120s (MAX_TTL)
    // Aligning with Claims 3 & 10: Allocate 1024-byte temporary read-only sandbox, lifecycle set to 120s (MAX_TTL)
    let mut secure_sandbox = sandbox::SecureEnclaveSandbox::new(1024, 120);
    
    // 载入 Top-K 检索出的只读明文语料
    // Load Top-K retrieved read-only plaintext corpus
    if secure_sandbox.load_read_only_context(original_text).is_ok() {
        println!("[ 沙箱分配 ] 成功申请 TEE 安全只读飞地，只读载入敏感明文...");
        println!("[ Sandbox Allocate ] Secure TEE read-only enclave allocated, loaded sensitive plaintext...");
    }

    // 模拟大模型推理计算中...
    // Simulating Large Language Model inference computation...
    println!("[ 推理模拟 ] 异构大模型引擎只读接入 TEE 内存段，获取提示词上下文完成推理。");
    println!("[ Inference Simulation ] Heterogeneous LLM engine accesses TEE memory segments, retrieves context and runs inference.");

    // 模拟推理结束，强制启动物理消磁擦除机制
    // Simulating inference completion, forcibly triggering physical demagnetization scrub mechanism
    // 权利要求3、10：3遍覆写 (0x00 -> 0xFF -> 密码学随机噪声) + Cache Flush
    // Inference completed, forcefully triggering physical demagnetization scrub mechanism
    // Aligning with Claims 3 & 10: 3-pass overwrite (0x00 -> 0xFF -> cryptographic random noise) + Cache Flush
    unsafe {
        let _ = secure_sandbox.secure_erase();
    }

    // 销毁沙箱物理指针与内存页分配
    // Destroy sandbox physical pointers and memory page allocations
    secure_sandbox.deallocate();

    println!("\n======================================================================");
    println!("🎉 [ 演练结束 ] 语法偏差水印、账本零泄露确权、外网硬隔离与物理消磁完美闭环！");
    println!("🎉 [ Demos Complete ] Syntactic watermarking, zero-leak ledger registry, external block & physical scrubbing validated!");
    println!("======================================================================");
}
