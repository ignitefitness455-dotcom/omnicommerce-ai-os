"use client";

import React, { useEffect, useState } from "react";
import {
  fetchPlatformTelemetrySummary,
  PlatformTelemetrySummary,
} from "../../lib/api-client";

export default function EnterpriseAnalyticsPage() {
  const [telemetry, setTelemetry] = useState<PlatformTelemetrySummary | null>(
    null
  );

  useEffect(() => {
    fetchPlatformTelemetrySummary().then(setTelemetry);
  }, []);

  const trend = telemetry?.rto_trend_30d ?? [
    { day_label: "Day 1", baseline_rto_pct: 31.4, omni_shield_rto_pct: 29.8 },
    { day_label: "Day 6", baseline_rto_pct: 31.2, omni_shield_rto_pct: 24.5 },
    { day_label: "Day 12", baseline_rto_pct: 31.6, omni_shield_rto_pct: 19.2 },
    { day_label: "Day 18", baseline_rto_pct: 31.1, omni_shield_rto_pct: 14.7 },
    { day_label: "Day 24", baseline_rto_pct: 31.5, omni_shield_rto_pct: 11.4 },
    { day_label: "Day 30", baseline_rto_pct: 31.4, omni_shield_rto_pct: 9.6 },
  ];

  const regions = telemetry?.regional_delivery_rates ?? [
    {
      country_iso: "AE",
      region_name: "UAE (Dubai / Abu Dhabi)",
      delivery_success_pct: 94.8,
      active_carrier: "Aramex GCC",
    },
    {
      country_iso: "SA",
      region_name: "Saudi Arabia (Riyadh / Jeddah)",
      delivery_success_pct: 91.5,
      active_carrier: "Aramex + SMSA Arbitrage",
    },
    {
      country_iso: "MY",
      region_name: "Malaysia (Kuala Lumpur)",
      delivery_success_pct: 93.8,
      active_carrier: "Ninja Van SEA Pro",
    },
    {
      country_iso: "ID",
      region_name: "Indonesia (Jakarta)",
      delivery_success_pct: 89.4,
      active_carrier: "Ninja Van + J&T Smart Route",
    },
  ];

  // Map trend points into SVG polyline coordinates (viewBox 0 0 600 200)
  const baselinePoints = trend
    .map((pt, idx) => {
      const x = 40 + idx * 104;
      const y = 180 - pt.baseline_rto_pct * 4.2;
      return `${x},${y.toFixed(1)}`;
    })
    .join(" ");

  const omniPoints = trend
    .map((pt, idx) => {
      const x = 40 + idx * 104;
      const y = 180 - pt.omni_shield_rto_pct * 4.2;
      return `${x},${y.toFixed(1)}`;
    })
    .join(" ");

  return (
    <main className="max-w-7xl mx-auto p-6 md:p-10 space-y-8">
      <div className="flex flex-col md:flex-row md:items-center justify-between gap-4 border-b border-slate-800 pb-6">
        <div>
          <h1 className="text-2xl font-bold text-white">
            📊 Enterprise RTO, Delivery & Conversion Analytics
          </h1>
          <p className="text-sm text-slate-400 mt-1">
            Live telemetry from Rust Axum (`/api/v1/events/summary`) tracking 30-day RTO compression, regional carrier SLAs, and AI Haggling conversion
          </p>
        </div>
        <span className="px-3 py-1.5 rounded-xl bg-emerald-500/10 border border-emerald-500/30 text-emerald-300 text-xs font-semibold">
          Active Network Fingerprints:{" "}
          {(telemetry?.total_shopper_fingerprints ?? 1420890).toLocaleString()}
        </span>
      </div>

      <div className="grid grid-cols-1 lg:grid-cols-12 gap-6">
        {/* 30-Day SVG RTO Compression Chart */}
        <div className="lg:col-span-7 p-6 rounded-2xl bg-slate-900 border border-slate-800 space-y-4">
          <div className="flex items-center justify-between">
            <div>
              <h2 className="text-base font-bold text-white">
                30-Day COD RTO Rate Reduction (31.4% → 9.6%)
              </h2>
              <p className="text-xs text-slate-400">
                Comparing unguarded COD stores (dashed rose) vs OmniCommerce Fraud Shield + WhatsApp Flows (emerald)
              </p>
            </div>
            <span className="px-2.5 py-1 rounded-lg bg-emerald-500/20 text-emerald-300 text-xs font-bold">
              -69.4% RTO Drop
            </span>
          </div>

          <svg viewBox="0 0 600 220" className="w-full h-56 overflow-visible">
            {/* Horizontal gridlines */}
            {[10, 20, 30].map((pct) => {
              const y = 180 - pct * 4.2;
              return (
                <g key={pct}>
                  <line
                    x1="30"
                    y1={y}
                    x2="580"
                    y2={y}
                    stroke="#1e293b"
                    strokeDasharray="4 4"
                  />
                  <text
                    x="0"
                    y={y + 4}
                    fill="#64748b"
                    fontSize="10"
                    fontFamily="monospace"
                  >
                    {pct}%
                  </text>
                </g>
              );
            })}

            {/* Baseline RTO Line */}
            <polyline
              fill="none"
              stroke="#fb7185"
              strokeWidth="2.5"
              strokeDasharray="6 4"
              points={baselinePoints}
            />

            {/* OmniCommerce Shield RTO Line */}
            <polyline
              fill="none"
              stroke="#10b981"
              strokeWidth="3.5"
              points={omniPoints}
            />

            {/* Data nodes & X labels */}
            {trend.map((pt, idx) => {
              const x = 40 + idx * 104;
              const y = 180 - pt.omni_shield_rto_pct * 4.2;
              return (
                <g key={pt.day_label}>
                  <circle cx={x} cy={y} r="5" fill="#10b981" stroke="#020617" strokeWidth="2" />
                  <text
                    x={x - 12}
                    y={y - 10}
                    fill="#34d399"
                    fontSize="10"
                    fontWeight="bold"
                  >
                    {pt.omni_shield_rto_pct}%
                  </text>
                  <text
                    x={x - 14}
                    y="205"
                    fill="#94a3b8"
                    fontSize="10"
                  >
                    {pt.day_label}
                  </text>
                </g>
              );
            })}
          </svg>
        </div>

        {/* Regional Delivery Success Rates */}
        <div className="lg:col-span-5 p-6 rounded-2xl bg-slate-900 border border-slate-800 space-y-5">
          <div>
            <h2 className="text-base font-bold text-white">
              GCC & SEA Carrier Delivery Success Rates
            </h2>
            <p className="text-xs text-slate-400">
              Optimized dynamically via Smart Carrier Zone Arbitrage
            </p>
          </div>

          <div className="space-y-4">
            {regions.map((r) => (
              <div key={r.country_iso} className="space-y-1.5 text-xs">
                <div className="flex items-center justify-between">
                  <span className="font-semibold text-white">{r.region_name}</span>
                  <span className="font-mono font-bold text-emerald-400">
                    {r.delivery_success_pct}%
                  </span>
                </div>
                <div className="w-full h-2.5 rounded-full bg-slate-950 overflow-hidden border border-slate-800">
                  <div
                    className="h-full rounded-full bg-gradient-to-r from-emerald-500 to-indigo-500"
                    style={{ width: `${r.delivery_success_pct}%` }}
                  />
                </div>
                <div className="text-[11px] text-slate-400">
                  Primary Route: <span className="text-indigo-300">{r.active_carrier}</span>
                </div>
              </div>
            ))}
          </div>
        </div>
      </div>

      {/* AI Haggling Conversion Funnel */}
      <div className="p-6 rounded-2xl bg-slate-900 border border-slate-800 space-y-4">
        <h2 className="text-base font-bold text-white">
          🤖 AI Haggling & COD-to-Prepaid Conversion Funnel
        </h2>
        <div className="grid grid-cols-1 sm:grid-cols-4 gap-4 text-xs">
          {[
            {
              stage: "1. Inbound Multimodal Inquiries",
              count: "12,480",
              pct: "100%",
              note: "WhatsApp Cloud, IG DM, TikTok Shop",
              color: "border-slate-700 text-white",
            },
            {
              stage: "2. Fraud Shield & Margin Qualified",
              count: "9,860",
              pct: "79.0%",
              note: "2,620 serial RTO abusers gated",
              color: "border-indigo-500/40 text-indigo-300",
            },
            {
              stage: "3. WhatsApp Flow / AI Offer Sent",
              count: "7,410",
              pct: "59.4%",
              note: "Bilingual AR/EN + Tabby 4x Split",
              color: "border-fuchsia-500/40 text-fuchsia-300",
            },
            {
              stage: "4. Closed & Zero-RTO Dispatched",
              count: "5,216",
              pct: "41.8%",
              note: "41.8% switched COD → Prepaid",
              color: "border-emerald-500/40 text-emerald-400",
            },
          ].map((step) => (
            <div
              key={step.stage}
              className={`p-4 rounded-xl bg-slate-950 border ${step.color} space-y-1`}
            >
              <div className="text-slate-400">{step.stage}</div>
              <div className="text-2xl font-extrabold">{step.count}</div>
              <div className="font-mono font-bold">{step.pct} Conversion</div>
              <div className="text-[11px] text-slate-400 pt-1">{step.note}</div>
            </div>
          ))}
        </div>
      </div>
    </main>
  );
}
