use omni_core::CurrencyCode;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum RetargetingWave {
    /// T + 45 mins: Stock scarcity reminder while intent is warm
    Wave1ScarcityReminder45m,
    /// T + 6 hours: Zero-interest 4x BNPL (Tabby/Tamara/Atome) framing
    Wave2BnplSplitFraming6h,
    /// T + 22 hours: Final margin-safe flash incentive before Meta's 24h conversation window closes
    Wave3Meta24hWindowClosingFlashOffer22h,
    /// Suppressed! Do not waste WhatsApp template cost on a high-RTO / blacklisted shopper
    SuppressedByFraudShield,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AbandonedChatInput {
    pub session_id: Uuid,
    pub buyer_name: String,
    pub buyer_phone_e164: String,
    pub buyer_fraud_risk_score: i32,
    pub product_sku: String,
    pub product_title: String,
    pub cart_value_major: f64,
    pub unit_cogs_major: f64,
    pub currency: CurrencyCode,
    pub hours_since_last_buyer_message: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScheduledRecoveryTouchpoint {
    pub wave: RetargetingWave,
    pub trigger_offset_label: &'static str,
    pub inside_free_meta_24h_service_window: bool,
    pub unlocked_discount_pct: f64,
    pub final_offer_price_major: f64,
    pub message_en: String,
    pub message_ar: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RetargetingCampaignPlan {
    pub session_id: Uuid,
    pub eligible_for_retargeting: bool,
    pub suppression_reason: Option<String>,
    pub meta_marketing_fee_saved_usd: f64,
    pub expected_conversion_probability_pct: f64,
    pub touchpoints: Vec<ScheduledRecoveryTouchpoint>,
}

/// Fraud-Filtered & Meta-24h-Window-Optimized Auto-Retargeting Engine.
/// 1. Automatically suppresses retargeting on High-Risk/Blocked RTO profiles (saving Meta messaging fees & future RTOs).
/// 2. Schedules 3 high-converting touchpoints inside the 24-hour open conversation window.
pub fn build_fraud_filtered_retargeting_plan(input: &AbandonedChatInput) -> RetargetingCampaignPlan {
    // Innovation: Never spend marketing dollars retargeting serial returners!
    if input.buyer_fraud_risk_score >= 65 {
        return RetargetingCampaignPlan {
            session_id: input.session_id,
            eligible_for_retargeting: false,
            suppression_reason: Some(format!(
                "SUPPRESSED BY FRAUD SHIELD: Buyer risk score is {}/100. Blocked automated retargeting to save Meta WhatsApp template cost and prevent a high-probability COD return.",
                input.buyer_fraud_risk_score
            )),
            meta_marketing_fee_saved_usd: 0.14,
            expected_conversion_probability_pct: 0.0,
            touchpoints: vec![],
        };
    }

    let cur = input.currency.as_str();
    let split_4x = ((input.cart_value_major / 4.0) * 100.0).round() / 100.0;
    let wave3_discount_pct = 6.0;
    let wave3_price = ((input.cart_value_major * (1.0 - wave3_discount_pct / 100.0)) * 100.0).round() / 100.0;

    let touchpoints = vec![
        ScheduledRecoveryTouchpoint {
            wave: RetargetingWave::Wave1ScarcityReminder45m,
            trigger_offset_label: "T + 45 Minutes (Warm Intent)",
            inside_free_meta_24h_service_window: true,
            unlocked_discount_pct: 0.0,
            final_offer_price_major: input.cart_value_major,
            message_en: format!(
                "Hi {}! We reserved your {} ({:.2} {}) in our warehouse for the next 2 hours. Tap below to confirm your delivery pin!",
                input.buyer_name, input.product_title, input.cart_value_major, cur
            ),
            message_ar: format!(
                "مرحباً {}! قمنا بحجز {} ({:.2} {}) لك لمدة ساعتين. اضغط أدناه لتأكيد عنوان التوصيل فوراً!",
                input.buyer_name, input.product_title, input.cart_value_major, cur
            ),
        },
        ScheduledRecoveryTouchpoint {
            wave: RetargetingWave::Wave2BnplSplitFraming6h,
            trigger_offset_label: "T + 6 Hours (BNPL 4x Split Unlock)",
            inside_free_meta_24h_service_window: true,
            unlocked_discount_pct: 0.0,
            final_offer_price_major: input.cart_value_major,
            message_en: format!(
                "Still thinking about {}? You can get it shipped today for just {:.2} {} now (4 interest-free payments via Tabby / Atome)!",
                input.product_title, split_4x, cur
            ),
            message_ar: format!(
                "لا تفوت فرصة الحصول على {}! يمكنك تقسيط المبلغ على 4 دفعات بدون فوائد عبر تابي/تمارا بقيمة {:.2} {} فقط للدفعة الأولى!",
                input.product_title, split_4x, cur
            ),
        },
        ScheduledRecoveryTouchpoint {
            wave: RetargetingWave::Wave3Meta24hWindowClosingFlashOffer22h,
            trigger_offset_label: "T + 22 Hours (Pre-24h Window Closing Flash Offer)",
            inside_free_meta_24h_service_window: true,
            unlocked_discount_pct: wave3_discount_pct,
            final_offer_price_major: wave3_price,
            message_en: format!(
                "Final 2-hour VIP offer for {}: Complete your checkout online now and enjoy {:.0}% off — pay just {:.2} {} with Priority Express Shipping!",
                input.buyer_name, wave3_discount_pct, wave3_price, cur
            ),
            message_ar: format!(
                "عرض أخير لمدة ساعتين فقط لـ {}: أكمل طلبك الآن عبر الدفع الإلكتروني واحصل على خصم {:.0}% ليصبح السعر {:.2} {} فقط مع شحن سريع!",
                input.buyer_name, wave3_discount_pct, wave3_price, cur
            ),
        },
    ];

    RetargetingCampaignPlan {
        session_id: input.session_id,
        eligible_for_retargeting: true,
        suppression_reason: None,
        meta_marketing_fee_saved_usd: 0.28, // Sent inside 24h service window before paid marketing template required
        expected_conversion_probability_pct: 34.5,
        touchpoints,
    }
}
