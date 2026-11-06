-- ============================================================================
-- Phase 3 Innovation Migration:
-- 1. Margin-Protected Autonomous AI Haggling & BNPL Conversion Guardrails
-- 2. Smart Carrier SLA & Zone RTO Arbitrage Router
-- ============================================================================

ALTER TABLE products
    ADD COLUMN IF NOT EXISTS cost_minor BIGINT NOT NULL DEFAULT 0,
    ADD COLUMN IF NOT EXISTS min_required_margin_pct NUMERIC(5, 2) NOT NULL DEFAULT 28.00,
    ADD COLUMN IF NOT EXISTS max_ai_discount_pct NUMERIC(5, 2) NOT NULL DEFAULT 12.00;

-- Tracks AI Haggling & COD-to-Prepaid/BNPL Conversion Sessions
CREATE TABLE ai_negotiation_sessions (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    merchant_id UUID NOT NULL REFERENCES merchants(id) ON DELETE CASCADE,
    product_id UUID NOT NULL REFERENCES products(id) ON DELETE CASCADE,
    buyer_phone_hash VARCHAR(64) NOT NULL,
    buyer_fraud_risk_score INT NOT NULL,
    requested_discount_pct NUMERIC(5, 2) NOT NULL DEFAULT 0.00,
    applied_strategy VARCHAR(50) NOT NULL,
    -- 'STANDARD_CATALOG_QUOTE', 'GRANT_VIP_OR_BUNDLE_DISCOUNT',
    -- 'LEVERAGE_DISCOUNT_FOR_PREPAID_SWITCH', 'COUNTER_OFFER_AT_MARGIN_FLOOR'
    unlocked_prepaid_discount_pct NUMERIC(5, 2) NOT NULL,
    projected_merchant_margin_pct NUMERIC(5, 2) NOT NULL,
    converted_to_prepaid_or_bnpl BOOLEAN NOT NULL DEFAULT FALSE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Tracks Per-Zone Carrier Delivery Success & RTO Arbitrage Performance
CREATE TABLE carrier_zone_performance (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    country_iso VARCHAR(2) NOT NULL,
    city_or_zone VARCHAR(120) NOT NULL,
    carrier_name VARCHAR(80) NOT NULL,
    shipping_fee_usd NUMERIC(8, 2) NOT NULL,
    zone_delivery_success_pct NUMERIC(5, 2) NOT NULL,
    avg_transit_hours INT NOT NULL DEFAULT 36,
    rto_reverse_fee_usd NUMERIC(8, 2) NOT NULL DEFAULT 4.00,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE (country_iso, city_or_zone, carrier_name)
);

CREATE INDEX idx_carrier_zone_lookup ON carrier_zone_performance(country_iso, city_or_zone);
