use serde::{Deserialize, Serialize};

/// Supported ISO 4217 Currency Codes across GCC, SEA, US/EU, and South Asia.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CurrencyCode {
    AED, // UAE Dirham
    SAR, // Saudi Riyal
    QAR, // Qatari Riyal
    KWD, // Kuwaiti Dinar
    MYR, // Malaysian Ringgit
    IDR, // Indonesian Rupiah
    SGD, // Singapore Dollar
    USD, // US Dollar
    EUR, // Euro
    GBP, // British Pound
    BDT, // Bangladeshi Taka
}

impl CurrencyCode {
    pub fn as_str(&self) -> &'static str {
        match self {
            CurrencyCode::AED => "AED",
            CurrencyCode::SAR => "SAR",
            CurrencyCode::QAR => "QAR",
            CurrencyCode::KWD => "KWD",
            CurrencyCode::MYR => "MYR",
            CurrencyCode::IDR => "IDR",
            CurrencyCode::SGD => "SGD",
            CurrencyCode::USD => "USD",
            CurrencyCode::EUR => "EUR",
            CurrencyCode::GBP => "GBP",
            CurrencyCode::BDT => "BDT",
        }
    }

    /// Number of decimal places (minor units) per ISO 4217.
    pub fn minor_unit_exponent(&self) -> u32 {
        match self {
            CurrencyCode::KWD => 3,
            CurrencyCode::IDR => 0,
            _ => 2,
        }
    }
}

/// Exact monetary representation stored in minor units (fils, halalas, cents, sen).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Money {
    pub amount_minor: i64,
    pub currency: CurrencyCode,
}

impl Money {
    pub fn new(amount_minor: i64, currency: CurrencyCode) -> Self {
        Self {
            amount_minor,
            currency,
        }
    }

    pub fn to_major_f64(&self) -> f64 {
        let divisor = 10_f64.powi(self.currency.minor_unit_exponent() as i32);
        (self.amount_minor as f64) / divisor
    }
}
