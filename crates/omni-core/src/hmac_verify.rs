use base64::{engine::general_purpose::STANDARD as BASE64_STANDARD, Engine as _};
use hmac::{Hmac, Mac};
use sha2::Sha256;

type HmacSha256 = Hmac<Sha256>;

/// Verifies Shopify `X-Shopify-Hmac-Sha256` webhook header (Base64-encoded HMAC-SHA256)
/// using constant-time comparison to prevent timing side-channel attacks.
pub fn verify_shopify_hmac_base64(raw_body: &[u8], secret: &str, header_signature: &str) -> bool {
    let Ok(expected_bytes) = BASE64_STANDARD.decode(header_signature.trim()) else {
        return false;
    };
    let Ok(mut mac) = HmacSha256::new_from_slice(secret.as_bytes()) else {
        return false;
    };
    mac.update(raw_body);
    mac.verify_slice(&expected_bytes).is_ok()
}

/// Verifies Salla (`X-Salla-Signature`) and Zid (`X-Zid-Signature`) webhook headers
/// (Hex-encoded HMAC-SHA256) using constant-time comparison.
pub fn verify_hex_hmac_sha256(raw_body: &[u8], secret: &str, header_signature: &str) -> bool {
    let clean_sig = header_signature
        .trim()
        .strip_prefix("sha256=")
        .unwrap_or(header_signature.trim());

    let Ok(expected_bytes) = hex::decode(clean_sig) else {
        return false;
    };
    let Ok(mut mac) = HmacSha256::new_from_slice(secret.as_bytes()) else {
        return false;
    };
    mac.update(raw_body);
    mac.verify_slice(&expected_bytes).is_ok()
}

/// Verifies Meta WhatsApp Cloud API `X-Hub-Signature-256` (`sha256=<hex>`) webhook header.
pub fn verify_meta_hub_signature(raw_body: &[u8], app_secret: &str, header_signature: &str) -> bool {
    if !header_signature.trim().starts_with("sha256=") {
        return false;
    }
    verify_hex_hmac_sha256(raw_body, app_secret, header_signature)
}
