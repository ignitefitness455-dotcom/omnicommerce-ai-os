use async_trait::async_trait;
use chrono::{DateTime, Utc};
use omni_core::{DomainError, Money};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum CarrierProvider {
    AramexGcc,
    NinjaVanSea,
    EasyPostGlobal,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum NormalizedShipmentStatus {
    LabelCreated,
    PickedUp,
    InTransit,
    OutForDelivery,
    Delivered,
    DeliveryFailedAttempt,
    RtoInitiated,
    RtoReturnedToMerchant,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShipmentRequest {
    pub order_id: Uuid,
    pub merchant_id: Uuid,
    pub recipient_name: String,
    pub recipient_phone_e164: String,
    pub address_line1: String,
    pub city: String,
    pub country_iso: String,
    pub postal_code: Option<String>,
    pub weight_grams: u32,
    pub is_cod: bool,
    pub cod_collection_amount: Option<Money>,
    pub item_description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShipmentBookingResponse {
    pub provider: CarrierProvider,
    pub tracking_number: String,
    pub awb_label_url: String,
    pub estimated_delivery_at: DateTime<Utc>,
    pub booked_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebhookTrackingUpdate {
    pub provider: CarrierProvider,
    pub tracking_number: String,
    pub normalized_status: NormalizedShipmentStatus,
    pub raw_carrier_code: String,
    pub rto_reason: Option<String>,
    pub occurred_at: DateTime<Utc>,
}

#[async_trait]
pub trait CarrierAdapter: Send + Sync {
    fn provider(&self) -> CarrierProvider;

    async fn create_shipment(
        &self,
        req: &ShipmentRequest,
    ) -> Result<ShipmentBookingResponse, DomainError>;

    async fn cancel_shipment(&self, tracking_number: &str) -> Result<bool, DomainError>;
}
