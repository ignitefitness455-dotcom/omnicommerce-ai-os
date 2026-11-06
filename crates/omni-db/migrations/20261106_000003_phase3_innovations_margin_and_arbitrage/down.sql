DROP TABLE IF EXISTS carrier_zone_performance;
DROP TABLE IF EXISTS ai_negotiation_sessions;
ALTER TABLE products
    DROP COLUMN IF EXISTS max_ai_discount_pct,
    DROP COLUMN IF EXISTS min_required_margin_pct,
    DROP COLUMN IF EXISTS cost_minor;
