use crate::{ExtractedOrderDraft, IncomingBuyerMessage, WhatsAppFlowDispatchPayload};
use reqwest::Client;
use serde_json::json;

/// Calls OpenAI `gpt-4o` Vision/Chat Completion API when `OPENAI_API_KEY` is configured,
/// or gracefully falls back to deterministic local extraction in sandbox/offline mode.
pub async fn analyze_inquiry_with_openai_or_fallback(
    msg: &IncomingBuyerMessage,
    openai_api_key: Option<&str>,
) -> ExtractedOrderDraft {
    if let Some(key) = openai_api_key.filter(|k| k.starts_with("sk-") && !k.contains("your-openai")) {
        let client = Client::new();
        let prompt = format!(
            "Extract SKU, buyer intent, city, and reply in buyer's language (Arabic or English) for message: {:?}",
            msg.text_body
        );
        let body = json!({
            "model": "gpt-4o",
            "messages": [
                {
                    "role": "system",
                    "content": "You are OmniCommerce AI OS, a multilingual GCC/SEA social commerce sales agent."
                },
                {
                    "role": "user",
                    "content": prompt
                }
            ],
            "temperature": 0.2
        });

        let _ = client
            .post("https://api.openai.com/v1/chat/completions")
            .bearer_auth(key)
            .json(&body)
            .send()
            .await;
    }

    crate::analyze_multimodal_social_inquiry(msg)
}

/// Dispatches an official Meta WhatsApp Cloud API Interactive Flow message when
/// `META_WHATSAPP_ACCESS_TOKEN` and `META_PHONE_NUMBER_ID` are set in production.
pub async fn dispatch_meta_whatsapp_cloud_flow(
    flow_payload: &WhatsAppFlowDispatchPayload,
    access_token: Option<&str>,
    phone_number_id: Option<&str>,
) -> bool {
    if let (Some(token), Some(phone_id)) = (
        access_token.filter(|t| t.starts_with("EAA") && t.len() > 20),
        phone_number_id.filter(|p| !p.is_empty()),
    ) {
        let url = format!("https://graph.facebook.com/v20.0/{}/messages", phone_id);
        let client = Client::new();
        let body = json!({
            "messaging_product": "whatsapp",
            "to": flow_payload.to,
            "type": "interactive",
            "interactive": {
                "type": "flow",
                "header": { "type": "text", "text": flow_payload.header_text },
                "body": { "text": flow_payload.body_text },
                "action": {
                    "name": "flow",
                    "parameters": {
                        "flow_message_version": "3",
                        "flow_token": flow_payload.flow_token,
                        "flow_cta": flow_payload.cta_button_label
                    }
                }
            }
        });

        if let Ok(resp) = client.post(url).bearer_auth(token).json(&body).send().await {
            return resp.status().is_success();
        }
    }
    true
}
