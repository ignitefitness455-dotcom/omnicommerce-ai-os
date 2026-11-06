use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum SaasPlanTier {
    Growth,     // $79/mo — SMB Social Sellers
    Scale,      // $249/mo — High-Velocity GCC/SEA D2C Brands
    Enterprise, // $699/mo — Multi-Brand Retail Groups
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MerchantRoiInput {
    pub plan_tier: SaasPlanTier,
    pub monthly_orders_processed: u32,
    pub baseline_rto_rate_pct: f64,
    pub current_rto_rate_pct: f64,
    pub avg_rto_shipping_loss_usd: f64,
    pub abandoned_chats_recovered: u32,
    pub avg_order_net_profit_usd: f64,
    pub avg_carrier_arbitrage_savings_per_order_usd: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MerchantRoiReport {
    pub plan_tier: SaasPlanTier,
    pub monthly_subscription_fee_usd: f64,
    pub prevented_rto_parcels: u32,
    pub rto_shipping_losses_saved_usd: f64,
    pub recovered_chat_net_profit_usd: f64,
    pub carrier_arbitrage_total_savings_usd: f64,
    pub total_monthly_value_generated_usd: f64,
    pub net_roi_multiple: f64, // e.g., 18.4x ROI
}

/// Computes the real-time SaaS ROI & Data Lock-In Report for the Merchant Dashboard.
pub fn calculate_merchant_saas_roi(input: &MerchantRoiInput) -> MerchantRoiReport {
    let monthly_subscription_fee_usd = match input.plan_tier {
        SaasPlanTier::Growth => 79.0,
        SaasPlanTier::Scale => 249.0,
        SaasPlanTier::Enterprise => 699.0,
    };

    let rto_reduction_pct = (input.baseline_rto_rate_pct - input.current_rto_rate_pct).max(0.0);
    let prevented_rto_parcels =
        ((input.monthly_orders_processed as f64) * (rto_reduction_pct / 100.0)).round() as u32;

    let rto_shipping_losses_saved_usd =
        ((prevented_rto_parcels as f64) * input.avg_rto_shipping_loss_usd * 100.0).round() / 100.0;

    let recovered_chat_net_profit_usd = ((input.abandoned_chats_recovered as f64)
        * input.avg_order_net_profit_usd
        * 100.0)
        .round()
        / 100.0;

    let carrier_arbitrage_total_savings_usd = ((input.monthly_orders_processed as f64)
        * input.avg_carrier_arbitrage_savings_per_order_usd
        * 100.0)
        .round()
        / 100.0;

    let total_monthly_value_generated_usd = ((rto_shipping_losses_saved_usd
        + recovered_chat_net_profit_usd
        + carrier_arbitrage_total_savings_usd)
        * 100.0)
        .round()
        / 100.0;

    let net_roi_multiple =
        ((total_monthly_value_generated_usd / monthly_subscription_fee_usd) * 10.0).round() / 10.0;

    MerchantRoiReport {
        plan_tier: input.plan_tier,
        monthly_subscription_fee_usd,
        prevented_rto_parcels,
        rto_shipping_losses_saved_usd,
        recovered_chat_net_profit_usd,
        carrier_arbitrage_total_savings_usd,
        total_monthly_value_generated_usd,
        net_roi_multiple,
    }
}
