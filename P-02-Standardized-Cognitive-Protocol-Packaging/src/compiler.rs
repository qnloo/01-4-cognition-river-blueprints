// P-RZZH-020-QNLOO-2026: 标准化认知协议封装 - 元认知协议编译器 (Cognitive Compiler Module)
// P-RZZH-020-QNLOO-2026: Standardized Cognitive Protocol Packaging - Cognitive Compiler Module
// 物理安全机制：异构算力引擎特征探测 + 参数槽强类型绑定 + 隐蔽数字水印注入
// Physical Security: Heterogeneous compute engine feature detection + strong-typed parameter slot binding + covert digital watermark injection

use crate::container::{ValueProtocolContainer, ProtocolError};

pub struct CognitiveCompiler {
    // 模拟内置的参数敏感度/注意力权重映射矩阵
    // Simulate built-in parameter sensitivity / attention weight mapping matrix
    // 定义不同大模型引擎对系统指令 (System Prompt) 的偏好与敏感等级
    // Define preference & sensitivity of different LLM engines to System Prompt
    engine_sensitivity_matrix: std::collections::HashMap<String, u8>,
}

impl CognitiveCompiler {
    pub fn new() -> Self {
        let mut matrix = std::collections::HashMap::new();
        // 1: 高度 System 敏感型，2: 中度敏感型，3: 弱系统敏感型（通常是端侧轻量化模型）
        // 1: Highly System sensitive, 2: Moderately sensitive, 3: Weak System sensitive (usually edge lightweight models)
        matrix.insert("GPT-4".to_string(), 1);
        matrix.insert("DeepSeek-V3".to_string(), 1);
        matrix.insert("EndSide-Model-Tiny".to_string(), 3);
        
        Self {
            engine_sensitivity_matrix: matrix,
        }
    }

    // 注入隐蔽水印特征
    // Inject covert watermark features
    // 对应权利要求4、9：在特定位置注入零宽间隔符号等隐蔽水印，使外部能够采样验证确权
    // Aligning with Claims 4 & 9: Inject covert watermark symbols at specific positions to enable external sampling & verification
    pub fn inject_watermark(&self, text: &str, container_id: &[u8; 32]) -> String {
        let mut result = text.to_string();
        
        // 解析 container_id 的前 4 位字节作为水印特征码
        // Parse first 4 bytes of container_id as watermark feature code
        let mark_val = container_id[0] ^ container_id[1];
        let watermark_char = if mark_val % 2 == 0 {
            '\u{200B}' // 零宽空格 Zero-Width Space (ZWSP)
        } else {
            '\u{200C}' // 零宽不连字 Zero-Width Non-Joiner (ZWNJ)
        };
        
        // 在首部注入
        // Inject at head
        result.insert(0, watermark_char);
        
        // 在中英文常见标点符号后面插入
        // Insert after common Chinese/English punctuation
        let mut char_indices = result.char_indices().collect::<Vec<_>>();
        let mut insert_indices = Vec::new();
        for (i, &(_, c)) in char_indices.iter().enumerate() {
            if c == '：' || c == ':' || c == '。' || c == ',' || c == '，' {
                // 记录需要在该字符后一个位置插入（即 i + 1 的字符对应的字节索引）
                if i + 1 < char_indices.len() {
                    insert_indices.push(char_indices[i + 1].0);
                } else {
                    insert_indices.push(result.len());
                }
                break; // 仅在首个标点后注入以防止过多干扰
            }
        }
        
        // 执行插入（由于只找了首个标点，只有一个插入点）
        let mut offset = 0;
        for idx in insert_indices {
            result.insert(idx + offset, watermark_char);
            offset += watermark_char.len_utf8();
        }
        
        // 在尾部注入
        // Inject at tail
        result.push(watermark_char);
        
        if mark_val % 2 == 0 {
            println!("[ 编译器水印 ] 注入多点产权水印标识: 零宽空格 (Code A)");
            println!("[ Compiler Watermark ] Injected multi-point property watermark: Zero-Width Space (Code A)");
        } else {
            println!("[ 编译器水印 ] 注入多点产权水印标识: 零宽不连字 (Code B)");
            println!("[ Compiler Watermark ] Injected multi-point property watermark: Zero-Width Non-Joiner (Code B)");
        }
        
        result
    }

    // 针对异构算力引擎的自适应转译与指令组装
    // Adaptive translation & instruction assembly for heterogeneous compute engines
    // 对应权利要求4、权利要求6、7、9：校验级联哈希、强类型参数插值、注入水印、自适应调整系统/用户输入组装顺序
    // Aligning with Claims 4, 6, 7, 9: Verify cascading hash, strong-typed parameter interpolation, watermark injection, adaptive assembly order
    pub fn compile(
        &self,
        container: &ValueProtocolContainer,
        engine_provider: &str,
        user_raw_input: &str,
    ) -> Result<String, ProtocolError> {
        
        // 1. 优先校验级联哈希完整性、数字签名及生命周期 (安全前置条件)
        // 1. Verify cascading hash integrity, digital signature, and lifecycle first (security prerequisite)
        container.verify_cascading_hash()?;
        
        // 验证非对称数字签名 (权利要求3)
        // Verify asymmetric digital signature (Claim 3)
        let mock_public_key = vec![0xCC, 0xDD, 0xEE, 0xFF];
        container.verify_signature(&mock_public_key)?;

        // 验证生命周期 (权利要求3)
        // Verify lifecycle (Claim 3)
        container.verify_lifecycle(50)?; // 假设当前会话已过 50 秒  Assume 50s have elapsed

        // 2. 解析专属口诀中的参数占位符并强类型绑定填充 (Parameter Slot Mapping)
        // 2. Parse instruction parameter placeholders with strong-typed slot binding
        let mut processed_template = container.instructions.rule_template.clone();
        for (slot, value) in &container.instructions.parameter_slots {
            let placeholder = format!("{{{}}}", slot);
            processed_template = processed_template.replace(&placeholder, value);
        }

        // 3. 动态注入隐蔽数字水印
        // 3. Dynamically inject covert digital watermark
        let watermarked_template = self.inject_watermark(&processed_template, &container.header.id);

        // 4. 根据敏感度矩阵调整系统角色指令的组装顺序与格式
        // 4. Adjust order & format of system instructions based on sensitivity matrix
        let sensitivity = self.engine_sensitivity_matrix.get(engine_provider).copied().unwrap_or(2);
        
        let compiled_output = match sensitivity {
            1 => {
                // 高度 System 敏感型（如 GPT-4
                // DeepSeek）：
                // 采用标准系统预设隔离插值，将控制断言和口诀作为最高权重 System Message 发出
                // Use standard system preset isolated interpolation, emitting control assertions and mantras as top-weight System Message
                println!("[ 编译器转译 ] 识别到高端引擎 '{}', 采用头部系统级预设组装模式", engine_provider);
                println!("[ Compiler Compile ] Identified high-end engine '{}', adopting header system-level preset assembly mode", engine_provider);
                format!(
                    "--- SYSTEM CONSTRAIN RING 0 ---\n{}\n--- PSQA ASSERTIONS ---\nMaxEntropy Limit: {:.4}\n---\nUSER MESSAGE: {}",
                    watermarked_template,
                    container.assertions.max_entropy,
                    user_raw_input
                )
            }
            3 => {
                // 弱 System 敏感端侧轻量模型（如 EndSide-Model-Tiny）：
                // Weak System sensitive edge lightweight models (e.g. EndSide-Model-Tiny):
                // System 权重较低，易被用户 prompt 覆盖。自适应重构指令流，将专属口诀编译封装于 XML 标签内，放置在交互最尾端强力约束
                // Weak System sensitive edge lightweight models (e.g. EndSide-Model-Tiny):
                // System weight is low, easily overwritten by user prompts. Adaptively reconstruct stream to wrap mantras in XML tags, placed at the very tail for strong constraint
                println!("[ 编译器转译 ] 识别到端侧引擎 '{}', 采用尾部 XML 结构化强制指令嵌入模式", engine_provider);
                println!("[ Compiler Compile ] Identified edge engine '{}', adopting trailing XML structured directive embedding mode", engine_provider);
                format!(
                    "<user_raw_input>\n{}\n</user_query>\n<system_constrain>\n{}\nPSQA_Limit: {}\n</system_constrain>",
                    user_raw_input,
                    watermarked_template,
                    container.assertions.max_entropy
                )
            }
            _ => {
                // 默认普通大模型引擎：通用前后缀拼接
                // Default regular LLM engine: generic prefix/suffix stitching
                println!("[ 编译器转译 ] 识别到通用引擎 '{}', 采用混合拼装模式", engine_provider);
                println!("[ Compiler Compile ] Identified generic engine '{}', adopting hybrid assembly mode", engine_provider);
                format!(
                    "INSTRUCTIONS:\n{}\nUSER INPUT:\n{}",
                    watermarked_template,
                    user_raw_input
                )
            }
        };

        Ok(compiled_output)
    }
}
