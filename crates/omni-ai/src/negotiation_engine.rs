use omni_core::CurrencyCode;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum NegotiationStrategy {
    /// Buyer isn't asking for a discount; quote standard price
    StandardCatalogQuote,
    /// Low-risk VIP buyer or multi-item bundle: grant instant loyalty discount within margin floor
    GrantVipOrBundleDiscount,
    /// Buyer wants discount on COD, or has Moderate/High RTO risk:
    /// Refuse COD discount, but offer an exclusive discount ONLY if they switch to Instant Prepaid / BNPL (Tabby/Atome)
    LeverageDiscountForPrepaidSwitch,
    /// Requested discount violates merchant's minimum margin guardrail: counter-offer at the safe floor
    CounterOfferAtMarginFloor,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NegotiationInput {
    pub product_sku: String,
    pub product_title: String,
    pub quantity: u32,
    pub unit_price_major: f64,
    pub unit_cogs_major: f64,          // Merchant's secret Cost of Goods Sold
    pub min_required_margin_pct: f64,  // e.g., 25.0% minimum net margin floor
    pub max_allowed_discount_pct: f64, // e.g., 12.0% hard cap
    pub currency: CurrencyCode,
    pub buyer_requested_discount_pct: f64,
    pub buyer_fraud_risk_score: i32,   // 0..=100 from Fraud Shield
    pub buyer_preferred_language: String, // "ar" | "en" | "bn"
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NegotiationDecision {
    pub strategy: NegotiationStrategy,
    pub original_total_major: f64,
    pub approved_cod_price_major: f64,
    pub approved_prepaid_or_bnpl_price_major: f64,
    pub unlocked_prepaid_discount_pct: f64,
    pub projected_merchant_margin_pct: f64,
    pub expected_rto_savings_major: f64,
    pub ai_sales_pitch_en: String,
    pub ai_sales_pitch_ar: String,
    pub bnpl_installment_preview: String, // Tabby / Tamara (GCC) or Atome (SEA) 4x interest-free split
}

/// Industry-First Innovation: Risk-Aware & Margin-Protected Autonomous AI Haggling Engine.
/// Uses dynamic discounts as a psychological lever to convert risky COD shoppers into 0%-RTO Prepaid/BNPL buyers
/// while mathematically guaranteeing the merchant never drops below `min_required_margin_pct`.
pub fn evaluate_autonomous_negotiation(input: &NegotiationInput) -> NegotiationDecision {
    let qty = input.quantity.max(1) as f64;
    let original_total = input.unit_price_major * qty;
    let total_cogs = input.unit_cogs_major * qty;

    // Calculate maximum discount % that still preserves `min_required_margin_pct`
    // Margin % = ((DiscountedPrice - COGS) / DiscountedPrice) * 100 >= min_margin
    // => MinimumSafePrice = COGS / (1.0 - min_margin / 100.0)
    let min_safe_ratio = (1.0 - (input.min_required_margin_pct / 100.0)).max(0.15);
    let min_safe_total_price = total_cogs / min_safe_ratio;
    let max_margin_safe_discount_pct = if original_total > min_safe_total_price {
        (((original_total - min_safe_total_price) / original_total) * 100.0).max(0.0)
    } else {
        0.0
    };

    let mut ceiling_discount_pct = input
        .max_allowed_discount_pct
        .min(max_margin_safe_discount_pct);

    // Volume bundle bonus: +2% extra ceiling if buying 2+ units (still capped by margin floor)
    if input.quantity >= 2 {
        ceiling_discount_pct = (ceiling_discount_pct + 2.0).min(max_margin_safe_discount_pct);
    }

    let requested = input.buyer_requested_discount_pct.max(0.0);

    let (strategy, cod_discount_pct, prepaid_discount_pct) = if requested == 0.0 && input.buyer_fraud_risk_score < 25 {
        (NegotiationStrategy::StandardCatalogQuote, 0.0, (ceiling_discount_pct * 0.4).round())
    } else if input.buyer_fraud_risk_score >= 25 {
        // Moderate or High RTO Risk: ZERO discount on COD, but unlock a generous margin-safe discount
        // if they switch to Instant Prepaid / Tabby BNPL!
        let nudge_discount = requested.clamp(5.0, ceiling_discount_pct.max(5.0)).min(max_margin_safe_discount_pct);
        (
            NegotiationStrategy::LeverageDiscountForPrepaidSwitch,
            0.0,
            (nudge_discount * 10.0).round() / 10.0,
        )
    } else if requested <= ceiling_discount_pct {
        // Low-risk VIP buyer asking within safe margin
        let cod_disc = (requested * 0.5).round();
        let prepay_disc = requested.max(5.0).min(ceiling_discount_pct);
        (
            NegotiationStrategy::GrantVipOrBundleDiscount,
            cod_disc,
            (prepay_disc * 10.0).round() / 10.0,
        )
    } else {
        // Buyer asked for e.g. 25% off, which breaks margin floor -> Counter-offer at exact safe ceiling
        let safe_disc = (ceiling_discount_pct * 10.0).round() / 10.0;
        (
            NegotiationStrategy::CounterOfferAtMarginFloor,
            0.0,
            safe_disc,
        )
    };

    let approved_cod_price = ((original_total * (1.0 - cod_discount_pct / 100.0)) * 100.0).round() / 100.0;
    let approved_prepaid_price =
        ((original_total * (1.0 - prepaid_discount_pct / 100.0)) * 100.0).round() / 100.0;

    let projected_margin_pct = if approved_prepaid_price > 0.0 {
        (((approved_prepaid_price - total_cogs) / approved_prepaid_price) * 100.0 * 10.0).round() / 10.0
    } else {
        0.0
    };

    // Estimated shipping + reverse logistics loss avoided when converting COD -> Prepaid
    let expected_rto_savings_major = if input.buyer_fraud_risk_score >= 25 {
        ((original_total * 0.08) + 35.0).round()
    } else {
        18.0
    };

    let installment_4x = ((approved_prepaid_price / 4.0) * 100.0).round() / 100.0;
    let cur = input.currency.as_str();

    let bnpl_installment_preview = format!(
        "4 interest-free payments of {:.2} {} via Tabby / Tamara / Atome (0% RTO Risk for Merchant)",
        installment_4x, cur
    );

    let ai_sales_pitch_en = match strategy {
        NegotiationStrategy::LeverageDiscountForPrepaidSwitch => format!(
            "I can't apply a discount on Cash-on-Delivery ({:.2} {}), BUT if you complete your order via Instant Prepaid or Tabby 4x Split right now, I've unlocked an exclusive {:.1}% VIP discount for you — pay just {:.2} {} (or 4x {:.2} {}) with Priority Express Dispatch!",
            approved_cod_price, cur, prepaid_discount_pct, approved_prepaid_price, cur, installment_4x, cur
        ),
        NegotiationStrategy::CounterOfferAtMarginFloor => format!(
            "Since {} is our craftsman limited edition, I can't do {:.0}% off — but as my best final offer, I can unlock {:.1}% off ({:.2} {} instead of {:.2} {}) when you checkout via Apple Pay or Tabby today!",
            input.product_title, requested, prepaid_discount_pct, approved_prepaid_price, cur, original_total, cur
        ),
        _ => format!(
            "Awesome choice! As a verified shopper, you can grab {} for {:.2} {} on COD, or save {:.1}% instantly ({:.2} {}) with Instant Prepaid / Tabby!",
            input.product_title, approved_cod_price, cur, prepaid_discount_pct, approved_prepaid_price, cur
        ),
    };

    let ai_sales_pitch_ar = format!(
        "عرض خاص لك! السعر عند الاستلام هو {:.2} {}، ولكن عند الدفع الإلكتروني الفوري (Apple Pay أو تابي 4 دفعات بقيمة {:.2} {}) ستحصل على خصم حصري {:.1}% ليصبح الإجمالي {:.2} {} فقط مع شحن سريع مجاني!",
        approved_cod_price, cur, installment_4x, cur, prepaid_discount_pct, approved_prepaid_price, cur
    );

    NegotiationDecision {
        strategy,
        original_total_major: original_total,
        approved_cod_price_major: approved_cod_price,
        approved_prepaid_or_bnpl_price_major: approved_prepaid_price,
        unlocked_prepaid_discount_pct: prepaid_discount_pct,
        projected_merchant_margin_pct: projected_margin_pct,
        expected_rto_savings_major,
        ai_sales_pitch_en,
        ai_sales_pitch_ar,
        bnpl_installment_preview,
    }
}
