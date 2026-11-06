-- ============================================================================
-- OmniCommerce AI OS — Global Production Demo Seed Data (GCC, SEA & Global)
-- ============================================================================

INSERT INTO merchants (
    id, business_name, store_domain, platform_type, primary_country_iso,
    default_currency, email, api_key_hash, cod_auto_approve_max_score, cod_block_min_score
) VALUES
    ('11111111-1111-1111-1111-111111111111', 'Al-Noor Luxury Abayas Dubai', 'alnoor-dubai.myshopify.com', 'SHOPIFY', 'AE', 'AED', 'ops@alnoor-dubai.ae', '9f86d081884c7d659a2feaa0c55ad015a3bf4f1b2b0b822cd15d6c15b0f00a01', 24, 75),
    ('22222222-2222-2222-2222-222222222222', 'Riyadh Oud & Perfumes KSA', 'salla.sa/riyadh-oud', 'SALLA', 'SA', 'SAR', 'founder@riyadhoud.sa', '9f86d081884c7d659a2feaa0c55ad015a3bf4f1b2b0b822cd15d6c15b0f00a02', 24, 70),
    ('33333333-3333-3333-3333-333333333333', 'KL Modest Fashion Malaysia', 'klmodest.com.my', 'SHOPIFY', 'MY', 'MYR', 'hello@klmodest.my', '9f86d081884c7d659a2feaa0c55ad015a3bf4f1b2b0b822cd15d6c15b0f00a03', 28, 75)
ON CONFLICT (email) DO NOTHING;

INSERT INTO customer_network_fingerprints (
    id, phone_hash, country_iso, region_group,
    total_orders_placed, total_orders_delivered, total_rto_returned,
    total_chargebacks, recent_90d_incidents, rto_rate_pct, risk_score, risk_tier, is_globally_blacklisted
) VALUES
    ('aaaa1111-1111-1111-1111-111111111111', 'e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b801', 'AE', 'GCC', 9, 9, 0, 0, 0, 0.00, 8, 'SAFE', FALSE),
    ('aaaa2222-2222-2222-2222-222222222222', 'e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b802', 'SA', 'GCC', 4, 3, 1, 0, 0, 25.00, 38, 'MODERATE', FALSE),
    ('aaaa3333-3333-3333-3333-333333333333', 'e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b803', 'AE', 'GCC', 6, 1, 5, 1, 3, 83.33, 94, 'BLOCKED', TRUE)
ON CONFLICT (phone_hash) DO NOTHING;

INSERT INTO products (
    id, merchant_id, sku, title_en, title_ar, price_minor, currency,
    stock_quantity, cost_minor, min_required_margin_pct, max_ai_discount_pct
) VALUES
    ('bbbb1111-1111-1111-1111-111111111111', '11111111-1111-1111-1111-111111111111', 'OMNI-GCC-ABAYA-902', 'Oud Royal Velvet Abaya — Limited Edition', 'عباية مخمل ملكي فاخرة', 42000, 'AED', 18, 24000, 28.00, 12.00),
    ('bbbb2222-2222-2222-2222-222222222222', '22222222-2222-2222-2222-222222222222', 'OMNI-KSA-OUD-501', 'Royal Cambodian Oud Elixir 100ml', 'دهن عود كمبودي ملكي', 54000, 'SAR', 32, 29000, 30.00, 10.00),
    ('bbbb3333-3333-3333-3333-333333333333', '33333333-3333-3333-3333-333333333333', 'OMNI-SEA-SILK-310', 'Kuala Lumpur Silk Kurung Set', NULL, 18500, 'MYR', 25, 9800, 26.00, 12.00)
ON CONFLICT (merchant_id, sku) DO NOTHING;
