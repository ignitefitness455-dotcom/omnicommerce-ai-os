import React from "react";
import Link from "next/link";
import "./globals.css";
import { SupportChatWidget } from "../components/SupportChatWidget";

export const metadata = {
  title: "OmniCommerce AI OS — Global Social Commerce & COD Fraud Shield",
  description:
    "Zero-Knowledge Fraud Shield, Margin-Protected AI Deal Closer, WhatsApp Flows, and Multi-Carrier Auto-Logistics for GCC, SEA & Global D2C Brands.",
  manifest: "/manifest.json",
};

export default function RootLayout({
  children,
}: {
  children: React.ReactNode;
}) {
  return (
    <html lang="en" className="dark">
      <body className="min-h-screen bg-slate-950 text-slate-100 flex flex-col">
        <nav className="sticky top-0 z-40 border-b border-slate-800 bg-slate-900/90 backdrop-blur px-6 py-3.5">
          <div className="max-w-7xl mx-auto flex flex-wrap items-center justify-between gap-4">
            <div className="flex items-center gap-3">
              <span className="h-3 w-3 rounded-full bg-emerald-400 animate-pulse" />
              <Link href="/landing" className="font-bold text-white tracking-tight">
                OmniCommerce AI OS
              </Link>
              <span className="text-[11px] px-2 py-0.5 rounded bg-emerald-500/10 text-emerald-400 border border-emerald-500/30 font-semibold">
                GCC • SEA • GLOBAL
              </span>
            </div>
            <div className="flex flex-wrap items-center gap-4 text-xs font-medium text-slate-300">
              <Link href="/landing" className="hover:text-emerald-400 transition">
                🌐 Product & ROI
              </Link>
              <Link href="/" className="hover:text-emerald-400 transition">
                🛡️ Command Center
              </Link>
              <Link href="/orders" className="hover:text-emerald-400 transition">
                📦 Orders & AWBs
              </Link>
              <Link href="/catalog" className="hover:text-emerald-400 transition">
                🏷️ Catalog & Margin
              </Link>
              <Link href="/analytics" className="hover:text-emerald-400 transition">
                📊 Analytics
              </Link>
              <Link href="/auth" className="hover:text-emerald-400 transition">
                🔑 Auth & RBAC
              </Link>
              <Link href="/settings" className="hover:text-emerald-400 transition">
                ⚙️ Integrations
              </Link>
              <Link href="/legal" className="hover:text-emerald-400 transition">
                📜 Legal & PDPL
              </Link>
            </div>
          </div>
        </nav>

        <div className="flex-1">{children}</div>

        <footer className="border-t border-slate-800 bg-slate-950 px-6 py-8 text-xs text-slate-400">
          <div className="max-w-7xl mx-auto flex flex-col md:flex-row items-center justify-between gap-4">
            <div>
              © 2026 OmniCommerce AI OS. Built with Rust (Axum + Diesel) & Next.js.
              GDPR & Saudi PDPL Zero-Knowledge Compliant.
            </div>
            <div className="flex flex-wrap gap-5">
              <Link href="/landing" className="hover:text-white">
                Pricing & ROI
              </Link>
              <Link href="/legal" className="hover:text-white">
                Privacy Policy (GDPR/PDPL)
              </Link>
              <Link href="/legal" className="hover:text-white">
                Terms & 99.95% SLA
              </Link>
              <Link href="/settings" className="hover:text-white">
                API & Webhooks
              </Link>
            </div>
          </div>
        </footer>

        <SupportChatWidget />
      </body>
    </html>
  );
}
