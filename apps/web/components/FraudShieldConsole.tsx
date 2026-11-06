"use client";

import React, { useState } from "react";

interface EvaluationResult {
  e164: string;
  countryIso: string;
  regionGroup: string;
  phoneHash: string;
  riskScore: number;
  riskTier: "SAFE" | "MODERATE" | "HIGH" | "BLOCKED";
  rtoRatePct: number;
  intervention: string;
  message: string;
  flags: string[];
}

export function FraudShieldConsole() {
  const [phone, setPhone] = useState("+971501234567");
  const [address, setAddress] = useState("Boulevard Point Tower 1, Apt 1402, Downtown Dubai");
  const [rtoCount, setRtoCount] = useState(0);
  const [totalOrders, setTotalOrders] = useState(6);
  const [result, setResult] = useState<EvaluationResult | null>(null);

  function runSimulation() {
    const score = Math.min(
      100,
      Math.round((rtoCount / Math.max(1, totalOrders)) * 65) +
        (address.length < 12 ? 35 : 0)
    );
    const tier =
      score <= 24
        ? "SAFE"
        : score <= 49
        ? "MODERATE"
        : score <= 74
        ? "HIGH"
        : "BLOCKED";

    const intervention =
      tier === "SAFE"
        ? "ALLOW_SEAMLESS_COD"
        : tier === "MODERATE"
        ? "WHATSAPP_FLOW_CONFIRMATION"
        : tier === "HIGH"
        ? "PARTIAL_ADVANCE_20_PCT_OR_5_PCT_PREPAID_DISCOUNT"
        : "BLOCK_COD_PREPAID_ONLY";

    setResult({
      e164: phone,
      countryIso: phone.startsWith("+966") ? "SA" : phone.startsWith("+60") ? "MY" : "AE",
      regionGroup: phone.startsWith("+60") ? "SEA" : "GCC",
      phoneHash: "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855",
      riskScore: score,
      riskTier: tier,
      rtoRatePct: Math.round((rtoCount / Math.max(1, totalOrders)) * 100),
      intervention,
      message:
        tier === "SAFE"
          ? "Verified low-risk shopper. Auto-confirm COD & dispatch to Aramex/NinjaVan."
          : tier === "MODERATE"
          ? "Trigger interactive WhatsApp Flow confirmation before booking courier."
          : "Hold COD dispatch. Request 20% shipping deposit or switch to Prepaid.",
      flags: address.length < 12 ? ["ADDRESS_TOO_SHORT"] : ["VERIFIED_BUILDING_NUMBER"],
    });
  }

  return (
    <section className="grid grid-cols-1 lg:grid-cols-2 gap-6">
      <div className="p-6 rounded-2xl bg-slate-900 border border-slate-800 space-y-4">
        <h2 className="text-lg font-semibold text-white">
          Live Cross-Merchant Fraud Shield Lookup
        </h2>
        <div className="space-y-3">
          <div>
            <label className="block text-xs text-slate-400 mb-1">
              Customer Phone (E.164: UAE +971, KSA +966, MY +60)
            </label>
            <input
              value={phone}
              onChange={(e) => setPhone(e.target.value)}
              className="w-full px-3 py-2 rounded-lg bg-slate-950 border border-slate-700 text-sm text-white"
            />
          </div>
          <div>
            <label className="block text-xs text-slate-400 mb-1">
              Shipping Address (English or Arabic)
            </label>
            <input
              value={address}
              onChange={(e) => setAddress(e.target.value)}
              className="w-full px-3 py-2 rounded-lg bg-slate-950 border border-slate-700 text-sm text-white"
            />
          </div>
          <div className="grid grid-cols-2 gap-3">
            <div>
              <label className="block text-xs text-slate-400 mb-1">
                Network Past Orders
              </label>
              <input
                type="number"
                value={totalOrders}
                onChange={(e) => setTotalOrders(Number(e.target.value))}
                className="w-full px-3 py-2 rounded-lg bg-slate-950 border border-slate-700 text-sm text-white"
              />
            </div>
            <div>
              <label className="block text-xs text-slate-400 mb-1">
                Network Past RTO Returns
              </label>
              <input
                type="number"
                value={rtoCount}
                onChange={(e) => setRtoCount(Number(e.target.value))}
                className="w-full px-3 py-2 rounded-lg bg-slate-950 border border-slate-700 text-sm text-white"
              />
            </div>
          </div>
          <button
            onClick={runSimulation}
            className="w-full py-2.5 rounded-lg bg-emerald-500 hover:bg-emerald-400 text-slate-950 font-semibold text-sm transition"
          >
            Evaluate Global Fraud Risk (&lt; 2ms)
          </button>
        </div>
      </div>

      <div className="p-6 rounded-2xl bg-slate-900 border border-slate-800">
        <h2 className="text-lg font-semibold text-white mb-4">
          Zero-Knowledge Network Assessment
        </h2>
        {result ? (
          <div className="space-y-3 text-sm">
            <div className="flex items-center justify-between p-3 rounded-xl bg-slate-950 border border-slate-800">
              <span>Risk Score & Tier</span>
              <span className="font-bold text-emerald-400">
                {result.riskScore}/100 ({result.riskTier})
              </span>
            </div>
            <div className="p-3 rounded-xl bg-slate-950 border border-slate-800">
              <div className="text-xs text-slate-400">Dynamic COD Intervention</div>
              <div className="font-mono text-indigo-400 font-semibold mt-0.5">
                {result.intervention}
              </div>
              <p className="text-slate-300 text-xs mt-1">{result.message}</p>
            </div>
          </div>
        ) : (
          <p className="text-slate-400 text-sm">
            Click &ldquo;Evaluate Global Fraud Risk&rdquo; to inspect the customer fingerprint and Dynamic COD Intervention.
          </p>
        )}
      </div>
    </section>
  );
}
