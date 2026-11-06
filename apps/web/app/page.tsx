import React from "react";
import { FraudShieldConsole } from "../components/FraudShieldConsole";

export default function OmniDashboardHomePage() {
  return (
    <main className="min-h-screen bg-slate-950 text-slate-100 p-6 md:p-10">
      <div className="max-w-7xl mx-auto space-y-8">
        <header className="flex flex-col md:flex-row md:items-center justify-between gap-4 border-b border-slate-800 pb-6">
          <div>
            <div className="inline-flex items-center gap-2 px-3 py-1 rounded-full bg-emerald-500/10 border border-emerald-500/30 text-emerald-400 text-xs font-semibold mb-2">
              GLOBAL SOCIAL COMMERCE & RTO SHIELD • GCC / SEA / EU
            </div>
            <h1 className="text-3xl font-bold tracking-tight text-white">
              OmniCommerce AI OS
            </h1>
            <p className="text-slate-400 text-sm mt-1">
              Zero-Knowledge COD Fraud Shield • Multilingual GPT-4o Vision Agent • Aramex / NinjaVan / EasyPost Sync
            </p>
          </div>
          <div className="flex items-center gap-3">
            <span className="px-3 py-1.5 rounded-lg bg-slate-900 border border-slate-800 text-xs text-slate-300">
              Compliance: <strong className="text-emerald-400">GDPR & Saudi PDPL</strong>
            </span>
            <span className="px-3 py-1.5 rounded-lg bg-indigo-600 text-white text-xs font-semibold">
              Rust Axum v0.7 Connected
            </span>
          </div>
        </header>

        {/* Client Interactive Console pushed to leaf component per Next.js RSC best practices */}
        <FraudShieldConsole />
      </div>
    </main>
  );
}
