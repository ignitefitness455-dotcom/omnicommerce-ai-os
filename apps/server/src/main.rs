mod innovations;
mod merchant_portal;
mod phase4_growth;
mod routes;
mod webhooks;

use axum::{routing::get, Json, Router};
use merchant_portal::CatalogProductItem;
use omni_core::CurrencyCode;
use serde_json::{json, Value};
use std::{
    collections::HashMap,
    env,
    net::SocketAddr,
    sync::{Arc, Mutex},
};
use tower_http::cors::CorsLayer;
use tower_http::trace::TraceLayer;
use uuid::Uuid;
use webhooks::WebhookOrchestrationResult;

pub struct AppState {
    pub network_pepper: String,
    pub environment: String,
    pub shopify_webhook_secret: String,
    pub salla_webhook_secret: String,
    pub meta_verify_token: String,
    pub idempotency_cache: Mutex<HashMap<String, WebhookOrchestrationResult>>,
    pub catalog_products: Mutex<Vec<CatalogProductItem>>,
}

#[tokio::main]
async fn main() {
    let _ = dotenvy::dotenv();

    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "omni_server=info,tower_http=info".into()),
        )
        .init();

    let state = Arc::new(AppState {
        network_pepper: env::var("NETWORK_PII_PEPPER")
            .unwrap_or_else(|_| "dev_global_pepper_secret_key_change_in_prod".to_string()),
        environment: env::var("APP_ENV").unwrap_or_else(|_| "development".to_string()),
        shopify_webhook_secret: env::var("SHOPIFY_WEBHOOK_SECRET")
            .unwrap_or_else(|_| "shpss_dev_secret".to_string()),
        salla_webhook_secret: env::var("SALLA_WEBHOOK_SECRET")
            .unwrap_or_else(|_| "salla_dev_secret".to_string()),
        meta_verify_token: env::var("META_WHATSAPP_VERIFY_TOKEN")
            .unwrap_or_else(|_| "omni_whatsapp_verify_token".to_string()),
        idempotency_cache: Mutex::new(HashMap::new()),
        catalog_products: Mutex::new(vec![
            CatalogProductItem {
                id: Uuid::new_v4(),
                sku: "OMNI-GCC-ABAYA-902".to_string(),
                title_en: "Oud Royal Velvet Abaya — Limited Edition".to_string(),
                title_ar: Some("عباية مخمل ملكي فاخرة".to_string()),
                unit_price_major: 420.0,
                unit_cogs_major: 240.0,
                currency: CurrencyCode::AED,
                stock_quantity: 18,
                min_required_margin_pct: 28.0,
                max_ai_discount_pct: 12.0,
                vector_indexed: true,
            },
            CatalogProductItem {
                id: Uuid::new_v4(),
                sku: "OMNI-KSA-OUD-501".to_string(),
                title_en: "Royal Cambodian Oud Elixir 100ml".to_string(),
                title_ar: Some("دهن عود كمبودي ملكي".to_string()),
                unit_price_major: 540.0,
                unit_cogs_major: 290.0,
                currency: CurrencyCode::SAR,
                stock_quantity: 32,
                min_required_margin_pct: 30.0,
                max_ai_discount_pct: 10.0,
                vector_indexed: true,
            },
        ]),
    });

    let app = Router::new()
        .route("/health", get(health_check))
        .nest("/api/v1", routes::api_v1_router(state))
        .layer(CorsLayer::permissive())
        .layer(TraceLayer::new_for_http());

    let port: u16 = env::var("SERVER_PORT")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(8080);

    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    tracing::info!("🚀 OmniCommerce AI OS Global API listening on http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .expect("Failed to bind TCP listener");
    axum::serve(listener, app).await.expect("Server crashed");
}

async fn health_check() -> Json<Value> {
    Json(json!({
        "status": "healthy",
        "service": "omnicommerce-ai-os-server",
        "phase": "Phase 2 — Storefront Webhooks & WhatsApp Flows Active",
        "regions_supported": ["GCC", "SEA", "US_EU", "SOUTH_ASIA"],
        "privacy_compliance": ["GDPR", "SAUDI_PDPL", "CCPA"]
    }))
}
