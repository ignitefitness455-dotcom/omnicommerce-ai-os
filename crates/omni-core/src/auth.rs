use crate::crypto::compute_network_fingerprint;
use chrono::{Duration, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MerchantSessionClaims {
    pub sub_merchant_id: Uuid,
    pub business_name: String,
    pub primary_country_iso: String,
    pub default_currency: String,
    pub plan_tier: String,
    pub issued_at_unix: i64,
    pub expires_at_unix: i64,
    pub signature_hex: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneratedApiKeyBundle {
    pub merchant_id: Uuid,
    pub publishable_key: String,
    pub secret_api_key_plaintext_once: String,
    pub api_key_hash_stored: String,
    pub created_at_iso: String,
}

/// Generates a Stripe-style `omni_live_...` secret API key and its `HMAC-SHA256` hash for storage.
pub fn generate_merchant_api_key(merchant_id: Uuid, pepper: &str) -> GeneratedApiKeyBundle {
    let random_part = format!("{}{}", Uuid::new_v4().simple(), Uuid::new_v4().simple());
    let secret_api_key = format!("omni_live_{}", &random_part[..36]);
    let publishable_key = format!("omni_pk_{}", &merchant_id.simple().to_string()[..16]);
    let api_key_hash = compute_network_fingerprint(&secret_api_key, pepper);

    GeneratedApiKeyBundle {
        merchant_id,
        publishable_key,
        secret_api_key_plaintext_once: secret_api_key,
        api_key_hash_stored: api_key_hash,
        created_at_iso: Utc::now().to_rfc3339(),
    }
}

/// Issues a signed HMAC-SHA256 session token for the Merchant Dashboard.
pub fn issue_merchant_session_token(
    merchant_id: Uuid,
    business_name: &str,
    country_iso: &str,
    currency: &str,
    plan_tier: &str,
    jwt_secret: &str,
) -> MerchantSessionClaims {
    let now = Utc::now();
    let exp = now + Duration::hours(24);
    let payload_str = format!(
        "{}:{}:{}:{}:{}:{}",
        merchant_id,
        business_name,
        country_iso,
        currency,
        plan_tier,
        exp.timestamp()
    );
    let signature_hex = compute_network_fingerprint(&payload_str, jwt_secret);

    MerchantSessionClaims {
        sub_merchant_id: merchant_id,
        business_name: business_name.to_string(),
        primary_country_iso: country_iso.to_string(),
        default_currency: currency.to_string(),
        plan_tier: plan_tier.to_string(),
        issued_at_unix: now.timestamp(),
        expires_at_unix: exp.timestamp(),
        signature_hex,
    }
}
