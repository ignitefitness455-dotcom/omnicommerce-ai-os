use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddressQualityReport {
    /// 0 (Valid detailed address) to 100 (Severe gibberish / fake address)
    pub penalty_score: i32,
    pub is_gibberish: bool,
    pub has_building_or_street_number: bool,
    pub flags: Vec<String>,
}

/// Evaluates shipping address quality across English and Arabic strings to detect fake COD orders,
/// keyboard mashing ("asdfgh"), incomplete landmarks, or missing building/Makani numbers.
pub fn evaluate_address_quality(address: &str, city: Option<&str>) -> AddressQualityReport {
    let trimmed = address.trim();
    let lower = trimmed.to_lowercase();
    let mut penalty_score = 0;
    let mut flags = Vec::new();

    if trimmed.chars().count() < 12 {
        penalty_score += 45;
        flags.push("ADDRESS_TOO_SHORT".to_string());
    }

    let suspicious_tokens = [
        "asdf", "qwerty", "test", "fake", "none", "null", "xyz", "0000", "1111", "9999", "idk",
    ];
    let mut is_gibberish = false;
    for token in suspicious_tokens {
        if lower.contains(token) {
            penalty_score += 50;
            is_gibberish = true;
            flags.push(format!("SUSPICIOUS_TOKEN_DETECTED:{}", token.to_uppercase()));
            break;
        }
    }

    // Check for repeated identical characters (e.g., "aaaaaaa")
    let mut max_repeat = 1;
    let mut current_repeat = 1;
    let chars: Vec<char> = lower.chars().collect();
    for i in 1..chars.len() {
        if chars[i] == chars[i - 1] && chars[i].is_alphabetic() {
            current_repeat += 1;
            if current_repeat > max_repeat {
                max_repeat = current_repeat;
            }
        } else {
            current_repeat = 1;
        }
    }
    if max_repeat >= 4 {
        penalty_score += 40;
        is_gibberish = true;
        flags.push("REPEATED_CHAR_MASHING".to_string());
    }

    // Check whether address includes digits (building no, street no, flat, PO Box, Makani, or Arabic numerals)
    let has_building_or_street_number = trimmed
        .chars()
        .any(|c| c.is_ascii_digit() || ('\u{0660}'..='\u{0669}').contains(&c));

    if !has_building_or_street_number {
        penalty_score += 20;
        flags.push("MISSING_BUILDING_OR_UNIT_NUMBER".to_string());
    }

    if city.map(|c| c.trim().is_empty()).unwrap_or(true) {
        penalty_score += 15;
        flags.push("MISSING_CITY_OR_EMIRATE".to_string());
    }

    AddressQualityReport {
        penalty_score: penalty_score.clamp(0, 100),
        is_gibberish,
        has_building_or_street_number,
        flags,
    }
}
