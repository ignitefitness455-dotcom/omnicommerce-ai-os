# OmniCommerce AI OS — Global Architecture & Engineering Rules

## 1. Product Vision & Target Market
- **Product:** OmniCommerce AI OS — Global Social Commerce Automation & Cross-Merchant Fraud Intelligence Platform.
- **Primary Beachhead Markets:** GCC (UAE `+971`, Saudi Arabia `+966`, Qatar `+974`) and Southeast Asia (Malaysia `+60`, Indonesia `+62`, Singapore `+65`), alongside Global Shopify/Salla/Zid D2C brands.
- **Core Pillars:**
  1. **Zero-Knowledge Fraud Shield (GoKwik / Signifyd Model):** Cross-merchant shopper reputation network using salted SHA-256 PII fingerprints (GDPR & Saudi PDPL compliant), address NLP quality scoring, velocity checks, and Dynamic COD Interventions (Seamless COD, WhatsApp Flow OTP, Partial Advance, COD-to-Prepaid Discount Nudge, Hard Block).
  2. **Multilingual AI Sales Agent (Respond.io / Zoko Model):** GPT-4o Vision + Whisper Voice + RAG product search handling English, Khaleeji/Modern Standard Arabic, Bahasa, and Bengali across WhatsApp Cloud API, Instagram DM, TikTok Shop, and Messenger.
  3. **Pluggable Global Auto-Logistics:** Unified `CarrierAdapter` trait supporting **Aramex** (GCC), **Ninja Van / J&T** (SEA), and **EasyPost / Shippo** (DHL, FedEx, UPS globally).
  4. **Storefront & Webhook Ecosystem:** Native webhook ingestion for **Shopify**, **Salla**, **Zid**, and **WooCommerce** with HMAC verification and `Idempotency-Key` enforcement.

## 2. Technical Standards
- **Backend:** Rust Workspace (`apps/server` with Axum 0.7 + Tokio, `crates/omni-core`, `crates/omni-fraud-engine`, `crates/omni-logistics`, `crates/omni-ai`, `crates/omni-db` with Diesel + PostgreSQL + Redis).
- **Frontend:** Next.js 14+ (App Router, TypeScript, Tailwind CSS, Mobile-First PWA).
- **Money & Time:** Store all monetary values in minor units (`BIGINT` cents/fils/halalas) alongside ISO 4217 currency codes (`AED`, `SAR`, `USD`, `MYR`, `EUR`). Store all timestamps in `TIMESTAMPTZ` (UTC).
- **Privacy by Design:** Never expose raw customer PII from Merchant A to Merchant B. All cross-merchant lookups operate on normalized E.164 phone hashes (`phone_hash`) and anonymized behavioral aggregates.
