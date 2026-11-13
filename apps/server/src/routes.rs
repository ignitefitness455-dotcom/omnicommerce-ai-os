use crate::AppState;
use axum::{
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::post,
    Json, Router,
};
use omni_ai::{analyze_multimodal_social_inquiry, ExtractedOrderDraft, IncomingBuyerMessage};
use omni_core::{compute_network_fingerprint, normalize_e164_phone, DomainError};
use omni_fraud_engine::{
    evaluate_global_fraud_risk, FraudEvaluationInput, FraudRiskAssessment,
};
use omni_logistics::{
    AramexAdapter, CarrierAdapter, CarrierProvider, EasyPostAdapter, NinjaVanAdapter,
    ShipmentBookingResponse, ShipmentRequest,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

pub fn api_v1_router(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/fraud/evaluate", post(evaluate_fraud_handler))
        .route("/ai/analyze-inquiry", post(analyze_social_inquiry_handler))
        .route("/logistics/book", post(book_shipment_handler))
        .nest("/webhooks", crate::webhooks::webhooks_router(state.clone()))
        .nest("/innovations", crate::innovations::innovations_router(state.clone()))
        .nest("/growth", crate::phase4_growth::phase4_growth_router(state.clone()))
        .nest("/merchants", crate::merchant_portal::merchant_portal_router(state.clone()))
        .nest("/events", crate::realtime_sse::events_router(state.clone()))
        .with_state(state)
}

pub struct ApiError {
    pub err: DomainError,
    pub instance: String,
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let problem = self.err.to_problem_details(Some(self.instance));
        let status = StatusCode::from_u16(problem.status).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);
        (status, Json(problem)).into_response()
    }
}

#[derive(Debug, Deserialize)]
pub struct FraudCheckRequestDto {
    pub phone_raw: String,
    pub default_country_dial: Option<String>,
    pub shipping_address: String,
    pub shipping_city: Option<String>,
    pub order_amount_usd_equiv: f64,
    pub total_network_orders: Option<i32>,
    pub total_network_delivered: Option<i32>,
    pub total_network_rto_returned: Option<i32>,
    pub total_network_chargebacks: Option<i32>,
    pub recent_90d_incidents: Option<i32>,
    pub orders_last_2_hours_velocity: Option<i32>,
    pub is_manually_blacklisted: Option<bool>,
    pub postal_cluster_rto_rate: Option<f64>,
}

#[derive(Debug, Serialize)]
pub struct FraudCheckResponseDto {
    pub e164_normalized: String,
    pub country_iso: String,
    pub region_group: String,
    pub assessment: FraudRiskAssessment,
}

async fn evaluate_fraud_handler(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<FraudCheckRequestDto>,
) -> Result<Json<FraudCheckResponseDto>, ApiError> {
    let phone_meta = normalize_e164_phone(&payload.phone_raw, payload.default_country_dial.as_deref())
        .map_err(|e| ApiError {
            err: DomainError::Validation(e),
            instance: "/api/v1/fraud/evaluate".to_string(),
        })?;

    let phone_hash = compute_network_fingerprint(&phone_meta.e164, &state.network_pepper);

    let eval_input = FraudEvaluationInput {
        phone_hash,
        country_iso: phone_meta.country_iso.to_string(),
        region_group: phone_meta.region_group.to_string(),
        total_network_orders: payload.total_network_orders.unwrap_or(4),
        total_network_delivered: payload.total_network_delivered.unwrap_or(4),
        total_network_rto_returned: payload.total_network_rto_returned.unwrap_or(0),
        total_network_chargebacks: payload.total_network_chargebacks.unwrap_or(0),
        recent_90d_incidents: payload.recent_90d_incidents.unwrap_or(0),
        orders_last_2_hours_velocity: payload.orders_last_2_hours_velocity.unwrap_or(1),
        is_manually_blacklisted: payload.is_manually_blacklisted.unwrap_or(false),
        shipping_address: payload.shipping_address,
        shipping_city: payload.shipping_city,
        postal_cluster_rto_rate: payload.postal_cluster_rto_rate.unwrap_or(12.5),
        order_amount_usd_equiv: payload.order_amount_usd_equiv,
    };

    let assessment = evaluate_global_fraud_risk(&eval_input);

    Ok(Json(FraudCheckResponseDto {
        e164_normalized: phone_meta.e164,
        country_iso: phone_meta.country_iso.to_string(),
        region_group: phone_meta.region_group.to_string(),
        assessment,
    }))
}

async fn analyze_social_inquiry_handler(
    Json(payload): Json<IncomingBuyerMessage>,
) -> Result<Json<ExtractedOrderDraft>, ApiError> {
    let draft = analyze_multimodal_social_inquiry(&payload);
    Ok(Json(draft))
}

#[derive(Debug, Deserialize)]
pub struct BookShipmentRequestDto {
    pub preferred_carrier: Option<CarrierProvider>,
    pub shipment: ShipmentRequest,
}

async fn book_shipment_handler(
    Json(payload): Json<BookShipmentRequestDto>,
) -> Result<Json<ShipmentBookingResponse>, ApiError> {
    let country = payload.shipment.country_iso.to_uppercase();
    let carrier = payload.preferred_carrier.unwrap_or_else(|| {
        if ["AE", "SA", "QA", "KW", "BH", "OM"].contains(&country.as_str()) {
            CarrierProvider::AramexGcc
        } else if ["MY", "ID", "SG", "PH", "TH"].contains(&country.as_str()) {
            CarrierProvider::NinjaVanSea
        } else {
            CarrierProvider::EasyPostGlobal
        }
    });

    let booking = match carrier {
        CarrierProvider::AramexGcc => {
            let adapter = AramexAdapter::new("20016", "DXB");
            adapter.create_shipment(&payload.shipment).await
        }
        CarrierProvider::NinjaVanSea => {
            let adapter = NinjaVanAdapter::new(&country);
            adapter.create_shipment(&payload.shipment).await
        }
        CarrierProvider::EasyPostGlobal => {
            let adapter = EasyPostAdapter;
            adapter.create_shipment(&payload.shipment).await
        }
    }
    .map_err(|err| ApiError {
        err,
        instance: "/api/v1/logistics/book".to_string(),
    })?;

    Ok(Json(booking))
}
