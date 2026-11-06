---
name: rust-axum-diesel-enterprise
description: Production architecture patterns for Rust microservices and monoliths using Axum, Tokio, Diesel ORM (PostgreSQL), Redis caching, RFC 9457 Problem Details error envelopes, and multi-tenant security.
---

# Rust + Axum + Diesel Enterprise Skill

Use this skill when building or extending the Rust backend crates in `apps/server` and `crates/*`.

## 1. Workspace Crate Boundaries
- `crates/omni-core`: Pure domain types, E.164 phone normalization, SHA-256 salted hashing, ISO 4217 Money types, and RFC 9457 API errors. Zero database or HTTP framework coupling.
- `crates/omni-fraud-engine`: Pure deterministic + ML-feature risk scoring engine, address NLP analyzer, and dynamic COD intervention rule engine.
- `crates/omni-logistics`: Async `CarrierAdapter` trait and concrete implementations for Aramex, Ninja Van, and EasyPost.
- `crates/omni-ai`: OpenAI GPT-4o Vision + Whisper transcription + RAG vector lookup pipelines.
- `crates/omni-db`: Diesel schema, migrations, connection pooling (`deadpool-diesel` / `r2d2`), and repository queries.
- `apps/server`: Axum router, extractors, HMAC webhook verifiers, rate limiting, and OpenAPI routes.

## 2. Error Handling (RFC 9457 Problem Details)
Every Axum handler returns `Result<Json<T>, ApiError>` where `ApiError` serializes to standard RFC 9457 JSON:
```json
{
  "type": "https://api.omnicommerce.ai/errors/fraud-high-risk",
  "title": "High RTO Fraud Risk Detected",
  "status": 422,
  "detail": "Customer fingerprint exceeds merchant COD risk threshold.",
  "instance": "/api/v1/orders"
}
```

## 3. Idempotency & Webhook Verification
- Require `Idempotency-Key` header on all state-mutating `POST` endpoints (`/api/v1/orders`, `/api/v1/logistics/shipments`).
- Verify `X-Shopify-Hmac-Sha256`, `X-Salla-Signature`, and `X-Hub-Signature-256` (Meta WhatsApp Cloud API) using constant-time comparison before deserializing payloads.
