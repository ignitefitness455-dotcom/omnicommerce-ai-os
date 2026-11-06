-- ============================================================================
-- Phase 4 Migration: Fraud-Filtered Auto-Retargeting & Stripe SaaS Billing ROI
-- ============================================================================

CREATE TABLE abandoned_chat_recoveries (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    merchant_id UUID NOT NULL REFERENCES merchants(id) ON DELETE CASCADE,
    fingerprint_id UUID NOT NULL REFERENCES customer_network_fingerprints(id) ON DELETE CASCADE,
    buyer_phone_e164 VARCHAR(32) NOT NULL,
    buyer_fraud_risk_score INT NOT NULL,
    product_sku VARCHAR(100) NOT NULL,
    cart_value_minor BIGINT NOT NULL,
    currency VARCHAR(3) NOT NULL DEFAULT 'AED',
    eligible_for_retargeting BOOLEAN NOT NULL DEFAULT TRUE,
    suppression_reason TEXT,
    current_wave VARCHAR(50) NOT NULL DEFAULT 'WAVE1_SCARCITY_45M',
    recovered_order_id UUID REFERENCES orders(id),
    last_buyer_message_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_retargeting_merchant_wave ON abandoned_chat_recoveries(merchant_id, eligible_for_retargeting, current_wave);

CREATE TABLE merchant_subscriptions (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    merchant_id UUID UNIQUE NOT NULL REFERENCES merchants(id) ON DELETE CASCADE,
    stripe_customer_id VARCHAR(128),
    stripe_subscription_id VARCHAR(128),
    plan_tier VARCHAR(30) NOT NULL DEFAULT 'SCALE', -- 'GROWTH', 'SCALE', 'ENTERPRISE'
    monthly_fee_usd NUMERIC(8, 2) NOT NULL DEFAULT 249.00,
    fraud_checks_included INT NOT NULL DEFAULT 10000,
    fraud_checks_used INT NOT NULL DEFAULT 0,
    ai_sessions_used INT NOT NULL DEFAULT 0,
    billing_cycle_anchor TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    status VARCHAR(30) NOT NULL DEFAULT 'ACTIVE',
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
