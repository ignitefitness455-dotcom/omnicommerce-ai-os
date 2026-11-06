use chrono::{Duration, Utc};
use omni_core::CurrencyCode;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WhatsAppFlowDispatchPayload {
    pub messaging_product: &'static str,
    pub recipient_type: &'static str,
    pub to: String,
    pub flow_token: String,
    pub language_code: &'static str,
    pub header_text: String,
    pub body_text: String,
    pub cta_button_label: &'static str,
    pub prepaid_instant_discount_pct: u8,
    pub prepaid_discounted_amount_major: f64,
    pub currency: CurrencyCode,
    pub expires_at_iso: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WhatsAppFlowBuyerReply {
    pub flow_token: String,
    pub order_id: Uuid,
    pub buyer_phone_e164: String,
    /// "CONFIRM_COD_ADDRESS" | "SWITCH_TO_PREPAID_SAVE_5PCT" | "CANCEL_ORDER"
    pub selected_action: String,
    pub building_or_makani_number: Option<String>,
    pub delivery_landmark: Option<String>,
    pub preferred_slot: Option<String>,
}

/// Builds a bilingual (Arabic for GCC `+966`/`+971`, English for SEA/Global) Meta WhatsApp Flow
/// message for Moderate/High-Risk COD orders, including a 5% COD-to-Prepaid conversion nudge.
pub fn build_cod_verification_whatsapp_flow(
    order_id: Uuid,
    recipient_phone_e164: &str,
    customer_name: &str,
    order_total_major: f64,
    currency: CurrencyCode,
) -> WhatsAppFlowDispatchPayload {
    let is_gcc_arabic = recipient_phone_e164.starts_with("+966")
        || recipient_phone_e164.starts_with("+971")
        || recipient_phone_e164.starts_with("+974");

    let flow_token = format!("omnicod_{}_{}", order_id.simple(), &Uuid::new_v4().simple().to_string()[..8]);
    let discounted_amount = ((order_total_major * 0.95) * 100.0).round() / 100.0;

    let (language_code, header_text, body_text, cta_button_label) = if is_gcc_arabic {
        (
            "ar",
            format!("تأكيد طلب الدفع عند الاستلام • {}", customer_name),
            format!(
                "مرحباً {}! لتسريع شحن طلبك بقيمة {:.2} {} عبر أرامكس، يرجى تأكيد رقم المبنى/العنوان أدناه، أو احصل على خصم فوري 5% ({:.2} {}) عند الدفع الإلكتروني الآن!",
                customer_name,
                order_total_major,
                currency.as_str(),
                discounted_amount,
                currency.as_str()
            ),
            "تأكيد العنوان والشحن",
        )
    } else {
        (
            "en",
            format!("Confirm Your Order • {}", customer_name),
            format!(
                "Hi {}! To dispatch your {:.2} {} COD order immediately, please tap below to confirm your building/unit number — or switch to Instant Prepaid now and save 5% (pay only {:.2} {})!",
                customer_name,
                order_total_major,
                currency.as_str(),
                discounted_amount,
                currency.as_str()
            ),
            "Confirm Address & Dispatch",
        )
    };

    WhatsAppFlowDispatchPayload {
        messaging_product: "whatsapp",
        recipient_type: "individual",
        to: recipient_phone_e164.to_string(),
        flow_token,
        language_code,
        header_text,
        body_text,
        cta_button_label,
        prepaid_instant_discount_pct: 5,
        prepaid_discounted_amount_major: discounted_amount,
        currency,
        expires_at_iso: (Utc::now() + Duration::hours(6)).to_rfc3339(),
    }
}
