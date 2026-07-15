// P-RZZH-020-QNLOO-2026: 标准化认知协议封装 - 出站自洽断言拦截器 (Outbound Interceptor Module)
// P-RZZH-020-QNLOO-2026: Standardized Cognitive Protocol Packaging - Outbound Interceptor Module
// 物理安全机制：实时逻辑熵计算 + 违禁语义过滤 + 幻觉熔断阻断
// Physical Security: Real-time logical entropy calculation + forbidden semantic filtering + hallucination circuit-breaker

use crate::container::ValueProtocolContainer;

pub struct OutboundAssertionInterceptor;

impl OutboundAssertionInterceptor {
    pub fn new() -> Self {
        Self
    }

    // 基于词频分布计算原始文本流的实时 Shannon 逻辑熵 H
    // Compute real-time Shannon logical entropy H of raw text stream
    // 对应权利要求5、8：基于原始输出流内词汇的生成概率计算逻辑熵
    // Aligning with Claims 5 & 8: Compute logical entropy based on generation probability of terms in the raw stream
    pub fn calculate_entropy(&self, text: &str) -> f64 {
        if text.is_empty() {
            return 0.0;
        }

        // 按字符(Char)粒度进行概率分布量化，完美适配多语种认知矩阵，防止 UTF-8 多字节中文被误杀
        // Quantize char-level probability distribution to adapt to multi-lingual matrices
        let mut char_counts = std::collections::HashMap::new();
        for c in text.chars() {
            *char_counts.entry(c).or_insert(0u32) += 1;
        }

        let total = text.chars().count() as f64;
        let mut entropy = 0.0;
        for &count in char_counts.values() {
            if count > 0 {
                let p = count as f64 / total;
                entropy -= p * p.log2();
            }
        }

        entropy
    }

    // 计算词级 (Word-level) 熵，针对西文字符串以空格分词，针对中文字符串以字分词
    // Compute word-level entropy. Words are split by spaces for Western texts, and by characters for Chinese.
    // 对应权利要求5、8：支持词级与字符级自适应熵校验，提高对语义分布概率判定的准确度
    // Aligning with Claims 5 & 8: Support adaptive word-level and character-level entropy verification to improve accuracy of semantic distribution probability determination
    pub fn calculate_word_entropy(&self, text: &str) -> f64 {
        if text.is_empty() {
            return 0.0;
        }

        // 检测文本中是否有较多空格以判定是否是西文
        // Detect if the text contains spaces to decide word segmentation strategy
        let has_spaces = text.contains(' ') || text.contains('\n');
        
        let words: Vec<String> = if has_spaces {
            // 西文分词：按空格、标点分词
            text.split(|c: char| c.is_whitespace() || c.is_ascii_punctuation())
                .filter(|s| !s.is_empty())
                .map(|s| s.to_lowercase())
                .collect()
        } else {
            // 中文分词：在没有空格的简单场景下，以字符(Char)作为基本词汇单元
            text.chars().map(|c| c.to_string()).collect()
        };

        if words.is_empty() {
            return 0.0;
        }

        let mut word_counts = std::collections::HashMap::new();
        for w in &words {
            *word_counts.entry(w.clone()).or_insert(0u32) += 1;
        }

        let total = words.len() as f64;
        let mut entropy = 0.0;
        for &count in word_counts.values() {
            if count > 0 {
                let p = count as f64 / total;
                entropy -= p * p.log2();
            }
        }

        entropy
    }

    // 执行出站审计拦截
    // Execute outbound interception and audit
    // 对应权利要求1、权利要求5、权利要求8：拦截原始文本、校验熵与违禁词、允许放行或通信异常熔断
    // Aligning with Claim 1, 5, 8: Intercept raw output, verify entropy & forbidden tokens, allow release or trigger communication circuit-break
    pub fn intercept_and_audit(
        &self,
        container: &ValueProtocolContainer,
        raw_output_text: &str,
    ) -> Result<(), &'static str> {
        println!("[ 拦截审计 ] 开始执行出站自洽性断言谓词审计...");
        println!("[ Interceptor Audit ] Initiating outbound self-consistency assertion predicate audit...");

        // 1. 检验违禁敏感词表 (Forbidden Tokens)
        // 1. Verify forbidden sensitive tokens
        // 权利要求5：禁忌语义词表校验
        // Claim 5: Taboo semantic wordlist verification
        for forbidden in &container.assertions.forbidden_tokens {
            if raw_output_text.contains(forbidden) {
                eprintln!("🚨 [ 熔断 ] 检测到违禁敏感词落地: '{}'！触发断言校验失败，阻断输出流！", forbidden);
                eprintln!("🚨 [ Circuit Break ] Forbidden sensitive token detected: '{}'! Assertion check failed, blocking output stream!", forbidden);
                return Err("ASSERTION_FAILED: Output contains forbidden token. Connection aborted.");
            }
        }

        // 2. 检查输出实时逻辑熵是否超出阈值限制 H_max
        // 2. Check if output real-time entropy exceeds H_max threshold
        // 权利要求8：计算输出实时逻辑熵并校验 H <= H_max。大模型发生严重幻觉（生成无规律乱码）时，逻辑熵会异常飙高
        // 支持字符级与词汇级双重熵指标审计 (Claim 8)
        // Claim 8: Calculate real-time logical entropy & verify H <= H_max. Under extreme hallucination, entropy spikes abnormally.
        // Support both character-level and word-level entropy auditing.
        let char_entropy = self.calculate_entropy(raw_output_text);
        let word_entropy = self.calculate_word_entropy(raw_output_text);
        let max_allowed_entropy = container.assertions.max_entropy;

        println!(
            "[ 拦截审计 ] 输出字符逻辑熵: {:.4}, 词级逻辑熵: {:.4} (许可上限 H_max: {:.4})",
            char_entropy,
            word_entropy,
            max_allowed_entropy
        );
        println!(
            "[ Interceptor Audit ] Output char entropy: {:.4}, word entropy: {:.4} (Max allowed H_max: {:.4})",
            char_entropy,
            word_entropy,
            max_allowed_entropy
        );

        if char_entropy > max_allowed_entropy || word_entropy > max_allowed_entropy {
            eprintln!("🚨 [ 熔断 ] 实时逻辑熵超标 (H > H_max)！大模型极有可能发生黑盒幻觉或乱码。");
            eprintln!("🚨 [ Circuit Break ] Real-time logical entropy exceeds limit (H > H_max)! AI model hallucination or corruption suspected.");
            return Err("ASSERTION_FAILED: Real-time entropy exceeds H_max threshold. AI output is incoherent.");
        }

        // 3. 通关通过
        // 3. Audit Clear
        println!("✅ [ 审计放行 ] 出站自洽性断言通过，判定执行流物理真诚，允许数据流向用户解密解构。");
        println!("✅ [ Audit Release ] Outbound self-consistency assertions passed. Output deemed physically sincere, allowing release.");
        Ok(())
    }
}
