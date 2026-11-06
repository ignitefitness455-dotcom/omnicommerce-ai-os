import React from "react";

export default function MerchantAuthOnboardingPage() {
  return (
    <main className="max-w-4xl mx-auto p-6 md:p-10 space-y-6">
      <div>
        <h1 className="text-2xl font-bold text-white">
          🔑 Merchant Onboarding, API Keys & Storefront App Connect
        </h1>
        <p className="text-sm text-slate-400 mt-1">
          Generate your Stripe-style `omni_live_...` API Key and connect your Shopify, Salla, Zid, or WooCommerce store in 1 click
        </p>
      </div>

      <div className="p-6 rounded-2xl bg-slate-900 border border-slate-800 space-y-4 text-xs">
        <div className="flex items-center justify-between border-b border-slate-800 pb-3">
          <span className="font-bold text-white text-sm">
            Active Production Credentials
          </span>
          <span className="px-2.5 py-1 rounded bg-emerald-500/20 text-emerald-300 font-semibold">
            HMAC-SHA256 Protected
          </span>
        </div>
        <div className="space-y-2 font-mono">
          <div className="p-3 rounded-lg bg-slate-950 border border-slate-800">
            <div className="text-slate-400">Publishable Key:</div>
            <div className="text-emerald-400 mt-0.5">
              omni_pk_1111111111111111
            </div>
          </div>
          <div className="p-3 rounded-lg bg-slate-950 border border-slate-800">
            <div className="text-slate-400">Secret API Key:</div>
            <div className="text-amber-300 mt-0.5">
              omni_live_9f86d081884c7d659a2feaa0c55ad015a3bf
            </div>
          </div>
        </div>
      </div>
    </main>
  );
}
