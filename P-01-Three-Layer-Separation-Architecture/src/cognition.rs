// P-RZZH-010-QNLOO-2026: 三层分离系统架构 - 规则认知层 (Cognition Layer, Ring 0)
// P-RZZH-010-QNLOO-2026: Three-Layer Separation System - Rule Cognition Layer (Ring 0)
// 物理安全机制：分配临时飞地内存 + 高精度 Timer_TTL 监控 + 3遍电荷中和消磁 + CPU 缓存清刷
// Physical Security: Allocating temporary enclave memory + precision Timer_TTL + 3-pass cache scrubbing + CPU cache flushing

use std::ptr;
use std::time::{SystemTime, UNIX_EPOCH, Instant};
use rand::RngCore;
use hmac::{Hmac, Mac};
use sha2::Sha256;

use crate::asset::EncryptedKnowledgeStore;
use crate::execution::ExecutionEngine;

// Aligning with Timer_TTL = 120s
// 对应 Timer_TTL = 120s
pub const MAX_TTL_SECONDS: u64 = 120; 
// Aligning with Theta_max
// 对应 Theta_max
pub const MAX_TOKEN_THRESHOLD: usize = 4096; 

#[derive(Debug)]
pub enum SecurityError {
    SessionTimeout,
    TokenOverflow,
    UnauthorizedLeak,
    ScrubFailure,
    AccessDenied,
}

pub struct SessionContext {
    pub session_id: [u8; 32],
    pub session_key: [u8; 32],
    pub memory_base_address: *mut u8,
    pub memory_size: usize,
    pub bytes_written: usize,
    pub start_time: Instant,
}

impl SessionContext {
    pub fn new(session_id: [u8; 32], session_key: [u8; 32], size: usize) -> Self {
        // 在堆上动态分配一段物理内存，模拟 TEE 飞地沙箱的临时安全存储段
        // Dynamically allocate memory on heap to simulate temporary secure storage of TEE enclave sandbox
        let layout = std::alloc::Layout::from_size_align(size, 64).unwrap();
        let ptr = unsafe { std::alloc::alloc_zeroed(layout) };

        Self {
            session_id,
            session_key,
            memory_base_address: ptr,
            memory_size: size,
            bytes_written: 0,
            start_time: Instant::now(),
        }
    }

    // 释放内存资源
    // Deallocate memory resources
    pub fn deallocate(&mut self) {
        if !self.memory_base_address.is_null() {
            let layout = std::alloc::Layout::from_size_align(self.memory_size, 64).unwrap();
            unsafe {
                std::alloc::dealloc(self.memory_base_address, layout);
                self.memory_base_address = ptr::null_mut();
            }
        }
    }

    // 执行物理内存强力擦除覆写，规避冷启动侧信道攻击
    // Execute physical memory overwrite scrubbing to mitigate cold boot side-channel attacks
    // 对应权利要求3：不少于三遍物理覆写循环 (0x00 -> 0xFF -> 随机数) + L1/L2 Cache 刷新 (_mm_clflush / ARM 屏障)
    // Aligning with Claim 3: No less than 3 physical overwrite loops (0x00 -> 0xFF -> random) + L1/L2 Cache flushes (_mm_clflush / ARM barrier)
    pub unsafe fn physical_scrub(&mut self) -> Result<(), SecurityError> {
        if self.memory_base_address.is_null() {
            return Err(SecurityError::ScrubFailure);
        }

        println!("[ Ring 0 规则认知层 ] 开始执行 3 遍物理内存强力覆写消磁程序...");
        println!("[ Ring 0 Rule Cognition Layer ] Initiating 3-pass physical memory overwrite scrubbing & demagnetization...");

        let slice = std::slice::from_raw_parts_mut(self.memory_base_address, self.memory_size);

        // Pass 1: 写入全 0x00 (使用 volatile 写入以防止编译器优化消除)
        // Pass 1: Write all 0x00 (using volatile writes to prevent compiler optimization bypass)
        for i in 0..self.memory_size {
            ptr::write_volatile(self.memory_base_address.add(i), 0x00);
        }
        
        // Pass 2: 写入全 0xFF
        // Pass 2: Write all 0xFF
        for i in 0..self.memory_size {
            ptr::write_volatile(self.memory_base_address.add(i), 0xFF);
        }
        
        // Pass 3: 写入强密码学随机噪声，打破硅片残存势能
        // Pass 3: Write secure cryptographic random noise to break residual charge states
        let mut rng = rand::thread_rng();
        rng.fill_bytes(slice);
        for i in 0..self.memory_size {
            ptr::write_volatile(self.memory_base_address.add(i), slice[i]);
        }

        // 刷新各级 CPU 缓存，强制将数据刷回物理 DRAM 单元，实现电荷中和
        // Flush CPU cache hierarchies to force data back to DRAM for charge neutralization
        // 针对不同目标架构（Intel/AMD 对应 x86_64，Apple Silicon / 华为鲲鹏 对应 ARM/Aarch64）进行兼容性编译
        // Cross-compile cache flush instructions for target architectures (x86_64, aarch64)
        for offset in (0..self.memory_size).step_by(64) {
            let addr = self.memory_base_address.add(offset);
            flush_cache_line(addr);
        }

        println!("[ Ring 0 规则认知层 ] 物理内存擦除完成！已清刷 CPU Cache 行，DRAM 电荷彻底消磁中和。");
        println!("[ Ring 0 Rule Cognition Layer ] Physical memory scrub complete! CPU Cache lines flushed, DRAM charge neutralized.");
        Ok(())
    }

    // 模拟边界穿透（如越权内存访问、侧信道溢出等异常场景）强制触发消磁
    // Simulate boundary penetration (e.g. page fault eBPF detection of unauthorized access) to force degaussing
    pub unsafe fn simulate_boundary_penetration(&mut self) -> Result<(), SecurityError> {
        println!("[ Ring 0 规则认知层 ] ⚠️ 警报：检测到边界被越权穿透！强制执行安全消磁中断！");
        println!("[ Ring 0 Rule Cognition Layer ] ⚠️ ALERT: Boundary penetration detected! Forcing emergency degaussing interrupt!");
        self.physical_scrub()
    }
}

// 为 SessionContext 实现 RAII 自动物理消磁，防止异常 panic 泄密
// Implement RAII auto-scrubbing for SessionContext to block leakage on unexpected panic
impl Drop for SessionContext {
    fn drop(&mut self) {
        if !self.memory_base_address.is_null() {
            unsafe {
                let _ = self.physical_scrub();
                let layout = std::alloc::Layout::from_size_align(self.memory_size, 64).unwrap();
                std::alloc::dealloc(self.memory_base_address, layout);
                self.memory_base_address = ptr::null_mut();
            }
        }
    }
}

// 跨架构兼容性编译缓存刷回指令
// Cross-architecture compatible cache flush commands
#[cfg(target_arch = "x86_64")]
#[inline(always)]
unsafe fn flush_cache_line(addr: *mut u8) {
    std::arch::x86_64::_mm_clflush(addr);
}

#[cfg(target_arch = "aarch64")]
#[inline(always)]
unsafe fn flush_cache_line(addr: *mut u8) {
    // 使用 ARM64 汇编：dc cvac (Data Cache Clean by Virtual Address to PoC)
    // Clean data cache line by virtual address to Point of Coherency (PoC)
    // 配合 dsb sy (Data Synchronization Barrier System) 强迫 L1/L2 缓存行内电荷沉降回物理 DRAM
    // Data synchronization barrier system to force L1/L2 charge settlement to physical DRAM
    std::arch::asm!(
        "dc cvac, {0}",
        "dsb sy",
        in(reg) addr,
        options(nostack, preserves_flags)
    );
}

#[cfg(not(any(target_arch = "x86_64", target_arch = "aarch64")))]
#[inline(always)]
unsafe fn flush_cache_line(_addr: *mut u8) {
    // 兜底策略：使用编译器屏障
    // Fallback policy: compiler fence barrier
    std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
}

    // 计算控制流指令的操作码逻辑熵 (Shannon Entropy)
    // Compute Shannon Entropy of control flow instruction payload
    // 公式: H_logic = - \sum P(o_i) log2 P(o_i)
    // Formula: H_logic = - \sum P(o_i) log2 P(o_i)
    // 对应权利要求4：逻辑熵计算公式实现
    // Aligning with Claim 4: Logical entropy calculation formula implementation
pub fn calculate_logic_entropy(payload: &[u8]) -> f64 {
    if payload.is_empty() {
        return 0.0;
    }

    let mut counts = [0u32; 256];
    for &byte in payload {
        counts[byte as usize] += 1;
    }

    let total = payload.len() as f64;
    let mut entropy = 0.0;
    for &count in counts.iter() {
        if count > 0 {
            let p = count as f64 / total;
            entropy -= p * p.log2();
        }
    }

    entropy
}

    // 因果序时间戳哈希签名，为指令流生成独一无二的“物理出生证明”
    // Causal timestamp hash signature to generate unique physical birth certificate for instruction stream
    // 公式: OutputHash = HMAC-SHA256(T_μs || H_logic, K_session)
    // Formula: OutputHash = HMAC-SHA256(T_μs || H_logic, K_session)
    // 对应权利要求4：HMAC签名生成
    // Aligning with Claim 4: HMAC signature generation
pub fn generate_causal_output_hash(
    payload: &[u8],
    session_key: &[u8; 32],
    logic_entropy: f64,
    t_micros: u64,
) -> [u8; 32] {
    let mut mac = Hmac::<Sha256>::new_from_slice(session_key).unwrap();
    
    // 注入物理时间维度特征 T_μs
    // Inject physical time dimension feature T_μs
    mac.update(&t_micros.to_le_bytes());
    
    // 注入控制指令本身的逻辑度量特征 H_logic
    // Inject logic metric feature H_logic of instructions
    let entropy_bytes = logic_entropy.to_bits();
    mac.update(&entropy_bytes.to_le_bytes());
    
    // 注入控制流数据载荷本体
    // Inject control flow data payload body
    mac.update(payload);
    
    mac.finalize().into_bytes().into()
}

// 调度协同控制引擎
// Orchestration and Coordination Control Engine
// 对应权利要求1、权利要求8：接收请求、申请飞地、Timer_TTL 启动、单向只读读取、聚合熵计算、HMAC指纹、消磁擦除
// Aligning with Claim 1 & 8: Receive requests, apply enclave, start Timer_TTL, read tunnel, calculate entropy, HMAC fingerprint, cache scrub
pub unsafe fn run_cortex_inference_session(
    session: &mut SessionContext,
    asset_store: &EncryptedKnowledgeStore,
    execution_engine: &ExecutionEngine,
    cortex_token: &[u8; 32],
    asset_key: &str,
    value_protocol_rules: &str,
) -> Result<Vec<u8>, SecurityError> {
    
    // 1. 检查会话生存周期是否已经超时 (Timer_TTL 倒计时)
    // Check if session TTL has expired (Timer_TTL countdown)
    if session.start_time.elapsed().as_secs() > MAX_TTL_SECONDS {
        session.physical_scrub()?;
        return Err(SecurityError::SessionTimeout);
    }

    // 2. 通过只读逻辑隧道从 Ring -1 资产层拉取私有数据切片
    // Pull private data slices from Ring -1 Asset Layer via read-only logic tunnel
    let mut raw_asset_data = match asset_store.request_read_tunnel(cortex_token, asset_key) {
        Ok(data) => data,
        Err(_) => return Err(SecurityError::AccessDenied),
    };

    // 3. 拦截语义暴食攻击 (Semantic Binge Attack / Over-ingestion Attack) (检查局部读入大小是否超过上限)
    // Intercept Semantic Binge Attack (Over-ingestion Attack) (check read size against threshold)
    if raw_asset_data.len() > MAX_TOKEN_THRESHOLD {
        session.physical_scrub()?;
        return Err(SecurityError::TokenOverflow);
    }

    // 4. 将读取到的数据安全复制入分配的 TEE 安全沙箱内存中 (Ring 0 Zone)
    // Securely copy retrieved data into allocated TEE sandbox memory (Ring 0 Zone)
    ptr::copy_nonoverlapping(
        raw_asset_data.as_ptr(),
        session.memory_base_address,
        raw_asset_data.len(),
    );
    session.bytes_written = raw_asset_data.len();

    // 4.1. 物理回收隔离只读隧道的临时映射段内存，完成两层之间的数据中和消退
    // Physically neutralize and recycle local mapping memory of read-only logic tunnel
    asset_store.recycle_tunnel_memory(&mut raw_asset_data);

    // 5. 按照“对等价值协议”编译拼装执行指令包 (在隔离运行域内聚合计算)
    // Compile & assemble execution instructions based on "Value Protocol"
    let memory_slice = std::slice::from_raw_parts(session.memory_base_address, session.bytes_written);
    let compiled_instruction_stream = format!(
        "[VALUE_PROTOCOL_RULES]: {}\n[脫敏知识语料切片]: {:?}",
        value_protocol_rules,
        memory_slice
    ).into_bytes();

    // 6. 语义与指令特征量化：计算逻辑熵 H_logic
    // Quantify semantic & instruction features: compute logical entropy H_logic
    let logic_entropy = calculate_logic_entropy(&compiled_instruction_stream);

    // 7. 生成指令出生证明与因果防重放指纹 (微秒时间戳 T_μs + HMAC)
    // Generate birth certificate & causal anti-replay fingerprint (Microsecond timestamp T_μs + HMAC)
    let t_micros = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_micros() as u64;

    let anti_replay_fingerprint = generate_causal_output_hash(
        &compiled_instruction_stream,
        &session.session_key,
        logic_entropy,
        t_micros
    );

    println!(
        "[ Ring 0 规则认知层 ] 构建出目标控制流指令包 (大小: {} 字节)，生成指令出生证明: HMAC-SHA256 [ T: {} μs ]",
        compiled_instruction_stream.len(),
        t_micros
    );
    println!(
        "[ Ring 0 Rule Cognition Layer ] Target control flow instruction stream compiled (size: {} bytes). Birth certificate generated: HMAC-SHA256 [ T: {} us ]",
        compiled_instruction_stream.len(),
        t_micros
    );

    // 8. 单向投递给 Ring 3 算力层进行模型矩阵运算
    // One-way dispatch to Ring 3 Execution Layer for model matrix computation
    let inference_result = match execution_engine.receive_and_execute(
        &session.session_id,
        &compiled_instruction_stream,
        &anti_replay_fingerprint,
        t_micros,
        logic_entropy
    ) {
        Ok(res) => res,
        Err(e) => {
            eprintln!("❌ [ Ring 0 规则认知层 ] 算力层返回严重异常，中断会话现场！异常原因: {}", e);
            eprintln!("❌ [ Ring 0 Rule Cognition Layer ] Execution Layer returned fatal exception. Terminating session! Reason: {}", e);
            session.physical_scrub()?;
            return Err(SecurityError::UnauthorizedLeak);
        }
    };

    // 9. 任务正常结束，在跳出函数前强制执行硬件消磁覆写擦除
    // Task finished normally. Force hardware memory scrubbing before returning
    session.physical_scrub()?;

    // 10. 验证再次检测 Timer_TTL 超时情况
    // Check Timer_TTL timeout once more before exit
    if session.start_time.elapsed().as_secs() > MAX_TTL_SECONDS {
        return Err(SecurityError::SessionTimeout);
    }

    Ok(inference_result)
}
