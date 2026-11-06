//! `omni-core`: Core domain primitives, GDPR/Saudi PDPL cryptographic PII hashing,
//! E.164 international phone normalization, ISO 4217 multi-currency types,
//! constant-time webhook HMAC verifiers, and RFC 9457 errors.

pub mod billing;
pub mod crypto;
pub mod currency;
pub mod error;
pub mod hmac_verify;

pub use billing::{
    calculate_merchant_saas_roi, MerchantRoiInput, MerchantRoiReport, SaasPlanTier,
};
pub use crypto::{compute_network_fingerprint, normalize_e164_phone, PhoneMetadata};
pub use currency::{CurrencyCode, Money};
pub use error::{DomainError, ProblemDetails};
pub use hmac_verify::{
    verify_hex_hmac_sha256, verify_meta_hub_signature, verify_shopify_hmac_base64,
};
