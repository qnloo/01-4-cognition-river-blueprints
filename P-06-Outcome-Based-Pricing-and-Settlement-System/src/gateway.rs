// P-RZZH-060-QNLOO-2026-v2: 本地隐私确权与同步 - 三层价格梯度网关模块 (Gateway Module)
// P-RZZH-060-QNLOO-2026-v2: Local Privacy Ownership & Sync - Three-Tier Price Gradient Gateway Module
// 物理安全机制：累计调用计数器控制计费层在 FreeTrial -> PaidAI -> OutcomeHuman 三层状态机强行跳转
// Physical Security: Call counter controls transition in FreeTrial -> PaidAI -> OutcomeHuman state machine

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PricingTier {
    FreeTrial,
    PaidAI,
    OutcomeHuman,
}

pub struct PriceGateway {
    pub free_trial_limit: u32,
    pub paid_ai_limit: u32,
}

impl PriceGateway {
    pub fn new() -> Self {
        Self {
            // 权利要求5：免费上限限制在 1-3 次
            // Claim 5: Free trial limit set at 1-3 times
            free_trial_limit: 3,
            // 付费AI限制在 3 次以内
            // Paid AI limit set within 3 times
            paid_ai_limit: 3,
        }
    }

    // 根据调用计数器强行转换价格档位状态
    // State transition of price tier based on call counter
    // 对应权利要求1、4、5：三层网关状态强行跳转与计费模式拦截切换
    // Aligning with Claims 1, 4, 5: Forced state transition and billing mode interception
    pub fn determine_tier(&self, trial_counter: u32, paid_ai_counter: u32) -> PricingTier {
        if trial_counter <= self.free_trial_limit {
            PricingTier::FreeTrial
        } else if paid_ai_counter <= self.paid_ai_limit {
            PricingTier::PaidAI
        } else {
            PricingTier::OutcomeHuman
        }
    }
}
