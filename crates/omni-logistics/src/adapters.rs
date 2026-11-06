use crate::traits::{
    CarrierAdapter, CarrierProvider, ShipmentBookingResponse, ShipmentRequest,
};
use async_trait::async_trait;
use chrono::{Duration, Utc};
use omni_core::DomainError;

/// Aramex GCC Shipping Adapter (UAE, Saudi Arabia, Qatar, Kuwait, Bahrain, Oman)
pub struct AramexAdapter {
    pub account_number: String,
    pub account_entity: String,
}

impl AramexAdapter {
    pub fn new(account_number: impl Into<String>, account_entity: impl Into<String>) -> Self {
        Self {
            account_number: account_number.into(),
            account_entity: account_entity.into(),
        }
    }
}

#[async_trait]
impl CarrierAdapter for AramexAdapter {
    fn provider(&self) -> CarrierProvider {
        CarrierProvider::AramexGcc
    }

    async fn create_shipment(
        &self,
        req: &ShipmentRequest,
    ) -> Result<ShipmentBookingResponse, DomainError> {
        let suffix = &req.order_id.simple().to_string()[..8].to_uppercase();
        let tracking_number = format!("ARMX-{}-{}", req.country_iso, suffix);
        Ok(ShipmentBookingResponse {
            provider: CarrierProvider::AramexGcc,
            awb_label_url: format!(
                "https://labels.omnicommerce.ai/aramex/{}.pdf",
                tracking_number
            ),
            tracking_number,
            estimated_delivery_at: Utc::now() + Duration::hours(36),
            booked_at: Utc::now(),
        })
    }

    async fn cancel_shipment(&self, _tracking_number: &str) -> Result<bool, DomainError> {
        Ok(true)
    }
}

/// Ninja Van Southeast Asia Adapter (Malaysia, Indonesia, Singapore, Philippines, Thailand)
pub struct NinjaVanAdapter {
    pub country_code: String,
}

impl NinjaVanAdapter {
    pub fn new(country_code: impl Into<String>) -> Self {
        Self {
            country_code: country_code.into(),
        }
    }
}

#[async_trait]
impl CarrierAdapter for NinjaVanAdapter {
    fn provider(&self) -> CarrierProvider {
        CarrierProvider::NinjaVanSea
    }

    async fn create_shipment(
        &self,
        req: &ShipmentRequest,
    ) -> Result<ShipmentBookingResponse, DomainError> {
        let suffix = &req.order_id.simple().to_string()[..8].to_uppercase();
        let tracking_number = format!("NJV{}{}", req.country_iso, suffix);
        Ok(ShipmentBookingResponse {
            provider: CarrierProvider::NinjaVanSea,
            awb_label_url: format!(
                "https://labels.omnicommerce.ai/ninjavan/{}.pdf",
                tracking_number
            ),
            tracking_number,
            estimated_delivery_at: Utc::now() + Duration::hours(48),
            booked_at: Utc::now(),
        })
    }

    async fn cancel_shipment(&self, _tracking_number: &str) -> Result<bool, DomainError> {
        Ok(true)
    }
}

/// EasyPost Global Multi-Carrier Aggregator (DHL Express, FedEx, UPS, USPS)
pub struct EasyPostAdapter;

#[async_trait]
impl CarrierAdapter for EasyPostAdapter {
    fn provider(&self) -> CarrierProvider {
        CarrierProvider::EasyPostGlobal
    }

    async fn create_shipment(
        &self,
        req: &ShipmentRequest,
    ) -> Result<ShipmentBookingResponse, DomainError> {
        let suffix = &req.order_id.simple().to_string()[..8].to_uppercase();
        let tracking_number = format!("EZP-DHL-{}-{}", req.country_iso, suffix);
        Ok(ShipmentBookingResponse {
            provider: CarrierProvider::EasyPostGlobal,
            awb_label_url: format!(
                "https://labels.omnicommerce.ai/easypost/{}.pdf",
                tracking_number
            ),
            tracking_number,
            estimated_delivery_at: Utc::now() + Duration::hours(72),
            booked_at: Utc::now(),
        })
    }

    async fn cancel_shipment(&self, _tracking_number: &str) -> Result<bool, DomainError> {
        Ok(true)
    }
}
