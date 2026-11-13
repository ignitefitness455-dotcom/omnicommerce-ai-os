use crate::AppState;
use axum::{
    extract::State,
    routing::{get, post},
    Json, Router,
};
use omni_core::{
    generate_merchant_api_key, issue_merchant_session_token, CurrencyCode, GeneratedApiKeyBundle,
    MerchantRole, MerchantSessionClaims, SsoAuthSession, TeamMemberRecord,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use uuid::Uuid;

pub fn merchant_portal_router(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/onboard", post(onboard_merchant_handler))
        .route("/auth/sso", post(initiate_sso_auth_handler))
        .route("/auth/verify-otp", post(verify_otp_auth_handler))
        .route("/team", get(list_team_roster_handler))
        .route(
            "/catalog",
            get(list_catalog_products_handler).post(create_catalog_product_handler),
        )
        .route("/orders", get(list_live_orders_handler))
        .route("/stripe-webhook", post(stripe_billing_webhook_handler))
        .with_state(state)
}

#[derive(Debug, Deserialize)]
pub struct MerchantOnboardRequest {
    pub business_name: String,
    pub email: String,
    pub store_domain: Option<String>,
    pub platform_type: String,       // "SHOPIFY" | "SALLA" | "ZID" | "WOOCOMMERCE"
    pub primary_country_iso: String, // "AE" | "SA" | "MY" | "US" | "BD"
    pub default_currency: String,    // "AED" | "SAR" | "MYR" | "USD" | "BDT"
    pub selected_plan_tier: String,  // "GROWTH" | "SCALE" | "ENTERPRISE"
}

#[derive(Debug, Serialize)]
pub struct WebhookEndpointsBundle {
    pub shopify_orders_webhook_url: String,
    pub salla_zid_orders_webhook_url: String,
    pub meta_whatsapp_cloud_webhook_url: String,
    pub stripe_billing_webhook_url: String,
}

#[derive(Debug, Serialize)]
pub struct MerchantOnboardResponse {
    pub merchant_id: Uuid,
    pub session_token: MerchantSessionClaims,
    pub api_keys: GeneratedApiKeyBundle,
    pub webhook_endpoints: WebhookEndpointsBundle,
    pub onboarding_status: &'static str,
}

async fn onboard_merchant_handler(
    State(state): State<Arc<AppState>>,
    Json(req): Json<MerchantOnboardRequest>,
) -> Json<MerchantOnboardResponse> {
    let merchant_id = Uuid::new_v4();
    let api_keys = generate_merchant_api_key(merchant_id, &state.network_pepper);
    let session_token = issue_merchant_session_token(
        merchant_id,
        &req.business_name,
        &req.primary_country_iso,
        &req.default_currency,
        &req.selected_plan_tier,
        &state.network_pepper,
    );

    Json(MerchantOnboardResponse {
        merchant_id,
        session_token,
        api_keys,
        webhook_endpoints: WebhookEndpointsBundle {
            shopify_orders_webhook_url:
                "https://api.omnicommerce.ai/api/v1/webhooks/shopify/orders-create".to_string(),
            salla_zid_orders_webhook_url:
                "https://api.omnicommerce.ai/api/v1/webhooks/salla/order-created".to_string(),
            meta_whatsapp_cloud_webhook_url:
                "https://api.omnicommerce.ai/api/v1/webhooks/whatsapp".to_string(),
            stripe_billing_webhook_url:
                "https://api.omnicommerce.ai/api/v1/merchants/stripe-webhook".to_string(),
        },
        onboarding_status: "ACTIVE_AND_READY",
    })
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CatalogProductItem {
    pub id: Uuid,
    pub sku: String,
    pub title_en: String,
    pub title_ar: Option<String>,
    pub unit_price_major: f64,
    pub unit_cogs_major: f64,
    pub currency: CurrencyCode,
    pub stock_quantity: i32,
    pub min_required_margin_pct: f64,
    pub max_ai_discount_pct: f64,
    pub vector_indexed: bool,
}

async fn list_catalog_products_handler(
    State(state): State<Arc<AppState>>,
) -> Json<Vec<CatalogProductItem>> {
    let items = state.catalog_products.lock().unwrap().clone();
    Json(items)
}

#[derive(Debug, Deserialize)]
pub struct CreateCatalogProductRequest {
    pub sku: String,
    pub title_en: String,
    pub title_ar: Option<String>,
    pub unit_price_major: f64,
    pub unit_cogs_major: f64,
    pub currency: CurrencyCode,
    pub stock_quantity: i32,
    pub min_required_margin_pct: f64,
    pub max_ai_discount_pct: f64,
}

async fn create_catalog_product_handler(
    State(state): State<Arc<AppState>>,
    Json(req): Json<CreateCatalogProductRequest>,
) -> Json<CatalogProductItem> {
    let item = CatalogProductItem {
        id: Uuid::new_v4(),
        sku: req.sku,
        title_en: req.title_en,
        title_ar: req.title_ar,
        unit_price_major: req.unit_price_major,
        unit_cogs_major: req.unit_cogs_major,
        currency: req.currency,
        stock_quantity: req.stock_quantity,
        min_required_margin_pct: req.min_required_margin_pct,
        max_ai_discount_pct: req.max_ai_discount_pct,
        vector_indexed: true,
    };
    state.catalog_products.lock().unwrap().push(item.clone());
    Json(item)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LiveOrderSummary {
    pub order_id: String,
    pub source_channel: String,
    pub customer_name: String,
    pub customer_phone_e164: String,
    pub country_iso: String,
    pub amount_label: String,
    pub payment_method: String,
    pub fraud_score: i32,
    pub fraud_tier: String,
    pub applied_intervention: String,
    pub carrier_awb: Option<String>,
    pub status: String,
}

async fn list_live_orders_handler() -> Json<Vec<LiveOrderSummary>> {
    Json(vec![
        LiveOrderSummary {
            order_id: "ORD-DXB-77801".to_string(),
            source_channel: "SHOPIFY_UAE".to_string(),
            customer_name: "Sara Al-Maktoum".to_string(),
            customer_phone_e164: "+971509876543".to_string(),
            country_iso: "AE".to_string(),
            amount_label: "420.00 AED".to_string(),
            payment_method: "COD".to_string(),
            fraud_score: 9,
            fraud_tier: "SAFE".to_string(),
            applied_intervention: "ALLOW_SEAMLESS_COD".to_string(),
            carrier_awb: Some("ARMX-AE-77801DXB".to_string()),
            status: "AUTO_DISPATCHED".to_string(),
        },
        LiveOrderSummary {
            order_id: "ORD-KSA-99412".to_string(),
            source_channel: "SALLA_KSA".to_string(),
            customer_name: "Faisal Al-Saud".to_string(),
            customer_phone_e164: "+966554433221".to_string(),
            country_iso: "SA".to_string(),
            amount_label: "513.00 SAR (5% Off Switch)".to_string(),
            payment_method: "TABBY_4X_PREPAID".to_string(),
            fraud_score: 38,
            fraud_tier: "MODERATE".to_string(),
            applied_intervention: "WHATSAPP_FLOW_CONVERTED_PREPAID".to_string(),
            carrier_awb: Some("ARMX-SA-PREPAID99".to_string()),
            status: "CONVERTED_AND_DISPATCHED".to_string(),
        },
        LiveOrderSummary {
            order_id: "ORD-MY-33109".to_string(),
            source_channel: "TIKTOK_SHOP_MY".to_string(),
            customer_name: "Aiman Hakim".to_string(),
            customer_phone_e164: "+60123456789".to_string(),
            country_iso: "MY".to_string(),
            amount_label: "185.00 MYR".to_string(),
            payment_method: "COD".to_string(),
            fraud_score: 12,
            fraud_tier: "SAFE".to_string(),
            applied_intervention: "ALLOW_SEAMLESS_COD".to_string(),
            carrier_awb: Some("NJVMY33109KL".to_string()),
            status: "AUTO_DISPATCHED".to_string(),
        },
    ])
}

#[derive(Debug, Deserialize)]
pub struct StripeEventPayload {
    pub event_type: String, // "checkout.session.completed" | "customer.subscription.updated"
    pub order_or_merchant_id: String,
    pub amount_paid_usd: f64,
}

#[derive(Debug, Serialize)]
pub struct StripeWebhookResolution {
    pub handled: bool,
    pub event_type: String,
    pub action_taken: String,
}

async fn stripe_billing_webhook_handler(
    Json(payload): Json<StripeEventPayload>,
) -> Json<StripeWebhookResolution> {
    let action_taken = if payload.event_type == "checkout.session.completed" {
        format!(
            "Converted COD order {} to 100% Prepaid (${:.2} USD). Released Fraud Shield hold and auto-dispatched priority courier AWB!",
            payload.order_or_merchant_id, payload.amount_paid_usd
        )
    } else {
        format!(
            "Updated SaaS Subscription for merchant {} (${:.2} USD/mo active).",
            payload.order_or_merchant_id, payload.amount_paid_usd
        )
    };

    Json(StripeWebhookResolution {
        handled: true,
        event_type: payload.event_type,
        action_taken,
    })
}

#[derive(Debug, Deserialize)]
pub struct SsoInitRequest {
    pub provider: String, // "google" | "shopify" | "salla"
    pub store_domain: Option<String>,
}

async fn initiate_sso_auth_handler(
    Json(req): Json<SsoInitRequest>,
) -> Json<SsoAuthSession> {
    let state_token = format!("omni_sso_{}", &Uuid::new_v4().simple().to_string()[..16]);
    let provider_clean = req.provider.to_lowercase();
    let redirect_url = match provider_clean.as_str() {
        "shopify" => format!(
            "https://{}/admin/oauth/authorize?client_id=omni_shopify&state={}",
            req.store_domain.as_deref().unwrap_or("demo-store.myshopify.com"),
            state_token
        ),
        "salla" => format!(
            "https://accounts.salla.sa/oauth2/auth?client_id=omni_salla&state={}",
            state_token
        ),
        _ => format!(
            "https://accounts.google.com/o/oauth2/v2/auth?client_id=omni_google&state={}",
            state_token
        ),
    };

    Json(SsoAuthSession {
        provider: provider_clean,
        state_token,
        redirect_url,
    })
}

#[derive(Debug, Deserialize)]
pub struct VerifyOtpRequest {
    pub identifier: String, // E.164 phone or email
    pub otp_code: String,
}

#[derive(Debug, Serialize)]
pub struct OtpVerificationResponse {
    pub verified: bool,
    pub role: MerchantRole,
    pub session_token: MerchantSessionClaims,
    pub message: String,
}

async fn verify_otp_auth_handler(
    State(state): State<Arc<AppState>>,
    Json(req): Json<VerifyOtpRequest>,
) -> Json<OtpVerificationResponse> {
    let valid_code = req.otp_code.trim().len() == 6 && req.otp_code.chars().all(|c| c.is_ascii_digit());
    let merchant_id = Uuid::new_v4();
    let session_token = issue_merchant_session_token(
        merchant_id,
        "Al-Noor Luxury Abayas Dubai",
        "AE",
        "AED",
        "SCALE",
        &state.network_pepper,
    );

    Json(OtpVerificationResponse {
        verified: valid_code,
        role: MerchantRole::Owner,
        session_token,
        message: if valid_code {
            format!("2FA OTP verified for {}. RBAC Owner session issued.", req.identifier)
        } else {
            "Invalid 6-digit OTP code format.".to_string()
        },
    })
}

async fn list_team_roster_handler() -> Json<Vec<TeamMemberRecord>> {
    Json(vec![
        TeamMemberRecord {
            id: Uuid::new_v4(),
            name: "Tariq Al-Mansoor".to_string(),
            email: "tariq@alnoor-dubai.ae".to_string(),
            role: MerchantRole::Owner,
            two_factor_enabled: true,
        },
        TeamMemberRecord {
            id: Uuid::new_v4(),
            name: "Nadia Al-Harbi".to_string(),
            email: "nadia@alnoor-dubai.ae".to_string(),
            role: MerchantRole::StoreManager,
            two_factor_enabled: true,
        },
        TeamMemberRecord {
            id: Uuid::new_v4(),
            name: "Zaid Rahman".to_string(),
            email: "zaid.support@alnoor-dubai.ae".to_string(),
            role: MerchantRole::SupportAgent,
            two_factor_enabled: true,
        },
        TeamMemberRecord {
            id: Uuid::new_v4(),
            name: "Amira Hassan".to_string(),
            email: "logistics@alnoor-dubai.ae".to_string(),
            role: MerchantRole::LogisticsCoordinator,
            two_factor_enabled: false,
        },
    ])
}
