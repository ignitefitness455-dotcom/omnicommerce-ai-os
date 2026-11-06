---
name: omnichannel-social-commerce-gcc-sea
description: Architecture patterns for WhatsApp Cloud API, WhatsApp Flows, Instagram DM, TikTok Shop, Shopify/Salla/Zid webhooks, GPT-4o Vision/Voice multilingual sales agents (Arabic, English, Bahasa), and Aramex/NinjaVan/EasyPost logistics.
---

# Omnichannel Social Commerce (GCC & SEA) Skill

Use this skill when building conversational commerce pipelines, storefront integrations, AI agents, and carrier adapters.

## 1. Conversational Commerce & WhatsApp Flows
- **Multilingual AI Agent:** Supports English, Modern Standard & Khaleeji Arabic (`ar-SA`, `ar-AE`), Bahasa (`id-ID`, `ms-MY`), and Bengali (`bn-BD`).
- **Multimodal Inputs:**
  1. **Product Screenshots (GPT-4o Vision):** Extracts visual attributes, SKU candidates, color/size variants, and queries the Vector catalog for stock & price.
  2. **Voice Notes (Whisper API):** Transcribes customer audio notes (including mixed-dialect addresses and quantities) into structured order intents.
- **WhatsApp Flows for COD Verification:** Automatically dispatches an interactive WhatsApp Flow where customers verify their delivery pin, building number, and delivery slot.

## 2. Storefront Sync (Shopify, Salla, Zid)
- Ingest `orders/create` (Shopify) and `order.created` (Salla / Zid) webhooks.
- If `payment_method == "cod"`, immediately run the **Fraud Shield Evaluation** and hold courier booking until the required intervention (`ALLOW_SEAMLESS_COD` or `WHATSAPP_FLOW_CONFIRMATION`) succeeds.

## 3. Multi-Carrier Logistics Adapter (`CarrierAdapter` Trait)
All logistics providers implement a unified async Rust interface:
- `create_shipment(&self, req: &ShipmentRequest) -> Result<ShipmentBooking, LogisticsError>`
- `cancel_shipment(&self, tracking_number: &str) -> Result<bool, LogisticsError>`
- `parse_webhook_status(&self, payload: &[u8]) -> Result<NormalizedTrackingEvent, LogisticsError>`
- **Aramex (GCC):** Supports COD collection in `AED`, `SAR`, `QAR`, `KWD` and bilingual Arabic/English consignee fields.
- **Ninja Van (SEA):** Supports COD in `MYR`, `IDR`, `SGD`, `PHP` with granular RTS (Return-to-Sender) reason codes.
- **EasyPost (Global):** Unified aggregator for DHL Express, FedEx, UPS, and USPS.
