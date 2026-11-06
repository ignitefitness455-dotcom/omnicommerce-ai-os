---
name: global-rto-fraud-intelligence
description: GoKwik and Signifyd-inspired COD Return-to-Origin (RTO) and Chargeback Fraud Detection Engine. Covers GDPR/Saudi PDPL zero-knowledge hashing, address NLP gibberish scoring, postal risk clustering, and dynamic COD-to-Prepaid interventions.
---

# Global RTO & Fraud Intelligence Skill

Use this skill when designing or modifying the Fraud Shield database, risk scoring engine, or checkout interventions.

## 1. Privacy-Preserving Network Identity (GDPR & Saudi PDPL)
1. **Normalize Phone to E.164:** Strip spaces, dashes, and leading `00`; validate country code (`+971` UAE, `+966` KSA, `+60` MY, `+62` ID, `+1` US, `+880` BD).
2. **Salted SHA-256 Fingerprint:** Compute `HMAC-SHA256(network_pepper, e164_phone)` so the global network table never exposes raw phone numbers across tenants.
3. **Behavioral Aggregates Only:** Cross-merchant queries return aggregate counts (`total_dispatched`, `total_delivered`, `total_rto_returned`, `total_chargebacks`, `risk_score`, `risk_tier`, `recommended_intervention`).

## 2. Multi-Signal Risk Scoring (0 to 100)
Combine 5 weighted signal groups:
- **Historical Network RTO Ratio (40% weight):** Bayesian-smoothed return rate across all participating merchants (`(rto_count + prior_rto) / (total_orders + prior_total)`).
- **Recent Incident Severity (25% weight):** Doorstep refusal, fake OTP, courier harassment, or chargeback within the last 90 days.
- **Address Quality NLP Score (15% weight):** Detects gibberish strings (e.g., `"asdfgh"`, `"test 123"`), missing building/street/Makani/postal numbers, or abnormally short strings (< 12 chars).
- **Order Velocity & COD Value Spike (10% weight):** Multiple COD orders across different merchants within 2 hours, or COD value > 2.5x regional category average.
- **Geo / Postal Cluster Risk (10% weight):** Historical RTO rate of the target postal code / district.

## 3. Dynamic Checkout Interventions (GoKwik Smart COD Model)
Map the final `risk_score` (0–100) to an automated intervention:
- `0..=24` (`SAFE`): `ALLOW_SEAMLESS_COD` — Instant order confirmation & auto-dispatch to courier.
- `25..=49` (`LOW_MODERATE`): `WHATSAPP_FLOW_CONFIRMATION` — Trigger interactive WhatsApp Flow requiring 1-tap address & intent confirmation before booking courier.
- `50..=74` (`HIGH`): `PARTIAL_ADVANCE_OR_PREPAID_NUDGE` — Require 15–20% advance shipping deposit via Stripe/ApplePay/Tabby OR offer a 5% instant discount to switch from COD to Prepaid.
- `75..=100` (`CRITICAL_BLOCKED`): `BLOCK_COD_PREPAID_ONLY` — Disable COD completely; require 100% verified digital prepayment.
