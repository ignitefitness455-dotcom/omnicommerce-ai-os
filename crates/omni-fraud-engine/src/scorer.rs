use crate::address_nlp::{evaluate_address_quality, AddressQualityReport};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum RiskTier {
    Safe,
    Moderate,
    High,
    Blocked,
}

/// GoKwik-inspired Dynamic Checkout & WhatsApp COD Interventions
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum CodIntervention {
    /// Score 0..=24: Auto-confirm order & book courier immediately
    AllowSeamlessCod,
    /// Score 25..=49: Dispatch interactive WhatsApp Flow requiring 1-tap address & OTP confirmation
    WhatsAppFlowConfirmation,
    /// Score 50..=74: Require 20% shipping deposit OR offer 5% instant discount to switch to Prepaid
    PartialAdvanceOrPrepaidDiscountNudge,
    /// Score 75..=100: Block COD completely; require 100% digital prepayment (Stripe/ApplePay/Tabby)
    BlockCodPrepaidOnly,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FraudEvaluationInput {
    pub phone_hash: String,
    pub country_iso: String,
    pub region_group: String,
    pub total_network_orders: i32,
    pub total_network_delivered: i32,
    pub total_network_rto_returned: i32,
    pub total_network_chargebacks: i32,
    pub recent_90d_incidents: i32,
    pub orders_last_2_hours_velocity: i32,
    pub is_manually_blacklisted: bool,
    pub shipping_address: String,
    pub shipping_city: Option<String>,
    pub postal_cluster_rto_rate: f64, // 0.0 to 100.0
    pub order_amount_usd_equiv: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiskSignalBreakdown {
    pub network_rto_component: i32,
    pub incident_severity_component: i32,
    pub address_nlp_component: i32,
    pub velocity_and_value_component: i32,
    pub geo_cluster_component: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FraudRiskAssessment {
    pub phone_hash: String,
    pub risk_score: i32,
    pub risk_tier: RiskTier,
    pub historical_rto_rate_pct: f64,
    pub network_trust_confidence: &'static str,
    pub recommended_intervention: CodIntervention,
    pub merchant_action_message: String,
    pub breakdown: RiskSignalBreakdown,
    pub address_report: AddressQualityReport,
    pub risk_flags: Vec<String>,
}

/// Evaluates cross-merchant shopper behavior and order context in < 2ms.
pub fn evaluate_global_fraud_risk(input: &FraudEvaluationInput) -> FraudRiskAssessment {
    let address_report =
        evaluate_address_quality(&input.shipping_address, input.shipping_city.as_deref());
    let mut risk_flags = address_report.flags.clone();

    let historical_rto_rate_pct = if input.total_network_orders > 0 {
        ((input.total_network_rto_returned as f64) / (input.total_network_orders as f64)) * 100.0
    } else {
        0.0
    };

    if input.is_manually_blacklisted {
        risk_flags.push("NETWORK_GLOBAL_BLACKLIST".to_string());
        return FraudRiskAssessment {
            phone_hash: input.phone_hash.clone(),
            risk_score: 100,
            risk_tier: RiskTier::Blocked,
            historical_rto_rate_pct: historical_rto_rate_pct.clamp(0.0, 100.0),
            network_trust_confidence: "HIGH",
            recommended_intervention: CodIntervention::BlockCodPrepaidOnly,
            merchant_action_message:
                "CRITICAL: Identity is blacklisted across the OmniCommerce Network due to confirmed fraud or repeated doorstep refusals. Disable COD immediately."
                    .to_string(),
            breakdown: RiskSignalBreakdown {
                network_rto_component: 40,
                incident_severity_component: 25,
                address_nlp_component: 15,
                velocity_and_value_component: 10,
                geo_cluster_component: 10,
            },
            address_report,
            risk_flags,
        };
    }

    // 1. Bayesian-Smoothed Network RTO Component (Max 40 points)
    // Prior: 1.5 virtual orders with 0.25 virtual RTO (~16% baseline for emerging COD markets)
    let smoothed_rto_ratio = (input.total_network_rto_returned as f64 + 0.25)
        / (input.total_network_orders as f64 + 1.5);
    let mut network_rto_component = (smoothed_rto_ratio * 48.0).round() as i32;
    if input.total_network_delivered >= 5 && input.total_network_rto_returned == 0 {
        // Reward verified loyal shoppers
        network_rto_component = 0;
    } else {
        network_rto_component = network_rto_component.clamp(0, 40);
    }

    if historical_rto_rate_pct >= 40.0 && input.total_network_orders >= 2 {
        risk_flags.push(format!(
            "HIGH_HISTORICAL_RTO_RATE:{:.0}%",
            historical_rto_rate_pct
        ));
    }

    // 2. Recent Incident & Chargeback Component (Max 25 points)
    let incident_points =
        (input.recent_90d_incidents * 12) + (input.total_network_chargebacks * 18);
    let incident_severity_component = incident_points.clamp(0, 25);
    if input.recent_90d_incidents > 0 {
        risk_flags.push(format!(
            "RECENT_90D_FRAUD_REPORTS:{}",
            input.recent_90d_incidents
        ));
    }
    if input.total_network_chargebacks > 0 {
        risk_flags.push("PAST_CHARGEBACK_DISPUTE".to_string());
    }

    // 3. Address Quality NLP Component (Max 15 points)
    let address_nlp_component = ((address_report.penalty_score as f64) * 0.15).round() as i32;

    // 4. Velocity & High Ticket COD Spike Component (Max 10 points)
    let mut velocity_and_value_component = 0;
    if input.orders_last_2_hours_velocity >= 3 {
        velocity_and_value_component += 7;
        risk_flags.push("ABNORMAL_ORDER_BURST_VELOCITY".to_string());
    }
    if input.order_amount_usd_equiv >= 250.0 && input.total_network_delivered == 0 {
        velocity_and_value_component += 5;
        risk_flags.push("FIRST_TIME_HIGH_TICKET_COD".to_string());
    }
    let velocity_and_value_component = velocity_and_value_component.clamp(0, 10);

    // 5. Geo / Postal Cluster Risk Component (Max 10 points)
    let geo_cluster_component = ((input.postal_cluster_rto_rate / 100.0) * 10.0)
        .round()
        .clamp(0.0, 10.0) as i32;
    if input.postal_cluster_rto_rate >= 35.0 {
        risk_flags.push("HIGH_RTO_POSTAL_ZONE".to_string());
    }

    let total_score = (network_rto_component
        + incident_severity_component
        + address_nlp_component
        + velocity_and_value_component
        + geo_cluster_component)
        .clamp(0, 100);

    let network_trust_confidence = match input.total_network_orders {
        0 => "NEW_SHOPPER",
        1..=3 => "MODERATE_HISTORY",
        _ => "HIGH_NETWORK_CONFIDENCE",
    };

    let (risk_tier, recommended_intervention, merchant_action_message) = match total_score {
        0..=24 => (
            RiskTier::Safe,
            CodIntervention::AllowSeamlessCod,
            "Verified low-risk shopper. Safe to auto-confirm COD and book Aramex/NinjaVan shipment immediately.".to_string(),
        ),
        25..=49 => (
            RiskTier::Moderate,
            CodIntervention::WhatsAppFlowConfirmation,
            "Moderate RTO risk. Auto-trigger WhatsApp Flow confirmation to verify delivery pin and intent before courier dispatch.".to_string(),
        ),
        50..=74 => (
            RiskTier::High,
            CodIntervention::PartialAdvanceOrPrepaidDiscountNudge,
            "High RTO probability. Hold COD dispatch; send automated WhatsApp payment link for 20% advance shipping deposit or offer 5% prepaid discount.".to_string(),
        ),
        _ => (
            RiskTier::Blocked,
            CodIntervention::BlockCodPrepaidOnly,
            "Severe RTO / Fraud signature across network. Block Cash-on-Delivery and require 100% verified digital prepayment.".to_string(),
        ),
    };

    FraudRiskAssessment {
        phone_hash: input.phone_hash.clone(),
        risk_score: total_score,
        risk_tier,
        historical_rto_rate_pct: (historical_rto_rate_pct * 100.0).round() / 100.0,
        network_trust_confidence,
        recommended_intervention,
        merchant_action_message,
        breakdown: RiskSignalBreakdown {
            network_rto_component,
            incident_severity_component,
            address_nlp_component,
            velocity_and_value_component,
            geo_cluster_component,
        },
        address_report,
        risk_flags,
    }
}
