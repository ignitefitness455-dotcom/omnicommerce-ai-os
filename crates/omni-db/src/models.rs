use crate::schema::{customer_network_fingerprints, fraud_incident_reports, merchants, orders};
use bigdecimal::BigDecimal;
use chrono::{DateTime, Utc};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Selectable, Identifiable)]
#[diesel(table_name = merchants)]
pub struct MerchantRecord {
    pub id: Uuid,
    pub business_name: String,
    pub store_domain: Option<String>,
    pub platform_type: String,
    pub primary_country_iso: String,
    pub default_currency: String,
    pub email: String,
    pub api_key_hash: String,
    pub cod_auto_approve_max_score: i32,
    pub cod_block_min_score: i32,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Selectable, Identifiable)]
#[diesel(table_name = customer_network_fingerprints)]
pub struct CustomerFingerprintRecord {
    pub id: Uuid,
    pub phone_hash: String,
    pub country_iso: String,
    pub region_group: String,
    pub total_orders_placed: i32,
    pub total_orders_delivered: i32,
    pub total_rto_returned: i32,
    pub total_chargebacks: i32,
    pub recent_90d_incidents: i32,
    pub rto_rate_pct: BigDecimal,
    pub risk_score: i32,
    pub risk_tier: String,
    pub is_globally_blacklisted: bool,
    pub blacklist_reason: Option<String>,
    pub last_seen_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Insertable)]
#[diesel(table_name = fraud_incident_reports)]
pub struct NewFraudIncidentReport {
    pub merchant_id: Uuid,
    pub fingerprint_id: Uuid,
    pub incident_type: String,
    pub carrier_provider: Option<String>,
    pub tracking_number: Option<String>,
    pub order_amount_minor: i64,
    pub currency: String,
    pub evidence_summary: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Selectable, Identifiable)]
#[diesel(table_name = orders)]
pub struct OrderRecord {
    pub id: Uuid,
    pub merchant_id: Uuid,
    pub external_store_order_id: Option<String>,
    pub fingerprint_id: Uuid,
    pub source_channel: String,
    pub customer_name: String,
    pub customer_phone_e164: String,
    pub shipping_address: String,
    pub shipping_city: String,
    pub country_iso: String,
    pub total_amount_minor: i64,
    pub currency: String,
    pub payment_method: String,
    pub assessed_risk_score: i32,
    pub assessed_risk_tier: String,
    pub applied_intervention: String,
    pub whatsapp_flow_confirmed: bool,
    pub order_status: String,
    pub carrier_provider: Option<String>,
    pub tracking_number: Option<String>,
    pub awb_label_url: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
