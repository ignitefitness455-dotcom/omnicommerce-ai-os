use hmac::{Hmac, Mac};
use serde::{Deserialize, Serialize};
use sha2::Sha256;

type HmacSha256 = Hmac<Sha256>;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PhoneMetadata {
    pub e164: String,
    pub country_iso: &'static str,
    pub region_group: &'static str,
}

/// Normalizes an international phone string into strict E.164 format (`+<country><subscriber>`)
/// and identifies the target market region (GCC, SEA, US_EU, SOUTH_ASIA, GLOBAL).
pub fn normalize_e164_phone(raw: &str, default_country_dial: Option<&str>) -> Result<PhoneMetadata, String> {
    let digits_and_plus: String = raw
        .chars()
        .filter(|c| c.is_ascii_digit() || *c == '+')
        .collect();

    if digits_and_plus.is_empty() {
        return Err("Phone number cannot be empty".to_string());
    }

    let cleaned = if let Some(stripped) = digits_and_plus.strip_prefix("00") {
        format!("+{}", stripped)
    } else if digits_and_plus.starts_with('+') {
        digits_and_plus
    } else if let Some(dial) = default_country_dial {
        let local = digits_and_plus.trim_start_matches('0');
        let dial_clean = dial.trim_start_matches('+');
        format!("+{}{}", dial_clean, local)
    } else {
        format!("+{}", digits_and_plus)
    };

    let digit_len = cleaned.len().saturating_sub(1);
    if !(8..=15).contains(&digit_len) {
        return Err(format!(
            "Invalid E.164 phone length ({} digits): must be between 8 and 15 digits",
            digit_len
        ));
    }

    let (country_iso, region_group) = if cleaned.starts_with("+971") {
        ("AE", "GCC")
    } else if cleaned.starts_with("+966") {
        ("SA", "GCC")
    } else if cleaned.starts_with("+974") {
        ("QA", "GCC")
    } else if cleaned.starts_with("+965") {
        ("KW", "GCC")
    } else if cleaned.starts_with("+973") {
        ("BH", "GCC")
    } else if cleaned.starts_with("+968") {
        ("OM", "GCC")
    } else if cleaned.starts_with("+60") {
        ("MY", "SEA")
    } else if cleaned.starts_with("+62") {
        ("ID", "SEA")
    } else if cleaned.starts_with("+65") {
        ("SG", "SEA")
    } else if cleaned.starts_with("+63") {
        ("PH", "SEA")
    } else if cleaned.starts_with("+66") {
        ("TH", "SEA")
    } else if cleaned.starts_with("+880") {
        ("BD", "SOUTH_ASIA")
    } else if cleaned.starts_with("+91") {
        ("IN", "SOUTH_ASIA")
    } else if cleaned.starts_with("+44") {
        ("GB", "US_EU")
    } else if cleaned.starts_with("+1") {
        ("US", "US_EU")
    } else {
        ("ZZ", "GLOBAL")
    };

    Ok(PhoneMetadata {
        e164: cleaned,
        country_iso,
        region_group,
    })
}

/// Computes a zero-knowledge HMAC-SHA256 fingerprint for cross-merchant fraud intelligence.
/// Complies with GDPR (EU) and Saudi PDPL by preventing raw PII leakage across tenants.
pub fn compute_network_fingerprint(e164_phone: &str, network_pepper: &str) -> String {
    let mut mac = HmacSha256::new_from_slice(network_pepper.as_bytes())
        .expect("HMAC accepts keys of any size");
    mac.update(e164_phone.trim().as_bytes());
    let result = mac.finalize();
    hex::encode(result.into_bytes())
}
