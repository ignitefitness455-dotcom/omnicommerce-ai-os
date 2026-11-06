//! `omni-logistics`: Unified Multi-Carrier Logistics Engine for GCC (Aramex),
//! Southeast Asia (Ninja Van / J&T), and Global (EasyPost: DHL, FedEx, UPS).

pub mod adapters;
pub mod smart_router;
pub mod traits;

pub use adapters::{AramexAdapter, EasyPostAdapter, NinjaVanAdapter};
pub use smart_router::{
    select_optimal_carrier_for_zone, CarrierArbitrageDecision, CarrierArbitrageRequest,
    RankedCarrierOption,
};
pub use traits::{
    CarrierAdapter, CarrierProvider, NormalizedShipmentStatus, ShipmentBookingResponse,
    ShipmentRequest, WebhookTrackingUpdate,
};
