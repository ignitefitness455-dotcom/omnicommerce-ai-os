-- ============================================================================
-- Phase 2 Migration: Storefront Webhook Idempotency & Meta WhatsApp Flows
-- ============================================================================

-- 1. Webhook Idempotency Keys (Prevents duplicate courier bookings on webhook retries)
CREATE TABLE webhook_idempotency_keys (
    idempotency_key VARCHAR(128) PRIMARY KEY,
    source_platform VARCHAR(40) NOT NULL,           -- 'SHOPIFY', 'SALLA', 'ZID', 'META_WHATSAPP'
    merchant_id UUID REFERENCES merchants(id) ON DELETE CASCADE,
    processing_status VARCHAR(20) NOT NULL DEFAULT 'COMPLETED', -- 'IN_PROGRESS', 'COMPLETED', 'FAILED'
    response_status_code INT NOT NULL DEFAULT 200,
    response_snapshot JSONB,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    expires_at TIMESTAMPTZ NOT NULL DEFAULT (NOW() + INTERVAL '48 hours')
);

CREATE INDEX idx_idempotency_expires ON webhook_idempotency_keys(expires_at);

-- 2. Meta WhatsApp Interactive Flow Sessions (COD Address Pin & Intent Verification)
CREATE TABLE whatsapp_flow_sessions (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    order_id UUID NOT NULL REFERENCES orders(id) ON DELETE CASCADE,
    merchant_id UUID NOT NULL REFERENCES merchants(id) ON DELETE CASCADE,
    recipient_phone_e164 VARCHAR(32) NOT NULL,
    flow_token VARCHAR(128) UNIQUE NOT NULL,
    language_code VARCHAR(10) NOT NULL DEFAULT 'en_US', -- 'ar_SA', 'ar_AE', 'en_US', 'ms_MY'
    flow_state VARCHAR(40) NOT NULL DEFAULT 'DISPATCHED',
    -- 'DISPATCHED', 'CONFIRMED_COD', 'SWITCHED_TO_PREPAID_5PCT_OFF', 'CANCELLED_BY_BUYER', 'EXPIRED'
    confirmed_building_or_makani VARCHAR(120),
    confirmed_landmark TEXT,
    preferred_delivery_slot VARCHAR(60),
    prepaid_checkout_url TEXT,
    dispatched_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    responded_at TIMESTAMPTZ,
    expires_at TIMESTAMPTZ NOT NULL DEFAULT (NOW() + INTERVAL '6 hours')
);

CREATE INDEX idx_wa_flows_order_state ON whatsapp_flow_sessions(order_id, flow_state);
