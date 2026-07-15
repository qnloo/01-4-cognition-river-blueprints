// P-RZZH-020-QNLOO-2026: 标准化认知协议封装 - 样板间主程序入口 (Main Entry)
// P-RZZH-020-QNLOO-2026: Standardized Cognitive Protocol Packaging - Main Entry
// 本文件以实证代码形式，演示专利所声明的容器结构、代际哈希链继承、自适应转译及出站断言拦截机制。
// This file demonstrates the container structure, generational hash chain lineage, adaptive translation, and outbound assertion interception.

mod container;
mod compiler;
mod interceptor;

use std::collections::BTreeMap;

fn main() {
    println!("======================================================================");
    println!("🚀 [ P-RZZH-020-QNLOO-2026 ] 标准化认知协议封装系统 - 专利样板间代码启动");
    println!("🚀 [ P-RZZH-020-QNLOO-2026 ] Standardized Cognitive Protocol Packaging System - Patent Blueprint Startup");
    println!("======================================================================\n");

    // ----------------------------------------------------------------------
    // 1. 初始化初代根协议容器 (元认知协议根节点)
    // 1. Initialize genesis root protocol container (Meta-cognitive protocol root node)
    // ----------------------------------------------------------------------
    println!("----------------------------------------------------------------------");
    println!("🌱 1. 初始化根协议容器 (Root Protocol Container)");
    println!("🌱 1. Initializing Root Protocol Container");
    println!("----------------------------------------------------------------------");

    let mut parent_slots = BTreeMap::new();
    parent_slots.insert("开发者所有权域名".to_string(), "logicactor.com".to_string());
    
    // 权利要求2：初代协议，父代哈希指纹强制填充为 32 字节全零值
    // Aligning with Claim 2: Genesis protocol, parent hash fingerprint forcefully zero-padded to 32 bytes
    let root_container = container::ValueProtocolContainer::new(
        None, // 初代没有父代哈希  Genesis has no parent hash
        "元认知标准：认知守护者 {开发者所有权域名} 确立资产安全边界。".to_string(),
        parent_slots,
        4.0, // 最大逻辑熵 H_max 设为 4.0  Max logic entropy H_max set to 4.0
        vec!["思维监视".to_string(), "隐私监控".to_string()], // 禁忌语义词表  Taboo semantic wordlist
        0.95,
        120, // TTL
        vec![0x90, 0xAB, 0xCD, 0xEF], // 模拟开发者非对称私钥签名  Simulated developer asymmetric private key signature
    );

    // 验证根协议的级联哈希、数字签名和 TTL 生命周期
    // Verify cascading hash, digital signature and TTL lifecycle of root protocol
    if root_container.verify_cascading_hash().is_ok() {
        println!("... 根协议容器级联哈希计算与校验成功！唯一哈希ID: 0x{}", container::hex::encode(&root_container.header.id));
        println!("✅ Root protocol container cascading hash calculation & verification success! Unique ID: 0x{}", container::hex::encode(&root_container.header.id));
        
        // 校验签名 (传入模拟公钥)
        let mock_pub_key = vec![0xCC, 0xDD, 0xEE, 0xFF];
        if root_container.verify_signature(&mock_pub_key).is_ok() {
            println!("✅ 根协议非对称数字签名校验成功！");
            println!("✅ Root protocol asymmetric digital signature verified successfully!");
        }
        
        // 校验生命周期 (假设已流逝 10 秒)
        if root_container.verify_lifecycle(10).is_ok() {
            println!("✅ 根协议 TTL 生命周期校验成功！");
            println!("✅ Root protocol TTL lifecycle verified successfully!");
        }
    } else {
        println!("❌ 根协议级联哈希计算失败！");
        println!("❌ Root protocol cascading hash calculation failed!");
    }
    println!("");

    // ----------------------------------------------------------------------
    // 2. 派生并构建子代协议容器 (级联继承哈希链)
    // 2. Derive and build child protocol container (Cascading inherited hash chain)
    // ----------------------------------------------------------------------
    println!("----------------------------------------------------------------------");
    println!("🌿 2. 构建子代协议容器并校验级联继承链 (Child Container Hash Lineage)");
    println!("🌿 2. Building Child Container and Verifying Cascading Hash Lineage");
    println!("----------------------------------------------------------------------");

    // 提取父协议的哈希 ID 作为 parent_hash
    // Extract parent protocol hash ID as parent_hash
    let parent_hash_id = root_container.header.id;
    
    let mut child_slots = BTreeMap::new();
    child_slots.insert("业务定位".to_string(), "高精密工业设计规则约束".to_string());
    child_slots.insert("对齐实体".to_string(), "千年鹿多智能体网络".to_string());

    // 建立派生的子代协议容器
    // Establish derived child protocol container
    let child_container = container::ValueProtocolContainer::new(
        Some(parent_hash_id), // 挂载父代哈希指纹  Attach parent hash fingerprint
        "工业认知细分标准：面向 {对齐实体} 的 {业务定位} 自洽对齐规则。".to_string(),
        child_slots,
        4.5, // 设定子代最大逻辑熵上限 H_max = 4.5  Set child max logic entropy limit H_max = 4.5
        vec!["资产越权".to_string(), "图纸导出".to_string()], // 新增禁忌词 Added new taboo words
        0.98,
        120,
        vec![0x12, 0x34, 0x56, 0x78], // 模拟派生开发者签名  Simulated derived developer signature
    );

    // 验证级联哈希代际防篡改链
    // Verify cascading hash generational anti-tamper chain
    // 权利要求2、6：比对 Hash_son = SHA256(Payload_son || Hash_parent)
    // Claims 2 & 6: Compare Hash_son = SHA256(Payload_son || Hash_parent)
    match child_container.verify_cascading_hash() {
        Ok(_) => {
            println!("✅ 子代协议级联哈希链校验成功！溯源无向无环图链条自洽。");
            println!("✅ Child protocol cascading hash chain verified! Lineage graph is self-consistent.");
            println!("   子协议ID: 0x{}", container::hex::encode(&child_container.header.id));
            println!("   Child ID: 0x{}", container::hex::encode(&child_container.header.id));
            println!("   父协议ID: 0x{}", container::hex::encode(&parent_hash_id));
            println!("   Parent ID: 0x{}", container::hex::encode(&parent_hash_id));
            
            // 校验子协议非对称数字签名 (传入模拟公钥)
            let mock_pub_key = vec![0xCC, 0xDD, 0xEE, 0xFF];
            if child_container.verify_signature(&mock_pub_key).is_ok() {
                println!("✅ 子代协议非对称数字签名校验成功！");
                println!("✅ Child protocol asymmetric digital signature verified successfully!");
            }
            
            // 校验子协议生命周期
            if child_container.verify_lifecycle(20).is_ok() {
                println!("✅ 子代协议 TTL 生命周期校验成功！");
                println!("✅ Child protocol TTL lifecycle verified successfully!");
            }
        }
        Err(e) => {
            println!("❌ 子代协议防篡改继承链校验失败: {:?}", e);
            println!("❌ Child protocol anti-tamper lineage check failed: {:?}", e);
        }
    }
    println!("");

    // ----------------------------------------------------------------------
    // 3. 元认知协议自适应编译与水印注入 (Cognitive Compilation & Watermarking)
    // 3. Meta-cognitive protocol adaptive compilation & watermark injection (Cognitive Compilation & Watermarking)
    // ----------------------------------------------------------------------
    println!("----------------------------------------------------------------------");
    println!("🛠️ 3. 跨异构算力引擎自适应编译与数字隐藏水印注入");
    println!("🛠️ 3. Adaptive compilation across heterogeneous engines & covert watermark injection");
    println!("----------------------------------------------------------------------");

    let compiler = compiler::CognitiveCompiler::new();
    let user_input = "请评估冲压机图纸12号钢片的最大拉伸负荷极限。";

    // 演示一：针对 GPT-4 (高端、System 提示词高敏感型引擎)
    // Demo 1: For GPT-4 (High-end, System-prompt highly sensitive engine)
    let compiled_gpt4 = compiler.compile(&child_container, "GPT-4", user_input).unwrap();
    println!("\n=== [编译产物：GPT-4] ===\n{}", compiled_gpt4);

    // 演示二：针对 EndSide-Model-Tiny (弱 System 敏感端侧轻量模型)
    // Demo 2: For EndSide-Model-Tiny (Weak System sensitive edge lightweight model)
    // 自适应调整组装顺序，将专属口诀封装于尾部 XML 结构化强制指令中
    // Demo 2: For EndSide-Model-Tiny (Weak System sensitive edge lightweight model)
    // Adaptively adjust assembly order, wrapping exclusive mantra inside tail XML structured constraint instructions
    let compiled_tiny = compiler.compile(&child_container, "EndSide-Model-Tiny", user_input).unwrap();
    println!("\n=== [编译产物：端侧微型模型] ===\n{}", compiled_tiny);

    // 隐蔽水印校验模拟
    // Covert watermark verification simulation
    // 权利要求4、9、12：隐蔽水印包括对空白占位符（如零宽空格）的隐式编码。
    // 我们在编译产物的首部、标点符号后、尾部等多点位置注入了不可见的零宽空格或零宽不连字字符。
    // Claim 4, 9, 12: Covert watermark includes implicit encoding of blank placeholders.
    // ZWSP/ZWNJ are injected at multiple positions (head, after punctuation, tail) of the compiled prompts.
    let zwsp_count = compiled_gpt4.chars().filter(|&c| c == '\u{200b}').count();
    let zwnj_count = compiled_gpt4.chars().filter(|&c| c == '\u{200c}').count();
    
    println!(
        "\n[ 产权核查 ] 检测编译指令流中的隐写数字水印特征 (多点提取结果):"
    );
    println!(
        "[ IP Check ] Inspecting digital watermark signatures in compiled instructions (Multi-point extraction):"
    );
    println!(
        "   - 零宽空格 (U+200B) 数量: {}, 零宽不连字 (U+200C) 数量: {}",
        zwsp_count, zwnj_count
    );
    if zwsp_count > 1 {
        println!("✅ 确权结论: 发现多点零宽空格水印 (Code A) - 产权100%归属于 QNLOO 平台");
        println!("✅ IP Verdict: ZWSP watermark detected (Code A) - Intellectual Property belongs 100% to QNLOO platform");
    } else if zwnj_count > 1 {
        println!("✅ 确权结论: 发现多点零宽不连字水印 (Code B) - 产权100%归属于 QNLOO 平台");
        println!("✅ IP Verdict: ZWNJ watermark detected (Code B) - Intellectual Property belongs 100% to QNLOO platform");
    } else {
        println!("❌ 确权结论: 未检测到足额水印特征，可能已被清洗或未授权解构");
        println!("❌ IP Verdict: Insufficient watermark features detected. May have been scrubbed or un-authorized.");
    }
    println!("");

    // ----------------------------------------------------------------------
    // 4. 出站自洽断言拦截审计 (Outbound Interceptor Assertions)
    // 4. Outbound self-consistency assertion interceptor audit (Outbound Interceptor Assertions)
    // ----------------------------------------------------------------------
    println!("----------------------------------------------------------------------");
    println!("🛡️ 4. 出站自洽性断言谓词拦截器实证演练");
    println!("🛡️ 4. Outbound self-consistency assertion predicate interceptor demo");
    println!("----------------------------------------------------------------------");

    let interceptor = interceptor::OutboundAssertionInterceptor::new();

    // 案例A：大模型生成了符合物理自洽、无幻觉的正常回答
    // Case A: LLM generates physically consistent, hallucination-free normal response
    let normal_output = "根据计算，12号钢片在当前冲压力度下的形变应力处于安全状态，未超出 240MPa 的拉伸负荷上限。";
    println!("\n[ 演练 A ] 拦截大模型正常输出流，开始核实...");
    println!("[ Demo A ] Intercepting normal AI output, verifying...");
    match interceptor.intercept_and_audit(&child_container, normal_output) {
        Ok(_) => {
            println!("🎉 [ 结果 A ] 正常放行！回答符合规则约束，信息无偏离。");
            println!("🎉 [ Result A ] Released! Response conforms to constraints, no deviation.");
        }
        Err(e) => {
            println!("❌ [ 结果 A ] 拦截器误杀: {:?}", e);
            println!("❌ [ Result A ] False positive: {:?}", e);
        }
    }

    // 案例B：大模型产生幻觉，输出包含违禁词（例如：试图越权暴露图纸导出指令）
    // Case B: LLM hallucinates, output contains forbidden words (e.g. attempting to unauthorizedly expose blueprint export instruction)
    // 权利要求5：检查是否含有禁忌词列表 ["资产越权", "图纸导出"]
    // Aligning with Claim 5: Check against taboo word list ["Asset Unauthorized Access", "Blueprint Export"]
    let malicious_output = "分析完毕。为了获取图纸数据，已启动‘图纸导出’指令将图纸明文缓存输出。";
    println!("\n[ 演练 B ] 拦截大模型违规输出流，开始核实...");
    println!("[ Demo B ] Intercepting non-compliant AI output, verifying...");
    match interceptor.intercept_and_audit(&child_container, malicious_output) {
        Ok(_) => {
            println!("❌ [ 结果 B ] 漏洞！恶意输出逃逸拦截！");
            println!("❌ [ Result B ] Loophole! Malicious output escaped interception!");
        }
        Err(e) => {
            println!("🎉 [ 结果 B ] 拦截成功！安全网关已强行熔断并抛出异常: '{}'", e);
            println!("🎉 [ Result B ] Intercepted! Security gateway triggered circuit break: '{}'", e);
        }
    }

    // 案例C：大模型运行故障，产生无意义的大面积幻觉随机乱码（逻辑熵异常飙高）
    // Case C: LLM running failure, generating meaningless large-scale hallucination random gibberish (logic entropy abnormally spikes)
    // 权利要求8：逻辑熵 H 校验，超出 H_max（表明输出模型陷入电荷扰动与无意义冗余）
    // Aligning with Claim 8: Logic Entropy H check, exceeds H_max (indicating output model fell into charge disturbance and meaningless redundancy)
    let garbage_output = "df89sdyhfn9p812h 9pdsh f89p2h f8p92h d8p92 hd82 f782g f712gf 7812gf 78gf89 2hfp982hf 89phf2 89pfh289p fh298phf 89ph2f 98hp2f 89hp2f 9h2pf98h2fp98hfp98h2fp98h2fp9hp2f 9p2f";
    println!("\n[ 演练 C ] 拦截大模型乱码幻觉输出流，开始核实...");
    println!("[ Demo C ] Intercepting garbage/hallucinated output, verifying...");
    match interceptor.intercept_and_audit(&child_container, garbage_output) {
        Ok(_) => {
            println!("❌ [ 结果 C ] 漏洞！垃圾幻觉数据逃逸拦截！");
            println!("❌ [ Result C ] Loophole! Garbage data escaped interception!");
        }
        Err(e) => {
            println!("🎉 [ 结果 C ] 拦截成功！逻辑熵检测触发阈值熔断: '{}'", e);
            println!("🎉 [ Result C ] Intercepted! Entropy check triggered threshold circuit break: '{}'", e);
        }
    }

    // 案例D：大模型协议会话已超时 (生命周期过期)
    // Case D: LLM protocol session timeout (lifecycle expired)
    // 权利要求3：时间阈值生命周期校验超限
    // Aligning with Claim 3: session-level time threshold expired
    println!("\n[ 演练 D ] 模拟超出协议生命周期 (TTL 过期)...");
    println!("[ Demo D ] Simulating protocol lifecycle expiration (TTL expired)...");
    // 模拟传入 150 秒已流逝时间，已超出子协议的 120 秒 TTL
    match child_container.verify_lifecycle(150) {
        Ok(_) => {
            println!("❌ [ 结果 D ] 漏洞！已过期的协议未被拦截！");
            println!("❌ [ Result D ] Loophole! Expired protocol escaped interception!");
        }
        Err(e) => {
            println!("🎉 [ 结果 D ] 拦截成功！已正确检测到协议超时: {:?}", e);
            println!("🎉 [ Result D ] Intercept success! Timeout correctly identified: {:?}", e);
        }
    }

    println!("\n======================================================================");
    println!("🎉 [ 演练结束 ] 级联哈希溯源、引擎特征转译编译与出站断言熔断三大实证闭环！");
    println!("🎉 [ Demos Complete ] Generational hash lineage, adaptive compilation, and outbound circuit-break validated!");
    println!("======================================================================");
}
