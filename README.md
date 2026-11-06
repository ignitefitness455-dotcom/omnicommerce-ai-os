# OmniCommerce AI OS 🌍

**Global Social Commerce Automation, Zero-Knowledge COD Fraud Shield & Autonomous Deal Closer**  
*Engineered for GCC (UAE, Saudi Arabia, Qatar), Southeast Asia (Malaysia, Indonesia, Singapore), and Global Shopify/Salla/Zid D2C Brands.*

---

## 🚀 Complete 4-Phase Architecture Implemented

### Phase 1: Zero-Knowledge Fraud Shield & Core Infrastructure
- **GDPR & Saudi PDPL Compliant Fingerprinting:** [`crypto.rs`](./crates/omni-core/src/crypto.rs) normalizes global phones (`+971`, `+966`, `+60`, `+62`, `+1`, `+880`) to E.164 and computes `HMAC-SHA256` network fingerprints so raw customer PII is never leaked across merchants.
- **Bayesian RTO Scorer & Address NLP Gibberish Detector:** [`scorer.rs`](./crates/omni-fraud-engine/src/scorer.rs) and [`address_nlp.rs`](./crates/omni-fraud-engine/src/address_nlp.rs) evaluate network return history, keyboard-mashed addresses, and missing building/Makani numbers in `< 2ms`.

### Phase 2: Storefront Webhooks, Meta WhatsApp Flows & Pluggable Logistics
- **Constant-Time HMAC Verification & Idempotency:** [`hmac_verify.rs`](./crates/omni-core/src/hmac_verify.rs) and [`webhooks.rs`](./apps/server/src/webhooks.rs) handle **Shopify** (`X-Shopify-Hmac-Sha256`), **Salla/Zid** (`X-Salla-Signature`), and **Meta WhatsApp Cloud API** (`X-Hub-Signature-256`) webhooks with duplicate replay protection.
- **Bilingual (Arabic + English) Meta WhatsApp Flows:** [`whatsapp_flows.rs`](./crates/omni-ai/src/whatsapp_flows.rs) dispatches interactive address confirmation and 5% COD-to-Prepaid conversion cards.
- **Unified Multi-Carrier Adapter:** [`adapters.rs`](./crates/omni-logistics/src/adapters.rs) auto-books shipments across **Aramex (GCC)**, **Ninja Van (SEA)**, and **EasyPost (DHL/FedEx Global)**.

### Phase 3: Category-Defining Innovations
1. **Risk-Aware & Margin-Protected Autonomous AI Haggling Engine ([`negotiation_engine.rs`](./crates/omni-ai/src/negotiation_engine.rs)):**
   - Protects the merchant's secret Cost of Goods Sold (`COGS`) and minimum net margin floor while using dynamic discounts + **Tabby / Tamara / Atome 4x interest-free split payments** as leverage to convert risky COD buyers into 0%-RTO Prepaid buyers.
2. **Smart Carrier SLA & Zone RTO Arbitrage Router ([`smart_router.rs`](./crates/omni-logistics/src/smart_router.rs)):**
   - Computes `Total Effective Logistics Cost = Base Fee + (Zone RTO Prob × Reverse Penalty)` per postal zone to automatically select the carrier that maximizes net delivered profit.

### Phase 4: Fraud-Filtered Auto-Retargeting & Stripe SaaS ROI Engine
- **Fraud-Filtered Meta 24h Window Retargeting ([`retargeting.rs`](./crates/omni-ai/src/retargeting.rs)):**
   - Schedules a 3-wave recovery sequence (`T+45m` Scarcity, `T+6h` BNPL Split Framing, `T+22h` Pre-24h Window Closing Flash Offer) inside Meta's free 24-hour service window—and **automatically suppresses retargeting for High-RTO/Blacklisted shoppers** so merchants never waste ad spend on serial returners.
- **SaaS Subscription & Net ROI Calculator ([`billing.rs`](./crates/omni-core/src/billing.rs)):**
   - Tracks Growth ($79/mo), Scale ($249/mo), and Enterprise ($699/mo) tiers and computes live merchant ROI (`Prevented RTO Losses + Recovered Chats + Carrier Arbitrage Savings`).

---

## 🔌 REST API Endpoints (`apps/server` — Port `8080`)

| Method | Endpoint | Description |
| :--- | :--- | :--- |
| `GET` | `/health` | Health, compliance, and active phase status |
| `POST` | `/api/v1/fraud/evaluate` | Evaluate E.164 phone + NLP address quality & get COD intervention |
| `POST` | `/api/v1/ai/analyze-inquiry` | Multimodal GPT-4o Vision + Whisper Voice + RAG order extraction |
| `POST` | `/api/v1/logistics/book` | Book shipment via Aramex, Ninja Van, or EasyPost |
| `POST` | `/api/v1/webhooks/shopify/orders-create` | Shopify order webhook with HMAC-SHA256 & Idempotency |
| `POST` | `/api/v1/webhooks/salla/order-created` | Salla & Zid (GCC) order webhook with HMAC-SHA256 |
| `GET/POST` | `/api/v1/webhooks/whatsapp` | Meta WhatsApp Cloud API challenge & Flow `nfm_reply` handler |
| `POST` | `/api/v1/innovations/negotiate` | Margin-protected AI haggling & COD-to-Prepaid/BNPL deal closer |
| `POST` | `/api/v1/innovations/carrier-arbitrage` | Smart per-zone carrier RTO & cost arbitrage router |
| `POST` | `/api/v1/growth/retargeting/plan` | Fraud-filtered 3-wave Meta 24h abandoned chat recovery |
| `POST` | `/api/v1/growth/billing/roi-report` | Merchant SaaS subscription & live ROI multiple report |

---

## 🖥️ Quick Start

1. **Instant Interactive Browser Preview (Zero Setup Required):**
   - Open [`apps/web/public/command-center-preview.html`](./apps/web/public/command-center-preview.html) directly in Chrome/Edge to test all 4 Phases interactively!
2. **Start PostgreSQL 16 & Redis 7:**
   ```bash
   docker compose up -d
   ```
3. **Run the Rust Axum Backend:**
   ```bash
   cargo run -p omni-server
   ```
4. **Run the Next.js Web Dashboard:**
   ```bash
   cd apps/web && npm install && npm run dev
   ```
