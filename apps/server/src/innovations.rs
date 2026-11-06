use crate::AppState;
use axum::{routing::post, Json, Router};
use omni_ai::{evaluate_autonomous_negotiation, NegotiationDecision, NegotiationInput};
use omni_logistics::{
    select_optimal_carrier_for_zone, CarrierArbitrageDecision, CarrierArbitrageRequest,
};
use std::sync::Arc;

pub fn innovations_router(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/negotiate", post(autonomous_negotiation_handler))
        .route("/carrier-arbitrage", post(carrier_arbitrage_handler))
        .with_state(state)
}

async fn autonomous_negotiation_handler(
    Json(payload): Json<NegotiationInput>,
) -> Json<NegotiationDecision> {
    Json(evaluate_autonomous_negotiation(&payload))
}

async fn carrier_arbitrage_handler(
    Json(payload): Json<CarrierArbitrageRequest>,
) -> Json<CarrierArbitrageDecision> {
    Json(select_optimal_carrier_for_zone(&payload))
}
