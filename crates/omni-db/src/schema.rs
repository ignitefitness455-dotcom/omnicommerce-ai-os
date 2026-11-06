// @generated automatically by Diesel CLI (and maintained for offline workspace compilation).

diesel::table! {
    merchants (id) {
        id -> Uuid,
        #[max_length = 255]
        business_name -> Varchar,
        #[max_length = 255]
        store_domain -> Nullable<Varchar>,
        #[max_length = 40]
        platform_type -> Varchar,
        #[max_length = 2]
        primary_country_iso -> Varchar,
        #[max_length = 3]
        default_currency -> Varchar,
        #[max_length = 255]
        email -> Varchar,
        #[max_length = 128]
        api_key_hash -> Varchar,
        cod_auto_approve_max_score -> Int4,
        cod_block_min_score -> Int4,
        is_active -> Bool,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    customer_network_fingerprints (id) {
        id -> Uuid,
        #[max_length = 64]
        phone_hash -> Varchar,
        #[max_length = 2]
        country_iso -> Varchar,
        #[max_length = 20]
        region_group -> Varchar,
        total_orders_placed -> Int4,
        total_orders_delivered -> Int4,
        total_rto_returned -> Int4,
        total_chargebacks -> Int4,
        recent_90d_incidents -> Int4,
        rto_rate_pct -> Numeric,
        risk_score -> Int4,
        #[max_length = 20]
        risk_tier -> Varchar,
        is_globally_blacklisted -> Bool,
        #[max_length = 100]
        blacklist_reason -> Nullable<Varchar>,
        last_seen_at -> Timestamptz,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    fraud_incident_reports (id) {
        id -> Uuid,
        merchant_id -> Uuid,
        fingerprint_id -> Uuid,
        #[max_length = 50]
        incident_type -> Varchar,
        #[max_length = 30]
        carrier_provider -> Nullable<Varchar>,
        #[max_length = 100]
        tracking_number -> Nullable<Varchar>,
        order_amount_minor -> Int8,
        #[max_length = 3]
        currency -> Varchar,
        evidence_summary -> Nullable<Text>,
        reported_at -> Timestamptz,
    }
}

diesel::table! {
    orders (id) {
        id -> Uuid,
        merchant_id -> Uuid,
        #[max_length = 128]
        external_store_order_id -> Nullable<Varchar>,
        fingerprint_id -> Uuid,
        #[max_length = 30]
        source_channel -> Varchar,
        #[max_length = 255]
        customer_name -> Varchar,
        #[max_length = 32]
        customer_phone_e164 -> Varchar,
        shipping_address -> Text,
        #[max_length = 120]
        shipping_city -> Varchar,
        #[max_length = 2]
        country_iso -> Varchar,
        total_amount_minor -> Int8,
        #[max_length = 3]
        currency -> Varchar,
        #[max_length = 20]
        payment_method -> Varchar,
        assessed_risk_score -> Int4,
        #[max_length = 20]
        assessed_risk_tier -> Varchar,
        #[max_length = 50]
        applied_intervention -> Varchar,
        whatsapp_flow_confirmed -> Bool,
        #[max_length = 40]
        order_status -> Varchar,
        #[max_length = 30]
        carrier_provider -> Nullable<Varchar>,
        #[max_length = 100]
        tracking_number -> Nullable<Varchar>,
        awb_label_url -> Nullable<Text>,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    webhook_idempotency_keys (idempotency_key) {
        #[max_length = 128]
        idempotency_key -> Varchar,
        #[max_length = 40]
        source_platform -> Varchar,
        merchant_id -> Nullable<Uuid>,
        #[max_length = 20]
        processing_status -> Varchar,
        response_status_code -> Int4,
        response_snapshot -> Nullable<Jsonb>,
        created_at -> Timestamptz,
        expires_at -> Timestamptz,
    }
}

diesel::table! {
    whatsapp_flow_sessions (id) {
        id -> Uuid,
        order_id -> Uuid,
        merchant_id -> Uuid,
        #[max_length = 32]
        recipient_phone_e164 -> Varchar,
        #[max_length = 128]
        flow_token -> Varchar,
        #[max_length = 10]
        language_code -> Varchar,
        #[max_length = 40]
        flow_state -> Varchar,
        #[max_length = 120]
        confirmed_building_or_makani -> Nullable<Varchar>,
        confirmed_landmark -> Nullable<Text>,
        #[max_length = 60]
        preferred_delivery_slot -> Nullable<Varchar>,
        prepaid_checkout_url -> Nullable<Text>,
        dispatched_at -> Timestamptz,
        responded_at -> Nullable<Timestamptz>,
        expires_at -> Timestamptz,
    }
}

diesel::table! {
    abandoned_chat_recoveries (id) {
        id -> Uuid,
        merchant_id -> Uuid,
        fingerprint_id -> Uuid,
        #[max_length = 32]
        buyer_phone_e164 -> Varchar,
        buyer_fraud_risk_score -> Int4,
        #[max_length = 100]
        product_sku -> Varchar,
        cart_value_minor -> Int8,
        #[max_length = 3]
        currency -> Varchar,
        eligible_for_retargeting -> Bool,
        suppression_reason -> Nullable<Text>,
        #[max_length = 50]
        current_wave -> Varchar,
        recovered_order_id -> Nullable<Uuid>,
        last_buyer_message_at -> Timestamptz,
        created_at -> Timestamptz,
    }
}

diesel::table! {
    merchant_subscriptions (id) {
        id -> Uuid,
        merchant_id -> Uuid,
        #[max_length = 128]
        stripe_customer_id -> Nullable<Varchar>,
        #[max_length = 128]
        stripe_subscription_id -> Nullable<Varchar>,
        #[max_length = 30]
        plan_tier -> Varchar,
        monthly_fee_usd -> Numeric,
        fraud_checks_included -> Int4,
        fraud_checks_used -> Int4,
        ai_sessions_used -> Int4,
        billing_cycle_anchor -> Timestamptz,
        #[max_length = 30]
        status -> Varchar,
        updated_at -> Timestamptz,
    }
}
