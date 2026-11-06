import React from "react";

export default function IntegrationsSettingsPage() {
  const integrations = [
    {
      name: "Meta WhatsApp Cloud API & Flows (WABA)",
      endpoint: "POST /api/v1/webhooks/whatsapp",
      envVars: "META_WHATSAPP_ACCESS_TOKEN, META_WHATSAPP_VERIFY_TOKEN",
      status: "Ready for Live WABA Token",
    },
    {
      name: "Shopify & Salla / Zid Storefront Webhooks",
      endpoint: "POST /api/v1/webhooks/shopify/orders-create • /salla/order-created",
      envVars: "SHOPIFY_WEBHOOK_SECRET, SALLA_WEBHOOK_SECRET",
      status: "HMAC-SHA256 Verifier Active",
    },
    {
      name: "OpenAI GPT-4o Vision + Whisper + Pinecone Vector DB",
      endpoint: "POST /api/v1/ai/analyze-inquiry",
      envVars: "OPENAI_API_KEY, PINECONE_API_KEY",
      status: "Hybrid Live + Deterministic Fallback Ready",
    },
    {
      name: "Aramex (GCC), Ninja Van (SEA) & EasyPost (DHL/FedEx)",
      endpoint: "POST /api/v1/logistics/book",
      envVars: "ARAMEX_ACCOUNT_NUMBER, NINJAVAN_CLIENT_ID, EASYPOST_API_KEY",
      status: "Smart Zone Arbitrage Router Active",
    },
    {
      name: "Stripe SaaS Billing & Instant Prepayment Links",
      endpoint: "POST /api/v1/merchants/stripe-webhook",
      envVars: "STRIPE_SECRET_KEY, STRIPE_WEBHOOK_SECRET",
      status: "Growth ($79) / Scale ($249) / Enterprise ($699) Configured",
    },
  ];

  return (
    <main className="max-w-6xl mx-auto p-6 md:p-10 space-y-6">
      <div>
        <h1 className="text-2xl font-bold text-white">
          ⚙️ Third-Party Integrations & Production Credentials Hub
        </h1>
        <p className="text-sm text-slate-400 mt-1">
          All external provider adapters are wired into the Rust Axum server and controlled via environment variables
        </p>
      </div>

      <div className="space-y-3">
        {integrations.map((item) => (
          <div
            key={item.name}
            className="p-5 rounded-2xl bg-slate-900 border border-slate-800 flex flex-col md:flex-row md:items-center justify-between gap-4 text-xs"
          >
            <div className="space-y-1">
              <div className="text-sm font-bold text-white">{item.name}</div>
              <div className="font-mono text-indigo-400">{item.endpoint}</div>
              <div className="text-slate-400">
                Env Keys: <code className="text-slate-300">{item.envVars}</code>
              </div>
            </div>
            <span className="px-3 py-1.5 rounded-lg bg-emerald-500/15 border border-emerald-500/30 text-emerald-300 font-semibold">
              ✅ {item.status}
            </span>
          </div>
        ))}
      </div>
    </main>
  );
}
