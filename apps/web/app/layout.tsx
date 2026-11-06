import React from "react";
import Link from "next/link";

export const metadata = {
  title: "OmniCommerce AI OS — Global Social Commerce & COD Fraud Shield",
  description:
    "Zero-Knowledge Fraud Shield, Margin-Protected AI Deal Closer, WhatsApp Flows, and Multi-Carrier Auto-Logistics for GCC, SEA & Global D2C Brands.",
};

export default function RootLayout({
  children,
}: {
  children: React.ReactNode;
}) {
  return (
    <html lang="en" className="dark">
      <body className="min-h-screen bg-slate-950 text-slate-100 flex flex-col">
        <nav className="border-b border-slate-800 bg-slate-900/80 backdrop-blur px-6 py-3.5">
          <div className="max-w-7xl mx-auto flex flex-wrap items-center justify-between gap-4">
            <div className="flex items-center gap-3">
              <span className="h-3 w-3 rounded-full bg-emerald-400 animate-pulse" />
              <span className="font-bold text-white tracking-tight">
                OmniCommerce AI OS
              </span>
              <span className="text-xs px-2 py-0.5 rounded bg-emerald-500/10 text-emerald-400 border border-emerald-500/30">
                GCC • SEA • GLOBAL
              </span>
            </div>
            <div className="flex flex-wrap items-center gap-5 text-xs font-medium text-slate-300">
              <Link href="/" className="hover:text-emerald-400 transition">
                🛡️ Command Center
              </Link>
              <Link href="/orders" className="hover:text-emerald-400 transition">
                📦 Live Orders & AWBs
              </Link>
              <Link href="/catalog" className="hover:text-emerald-400 transition">
                🏷️ Catalog & Margin Guardrails
              </Link>
              <Link href="/auth" className="hover:text-emerald-400 transition">
                🔑 Merchant Onboarding & API Keys
              </Link>
              <Link href="/settings" className="hover:text-emerald-400 transition">
                ⚙️ Integrations (Meta/Stripe/Carriers)
              </Link>
            </div>
          </div>
        </nav>
        <div className="flex-1">{children}</div>
      </body>
    </html>
  );
}
