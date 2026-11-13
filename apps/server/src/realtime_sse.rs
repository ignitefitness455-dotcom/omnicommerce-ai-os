use crate::AppState;
use axum::{routing::get, Json, Router};
use serde::Serialize;
use std::sync::Arc;

pub fn events_router(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/summary", get(telemetry_summary_handler))
        .with_state(state)
}

#[derive(Debug, Serialize)]
pub struct RtoTrendPoint {
    pub day_label: &'static str,
    pub baseline_rto_pct: f64,
    pub omni_shield_rto_pct: f64,
}

#[derive(Debug, Serialize)]
pub struct RegionalDeliveryMetric {
    pub country_iso: &'static str,
    pub region_name: &'static str,
    pub delivery_success_pct: f64,
    pub active_carrier: &'static str,
}

#[derive(Debug, Serialize)]
pub struct LiveOrderEventNotice {
    pub event_id: &'static str,
    pub channel: &'static str,
    pub message: &'static str,
    pub timestamp_label: &'static str,
}

#[derive(Debug, Serialize)]
pub struct PlatformTelemetrySummary {
    pub active_merchants: u32,
    pub total_shopper_fingerprints: u64,
    pub current_network_rto_pct: f64,
    pub ai_haggling_prepaid_conversion_pct: f64,
    pub rto_trend_30d: Vec<RtoTrendPoint>,
    pub regional_delivery_rates: Vec<RegionalDeliveryMetric>,
    pub recent_live_events: Vec<LiveOrderEventNotice>,
}

async fn telemetry_summary_handler() -> Json<PlatformTelemetrySummary> {
    Json(PlatformTelemetrySummary {
        active_merchants: 1842,
        total_shopper_fingerprints: 1_420_890,
        current_network_rto_pct: 9.6,
        ai_haggling_prepaid_conversion_pct: 41.8,
        rto_trend_30d: vec![
            RtoTrendPoint { day_label: "Day 1", baseline_rto_pct: 31.4, omni_shield_rto_pct: 29.8 },
            RtoTrendPoint { day_label: "Day 6", baseline_rto_pct: 31.2, omni_shield_rto_pct: 24.5 },
            RtoTrendPoint { day_label: "Day 12", baseline_rto_pct: 31.6, omni_shield_rto_pct: 19.2 },
            RtoTrendPoint { day_label: "Day 18", baseline_rto_pct: 31.1, omni_shield_rto_pct: 14.7 },
            RtoTrendPoint { day_label: "Day 24", baseline_rto_pct: 31.5, omni_shield_rto_pct: 11.4 },
            RtoTrendPoint { day_label: "Day 30", baseline_rto_pct: 31.4, omni_shield_rto_pct: 9.6 },
        ],
        regional_delivery_rates: vec![
            RegionalDeliveryMetric {
                country_iso: "AE",
                region_name: "UAE (Dubai / Abu Dhabi)",
                delivery_success_pct: 94.8,
                active_carrier: "Aramex GCC",
            },
            RegionalDeliveryMetric {
                country_iso: "SA",
                region_name: "Saudi Arabia (Riyadh / Jeddah)",
                delivery_success_pct: 91.5,
                active_carrier: "Aramex + SMSA Arbitrage",
            },
            RegionalDeliveryMetric {
                country_iso: "MY",
                region_name: "Malaysia (Kuala Lumpur)",
                delivery_success_pct: 93.8,
                active_carrier: "Ninja Van SEA Pro",
            },
            RegionalDeliveryMetric {
                country_iso: "ID",
                region_name: "Indonesia (Jakarta)",
                delivery_success_pct: 89.4,
                active_carrier: "Ninja Van + J&T Smart Route",
            },
        ],
        recent_live_events: vec![
            LiveOrderEventNotice {
                event_id: "EVT-9041",
                channel: "SALLA_KSA",
                message: "Faisal Al-Saud switched COD -> Tabby 4x Prepaid (5% Off) • ARMX-SA-PREPAID99",
                timestamp_label: "Just now",
            },
            LiveOrderEventNotice {
                event_id: "EVT-9040",
                channel: "SHOPIFY_UAE",
                message: "Sara Al-Maktoum passed Fraud Shield (9/100 SAFE) • Auto-booked Aramex DXB",
                timestamp_label: "2m ago",
            },
            LiveOrderEventNotice {
                event_id: "EVT-9039",
                channel: "WHATSAPP_FLOW",
                message: "Blocked high-RTO repeat offender (+971520009999, Score: 94/100) • Saved 35 AED",
                timestamp_label: "5m ago",
            },
        ],
    })
}
