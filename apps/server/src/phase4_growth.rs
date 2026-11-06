use crate::AppState;
use axum::{routing::post, Json, Router};
use omni_ai::{
    build_fraud_filtered_retargeting_plan, AbandonedChatInput, RetargetingCampaignPlan,
};
use omni_core::{calculate_merchant_saas_roi, MerchantRoiInput, MerchantRoiReport};
use std::sync::Arc;

pub fn phase4_growth_router(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/retargeting/plan", post(retargeting_plan_handler))
        .route("/billing/roi-report", post(merchant_roi_report_handler))
        .with_state(state)
}

async fn retargeting_plan_handler(
    Json(payload): Json<AbandonedChatInput>,
) -> Json<RetargetingCampaignPlan> {
    Json(build_fraud_filtered_retargeting_plan(&payload))
}

async fn merchant_roi_report_handler(
    Json(payload): Json<MerchantRoiInput>,
) -> Json<MerchantRoiReport> {
    Json(calculate_merchant_saas_roi(&payload))
}
