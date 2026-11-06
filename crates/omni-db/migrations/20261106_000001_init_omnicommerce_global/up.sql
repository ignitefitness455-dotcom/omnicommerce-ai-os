-- ============================================================================
-- OmniCommerce AI OS — Global Multi-Tenant & Zero-Knowledge Fraud Shield Schema
-- Compliant with GDPR (EU) & Saudi PDPL (Personal Data Protection Law)
-- ============================================================================

CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

-- 1. Global Merchants (Tenants)
CREATE TABLE merchants (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    business_name VARCHAR(255) NOT NULL,
    store_domain VARCHAR(255),
    platform_type VARCHAR(40) NOT NULL DEFAULT 'STANDALONE', -- 'SHOPIFY', 'SALLA', 'ZID', 'WOOCOMMERCE', 'STANDALONE'
    primary_country_iso VARCHAR(2) NOT NULL DEFAULT 'AE',
    default_currency VARCHAR(3) NOT NULL DEFAULT 'AED',
    email VARCHAR(255) UNIQUE NOT NULL,
    api_key_hash VARCHAR(128) UNIQUE NOT NULL,
    cod_auto_approve_max_score INT NOT NULL DEFAULT 24 CHECK (cod_auto_approve_max_score BETWEEN 0 AND 100),
    cod_block_min_score INT NOT NULL DEFAULT 75 CHECK (cod_block_min_score BETWEEN 0 AND 100),
    is_active BOOLEAN NOT NULL DEFAULT TRUE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- 2. Zero-Knowledge Cross-Merchant Customer Fingerprints (Shared Fraud Shield)
-- Raw PII is NEVER stored in this shared table; only HMAC-SHA256 fingerprints.
CREATE TABLE customer_network_fingerprints (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    phone_hash VARCHAR(64) UNIQUE NOT NULL,         -- HMAC-SHA256(pepper, E.164 phone)
    country_iso VARCHAR(2) NOT NULL,                -- e.g. 'AE', 'SA', 'MY', 'US'
    region_group VARCHAR(20) NOT NULL,              -- 'GCC', 'SEA', 'US_EU', 'SOUTH_ASIA'

    -- Network Behavioral Aggregates
    total_orders_placed INT NOT NULL DEFAULT 0,
    total_orders_delivered INT NOT NULL DEFAULT 0,
    total_rto_returned INT NOT NULL DEFAULT 0,
    total_chargebacks INT NOT NULL DEFAULT 0,
    recent_90d_incidents INT NOT NULL DEFAULT 0,

    -- Calculated Risk Intelligence
    rto_rate_pct NUMERIC(5, 2) NOT NULL DEFAULT 0.00,
    risk_score INT NOT NULL DEFAULT 15 CHECK (risk_score BETWEEN 0 AND 100),
    risk_tier VARCHAR(20) NOT NULL DEFAULT 'SAFE',  -- 'SAFE', 'MODERATE', 'HIGH', 'BLOCKED'
    is_globally_blacklisted BOOLEAN NOT NULL DEFAULT FALSE,
    blacklist_reason VARCHAR(100),

    last_seen_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_fingerprints_country_tier ON customer_network_fingerprints(country_iso, risk_tier);
CREATE INDEX idx_fingerprints_risk_score ON customer_network_fingerprints(risk_score DESC);

-- 3. Fraud & RTO Incident Reports (Contributed by Merchants & Carrier Webhooks)
CREATE TABLE fraud_incident_reports (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    merchant_id UUID NOT NULL REFERENCES merchants(id) ON DELETE CASCADE,
    fingerprint_id UUID NOT NULL REFERENCES customer_network_fingerprints(id) ON DELETE CASCADE,
    incident_type VARCHAR(50) NOT NULL,
    -- 'DOORSTEP_REFUSAL', 'FAKE_ADDRESS_GIBBERISH', 'UNREACHABLE_PHONE', 'CHARGEBACK_FRAUD', 'COURIER_ABUSE'
    carrier_provider VARCHAR(30),                   -- 'ARAMEX_GCC', 'NINJAVAN_SEA', 'EASYPOST_GLOBAL'
    tracking_number VARCHAR(100),
    order_amount_minor BIGINT NOT NULL DEFAULT 0,
    currency VARCHAR(3) NOT NULL DEFAULT 'AED',
    evidence_summary TEXT,
    reported_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_incidents_fingerprint_date ON fraud_incident_reports(fingerprint_id, reported_at DESC);

-- 4. Postal / District Risk Clusters (Geo Intelligence)
CREATE TABLE postal_risk_clusters (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    country_iso VARCHAR(2) NOT NULL,
    city_or_emirate VARCHAR(120) NOT NULL,
    postal_or_zone_code VARCHAR(40) NOT NULL,
    historical_rto_rate_pct NUMERIC(5, 2) NOT NULL DEFAULT 12.00,
    total_shipments_sampled INT NOT NULL DEFAULT 0,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE (country_iso, postal_or_zone_code)
);

-- 5. Merchant Product Catalog (Synced with Vector DB for GPT-4o Vision Agent)
CREATE TABLE products (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    merchant_id UUID NOT NULL REFERENCES merchants(id) ON DELETE CASCADE,
    sku VARCHAR(100) NOT NULL,
    title_en VARCHAR(255) NOT NULL,
    title_ar VARCHAR(255),
    price_minor BIGINT NOT NULL,
    currency VARCHAR(3) NOT NULL DEFAULT 'AED',
    stock_quantity INT NOT NULL DEFAULT 0,
    vector_embedding_id VARCHAR(128),
    image_url TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE (merchant_id, sku)
);

-- 6. Omnichannel Orders & Dynamic COD Verification State Machine
CREATE TABLE orders (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    merchant_id UUID NOT NULL REFERENCES merchants(id) ON DELETE CASCADE,
    external_store_order_id VARCHAR(128),
    fingerprint_id UUID NOT NULL REFERENCES customer_network_fingerprints(id),
    source_channel VARCHAR(30) NOT NULL,            -- 'WHATSAPP_CLOUD', 'INSTAGRAM_DM', 'TIKTOK_SHOP', 'SHOPIFY', 'SALLA'

    customer_name VARCHAR(255) NOT NULL,
    customer_phone_e164 VARCHAR(32) NOT NULL,
    shipping_address TEXT NOT NULL,
    shipping_city VARCHAR(120) NOT NULL,
    country_iso VARCHAR(2) NOT NULL,

    total_amount_minor BIGINT NOT NULL,
    currency VARCHAR(3) NOT NULL,
    payment_method VARCHAR(20) NOT NULL,            -- 'COD', 'PREPAID_CARD', 'APPLE_PAY', 'TABBY_BNPL'

    -- Fraud Shield Evaluation Snapshot
    assessed_risk_score INT NOT NULL,
    assessed_risk_tier VARCHAR(20) NOT NULL,
    applied_intervention VARCHAR(50) NOT NULL,      -- 'ALLOW_SEAMLESS_COD', 'WHATSAPP_FLOW_CONFIRMATION', etc.
    whatsapp_flow_confirmed BOOLEAN NOT NULL DEFAULT FALSE,

    -- Fulfillment & Carrier State
    order_status VARCHAR(40) NOT NULL DEFAULT 'PENDING_VERIFICATION',
    carrier_provider VARCHAR(30),
    tracking_number VARCHAR(100),
    awb_label_url TEXT,

    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_orders_merchant_status ON orders(merchant_id, order_status, created_at DESC);
