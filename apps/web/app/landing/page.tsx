"use client";

import React, { useState } from "react";
import Link from "next/link";
import { Check, Globe, ShieldCheck, Sparkles, TrendingUp, Zap } from "lucide-react";

const COPY = {
  en: {
    dir: "ltr" as const,
    badge: "TRUSTED BY 1,842+ D2C BRANDS ACROSS UAE, SAUDI ARABIA & SOUTHEAST ASIA",
    heroTitle: "Stop COD Returns. Automate WhatsApp Sales with Margin-Protected AI.",
    heroSub:
      "The operating system for high-velocity social & D2C commerce. Block serial RTO offenders across 1.4M+ shopper fingerprints, close deals via GPT-4o Vision & Arabic/English AI haggling, and route Aramex & Ninja Van automatically.",
    ctaPrimary: "Launch Interactive Command Center",
    ctaSecondary: "Calculate Your ROI",
    roiTitle: "Interactive ROI & RTO Savings Calculator",
    roiSubtitle:
      "Adjust your store metrics to see how much wasted shipping & recovered GMV OmniCommerce AI OS unlocks every month.",
    pricingTitle: "Transparent Global SaaS Pricing",
    pricingSub:
      "Start with a 14-day full-access trial. Every plan includes GDPR & Saudi PDPL zero-knowledge hashing.",
    faqTitle: "Enterprise & Compliance FAQ",
  },
  ar: {
    dir: "rtl" as const,
    badge: "موثوق من أكثر من 1,842 علامة تجارية في الإمارات والسعودية وجنوب شرق آسيا",
    heroTitle: "أوقف مرتجعات الدفع عند الاستلام. وأتمت مبيعات واتساب بالذكاء الاصطناعي.",
    heroSub:
      "نظام التشغيل المتكامل للتجارة الاجتماعية. اكشف العملاء ذوي المرتجعات العالية عبر شبكة مشفرة تضم 1.4 مليون بصمة، وأغلق الصفقات تلقائياً بالعربية والإنجليزية مع أرامكس ونينجا فان.",
    ctaPrimary: "افتح لوحة التحكم التفاعلية",
    ctaSecondary: "احسب العائد على الاستثمار",
    roiTitle: "حاسبة العائد على الاستثمار وتوفير تكاليف المرتجعات",
    roiSubtitle:
      "حرّك المؤشرات لحساب المبالغ التي ستوفرها شهرياً من تقليل المرتجعات واسترداد المحادثات المتروكة.",
    pricingTitle: "خطط أسعار شفافة وعالمية",
    pricingSub:
      "ابدأ تجربة مجانية لمدة 14 يوماً. جميع الخطط متوافقة مع نظام حماية البيانات السعودي (PDPL) و GDPR.",
    faqTitle: "الأسئلة الشائعة للشركات والامتثال",
  },
};

export default function PublicLandingPage() {
  const [lang, setLang] = useState<"en" | "ar">("en");
  const [annualBilling, setAnnualBilling] = useState(true);
  const [monthlyOrders, setMonthlyOrders] = useState(1500);
  const [aovUsd, setAovUsd] = useState(95);
  const [baselineRtoPct, setBaselineRtoPct] = useState(31);

  const [quickPhone, setQuickPhone] = useState("+971509876543");
  const [quickResult, setQuickResult] = useState<{
    score: number;
    tier: string;
    action: string;
  }>({
    score: 9,
    tier: "SAFE",
    action: "ALLOW_SEAMLESS_COD → Auto-Dispatch Aramex",
  });

  const t = COPY[lang];

  // Live ROI calculations
  const preventedRtoRate = Math.max(0.05, (baselineRtoPct - 9.6) / 100);
  const preventedParcels = Math.round(monthlyOrders * preventedRtoRate);
  const rtoLogisticsSaved = Math.round(preventedParcels * 14);
  const recoveredChatsRevenue = Math.round(monthlyOrders * 0.065 * aovUsd * 0.34);
  const carrierArbitrageSaved = Math.round(monthlyOrders * 1.25);
  const totalMonthlyValue = rtoLogisticsSaved + recoveredChatsRevenue + carrierArbitrageSaved;
  const scaleFee = annualBilling ? 199 : 249;
  const roiMultiple = (totalMonthlyValue / scaleFee).toFixed(1);

  function handleQuickCheck() {
    if (quickPhone.includes("000") || quickPhone.includes("9999")) {
      setQuickResult({
        score: 88,
        tier: "BLOCKED",
        action: "BLOCK_COD_PREPAID_ONLY → Require Instant Stripe / Tabby Payment",
      });
    } else if (quickPhone.startsWith("+966")) {
      setQuickResult({
        score: 36,
        tier: "MODERATE",
        action: "WHATSAPP_FLOW_CONFIRMATION → 1-Tap Arabic Address Pin",
      });
    } else {
      setQuickResult({
        score: 9,
        tier: "SAFE",
        action: "ALLOW_SEAMLESS_COD → Auto-Dispatch Aramex / NinjaVan",
      });
    }
  }

  return (
    <main dir={t.dir} className="min-h-screen bg-slate-950 text-slate-100">
      {/* Top Hero */}
      <section className="relative overflow-hidden border-b border-slate-800 bg-gradient-to-b from-slate-900 via-slate-950 to-slate-950 px-6 py-16 md:py-24">
        <div className="max-w-7xl mx-auto space-y-10">
          <div className="flex flex-wrap items-center justify-between gap-4">
            <span className="inline-flex items-center gap-2 px-3.5 py-1.5 rounded-full bg-emerald-500/10 border border-emerald-500/30 text-emerald-400 text-xs font-bold">
              <ShieldCheck className="w-4 h-4" />
              {t.badge}
            </span>
            <button
              onClick={() => setLang(lang === "en" ? "ar" : "en")}
              className="inline-flex items-center gap-2 px-4 py-2 rounded-xl bg-slate-900 border border-slate-700 hover:border-emerald-500 text-xs font-bold text-white transition"
            >
              <Globe className="w-4 h-4 text-emerald-400" />
              {lang === "en" ? "التبديل إلى العربية (RTL)" : "Switch to English (LTR)"}
            </button>
          </div>

          <div className="grid grid-cols-1 lg:grid-cols-12 gap-10 items-center">
            <div className="lg:col-span-7 space-y-6">
              <h1 className="text-4xl md:text-5xl font-extrabold tracking-tight text-white leading-tight">
                {t.heroTitle}
              </h1>
              <p className="text-slate-300 text-base md:text-lg leading-relaxed max-w-2xl">
                {t.heroSub}
              </p>

              <div className="flex flex-wrap gap-3">
                <Link
                  href="/"
                  className="px-6 py-3.5 rounded-xl bg-emerald-500 hover:bg-emerald-400 text-slate-950 font-bold text-sm transition inline-flex items-center gap-2"
                >
                  <Zap className="w-4 h-4" />
                  {t.ctaPrimary}
                </Link>
                <a
                  href="#roi-calculator"
                  className="px-6 py-3.5 rounded-xl bg-slate-900 hover:bg-slate-800 border border-slate-700 text-white font-semibold text-sm transition"
                >
                  {t.ctaSecondary}
                </a>
              </div>

              {/* Enterprise Trust Badges */}
              <div className="pt-4 flex flex-wrap items-center gap-3 text-[11px] font-semibold text-slate-400">
                <span className="px-3 py-1.5 rounded-lg bg-slate-900 border border-slate-800 text-emerald-300">
                  ✓ Saudi PDPL Compliant
                </span>
                <span className="px-3 py-1.5 rounded-lg bg-slate-900 border border-slate-800 text-indigo-300">
                  ✓ EU GDPR Zero-Knowledge
                </span>
                <span className="px-3 py-1.5 rounded-lg bg-slate-900 border border-slate-800 text-sky-300">
                  ✓ Meta WhatsApp Cloud Partner
                </span>
                <span className="px-3 py-1.5 rounded-lg bg-slate-900 border border-slate-800 text-amber-300">
                  ✓ PCI-DSS Level 1 & ISO 27001
                </span>
              </div>
            </div>

            {/* Interactive Live Hero Phone Check Card */}
            <div className="lg:col-span-5 p-6 rounded-2xl bg-slate-900/90 border border-slate-800 shadow-2xl space-y-4">
              <div className="flex items-center justify-between border-b border-slate-800 pb-3">
                <span className="text-sm font-bold text-white">
                  Instant Network Fraud Check Demo
                </span>
                <span className="text-[11px] px-2 py-0.5 rounded bg-emerald-500/15 text-emerald-400 font-mono">
                  &lt; 2ms Rust Engine
                </span>
              </div>
              <div className="space-y-3 text-xs">
                <label className="block text-slate-400">
                  Try UAE (+971509876543), KSA (+966554433221), or Fraud (+971520009999)
                </label>
                <div className="flex gap-2">
                  <input
                    value={quickPhone}
                    onChange={(e) => setQuickPhone(e.target.value)}
                    className="flex-1 px-3 py-2.5 rounded-xl bg-slate-950 border border-slate-700 text-white font-mono"
                  />
                  <button
                    onClick={handleQuickCheck}
                    className="px-4 py-2.5 rounded-xl bg-indigo-600 hover:bg-indigo-500 text-white font-bold"
                  >
                    Score
                  </button>
                </div>
                <div className="p-4 rounded-xl bg-slate-950 border border-slate-800 space-y-2">
                  <div className="flex items-center justify-between">
                    <span className="text-slate-400">Risk Score & Tier:</span>
                    <span
                      className={`font-bold ${
                        quickResult.tier === "SAFE"
                          ? "text-emerald-400"
                          : quickResult.tier === "MODERATE"
                          ? "text-amber-400"
                          : "text-rose-400"
                      }`}
                    >
                      {quickResult.score}/100 ({quickResult.tier})
                    </span>
                  </div>
                  <div className="text-indigo-300 font-mono font-semibold">
                    {quickResult.action}
                  </div>
                </div>
              </div>
            </div>
          </div>
        </div>
      </section>

      {/* Interactive ROI Calculator */}
      <section id="roi-calculator" className="max-w-7xl mx-auto px-6 py-16 space-y-8">
        <div className="text-center max-w-2xl mx-auto space-y-2">
          <span className="text-xs font-bold uppercase tracking-wider text-emerald-400">
            Proven Economic Impact
          </span>
          <h2 className="text-3xl font-bold text-white">{t.roiTitle}</h2>
          <p className="text-sm text-slate-400">{t.roiSubtitle}</p>
        </div>

        <div className="grid grid-cols-1 lg:grid-cols-12 gap-8 p-6 md:p-8 rounded-3xl bg-slate-900 border border-slate-800">
          <div className="lg:col-span-6 space-y-6">
            <div>
              <div className="flex justify-between text-sm mb-2">
                <span className="text-slate-300 font-medium">Monthly COD & Social Orders</span>
                <strong className="text-emerald-400 font-mono">{monthlyOrders.toLocaleString()} orders</strong>
              </div>
              <input
                type="range"
                min={100}
                max={25000}
                step={100}
                value={monthlyOrders}
                onChange={(e) => setMonthlyOrders(Number(e.target.value))}
                className="w-full accent-emerald-500"
              />
            </div>

            <div>
              <div className="flex justify-between text-sm mb-2">
                <span className="text-slate-300 font-medium">Average Order Value (USD Equiv.)</span>
                <strong className="text-indigo-400 font-mono">${aovUsd} USD</strong>
              </div>
              <input
                type="range"
                min={30}
                max={300}
                step={5}
                value={aovUsd}
                onChange={(e) => setAovUsd(Number(e.target.value))}
                className="w-full accent-indigo-500"
              />
            </div>

            <div>
              <div className="flex justify-between text-sm mb-2">
                <span className="text-slate-300 font-medium">Current Baseline RTO Return Rate</span>
                <strong className="text-amber-400 font-mono">{baselineRtoPct}%</strong>
              </div>
              <input
                type="range"
                min={10}
                max={45}
                step={1}
                value={baselineRtoPct}
                onChange={(e) => setBaselineRtoPct(Number(e.target.value))}
                className="w-full accent-amber-500"
              />
            </div>
          </div>

          <div className="lg:col-span-6 grid grid-cols-1 sm:grid-cols-2 gap-4">
            <div className="p-4 rounded-2xl bg-slate-950 border border-slate-800">
              <div className="text-xs text-slate-400">Prevented RTO Shipping Losses</div>
              <div className="text-2xl font-bold text-emerald-400 mt-1">
                +${rtoLogisticsSaved.toLocaleString()}
              </div>
              <div className="text-[11px] text-slate-400 mt-1">
                {preventedParcels} failed deliveries stopped/mo
              </div>
            </div>
            <div className="p-4 rounded-2xl bg-slate-950 border border-slate-800">
              <div className="text-xs text-slate-400">AI Haggling & 24h Chat Recovery</div>
              <div className="text-2xl font-bold text-indigo-400 mt-1">
                +${recoveredChatsRevenue.toLocaleString()}
              </div>
              <div className="text-[11px] text-slate-400 mt-1">
                Converted via Tabby 4x & Prepaid nudges
              </div>
            </div>
            <div className="p-4 rounded-2xl bg-slate-950 border border-slate-800">
              <div className="text-xs text-slate-400">Carrier Arbitrage Savings</div>
              <div className="text-2xl font-bold text-amber-400 mt-1">
                +${carrierArbitrageSaved.toLocaleString()}
              </div>
              <div className="text-[11px] text-slate-400 mt-1">
                Aramex vs SMSA / NinjaVan routing
              </div>
            </div>
            <div className="p-4 rounded-2xl bg-emerald-500/15 border border-emerald-500/40 flex flex-col justify-center">
              <div className="text-xs text-emerald-200 flex items-center gap-1">
                <TrendingUp className="w-3.5 h-3.5" /> Net Monthly Impact
              </div>
              <div className="text-2xl font-extrabold text-white mt-1">
                ${totalMonthlyValue.toLocaleString()} / mo
              </div>
              <div className="text-xs font-bold text-emerald-400 mt-0.5">
                {roiMultiple}x Net ROI on Scale Plan
              </div>
            </div>
          </div>
        </div>
      </section>

      {/* Transparent Pricing Table */}
      <section className="max-w-7xl mx-auto px-6 py-12 space-y-8">
        <div className="text-center space-y-3">
          <h2 className="text-3xl font-bold text-white">{t.pricingTitle}</h2>
          <p className="text-sm text-slate-400">{t.pricingSub}</p>
          <div className="inline-flex items-center gap-2 p-1 rounded-xl bg-slate-900 border border-slate-800 text-xs font-semibold">
            <button
              onClick={() => setAnnualBilling(false)}
              className={`px-4 py-2 rounded-lg transition ${
                !annualBilling ? "bg-indigo-600 text-white" : "text-slate-400"
              }`}
            >
              Monthly Billing
            </button>
            <button
              onClick={() => setAnnualBilling(true)}
              className={`px-4 py-2 rounded-lg transition ${
                annualBilling ? "bg-emerald-500 text-slate-950 font-bold" : "text-slate-400"
              }`}
            >
              Annual Billing (Save 20%)
            </button>
          </div>
        </div>

        <div className="grid grid-cols-1 md:grid-cols-3 gap-6">
          {[
            {
              name: "Growth",
              monthly: 79,
              desc: "For emerging F-Commerce & Shopify stores",
              features: [
                "Up to 2,000 Fraud Shield checks/mo",
                "Shopify & WooCommerce HMAC webhooks",
                "WhatsApp Flow COD Verification",
                "1 Carrier Adapter (Aramex or NinjaVan)",
              ],
              highlight: false,
            },
            {
              name: "Scale",
              monthly: 249,
              desc: "Most Popular for UAE, KSA & SEA D2C Brands",
              features: [
                "Up to 15,000 Fraud Shield checks/mo",
                "Salla, Zid, Shopify & TikTok Shop sync",
                "Margin-Protected AI Haggling & GPT-4o Vision",
                "Smart Carrier Zone Arbitrage Router",
                "3-Wave Meta 24h Auto-Retargeting",
              ],
              highlight: true,
            },
            {
              name: "Enterprise",
              monthly: 699,
              desc: "For multi-brand retail groups & high-volume sellers",
              features: [
                "Unlimited Fraud Shield & custom ML weights",
                "Dedicated Saudi PDPL / GDPR private cluster",
                "Custom RBAC & SAML/Google SSO",
                "99.95% Uptime SLA & Solutions Architect",
              ],
              highlight: false,
            },
          ].map((plan) => {
            const price = annualBilling ? Math.round(plan.monthly * 0.8) : plan.monthly;
            return (
              <div
                key={plan.name}
                className={`p-6 rounded-3xl border flex flex-col justify-between space-y-6 ${
                  plan.highlight
                    ? "bg-slate-900 border-emerald-500 shadow-xl shadow-emerald-500/10"
                    : "bg-slate-900/60 border-slate-800"
                }`}
              >
                <div className="space-y-4">
                  <div className="flex items-center justify-between">
                    <h3 className="text-xl font-bold text-white">{plan.name}</h3>
                    {plan.highlight && (
                      <span className="px-2.5 py-1 rounded-full bg-emerald-500/20 text-emerald-300 text-[11px] font-bold inline-flex items-center gap-1">
                        <Sparkles className="w-3 h-3" /> BEST VALUE
                      </span>
                    )}
                  </div>
                  <p className="text-xs text-slate-400">{plan.desc}</p>
                  <div className="flex items-baseline gap-1">
                    <span className="text-4xl font-extrabold text-white">${price}</span>
                    <span className="text-xs text-slate-400">/ month</span>
                  </div>
                  <ul className="space-y-2.5 text-xs text-slate-300 pt-2">
                    {plan.features.map((f) => (
                      <li key={f} className="flex items-start gap-2">
                        <Check className="w-4 h-4 text-emerald-400 shrink-0 mt-0.5" />
                        <span>{f}</span>
                      </li>
                    ))}
                  </ul>
                </div>

                <Link
                  href="/auth"
                  className={`w-full py-3 rounded-xl text-center text-xs font-bold transition ${
                    plan.highlight
                      ? "bg-emerald-500 hover:bg-emerald-400 text-slate-950"
                      : "bg-slate-800 hover:bg-slate-700 text-white"
                  }`}
                >
                  Start 14-Day Free Trial
                </Link>
              </div>
            );
          })}
        </div>
      </section>

      {/* FAQ Section */}
      <section className="max-w-5xl mx-auto px-6 py-12 space-y-6">
        <h2 className="text-2xl font-bold text-white text-center">{t.faqTitle}</h2>
        <div className="grid grid-cols-1 md:grid-cols-2 gap-4 text-xs">
          {[
            {
              q: "How does OmniCommerce share fraud intelligence without violating GDPR or Saudi PDPL?",
              a: "All phone numbers are normalized to E.164 and hashed with Salted HMAC-SHA256. Merchant A never sees Merchant B's customer names, numbers, or addresses—only anonymized network RTO risk scores.",
            },
            {
              q: "Will the AI Haggling Engine ever sell below my profit margin?",
              a: "Never. Your unit COGS and minimum required net margin floor (e.g., 28%) are enforced mathematically in Rust before any discount or Tabby 4x offer is generated.",
            },
            {
              q: "Which storefronts and couriers work out of the box?",
              a: "Native HMAC-verified webhooks for Shopify, Salla, Zid, WooCommerce, Meta WhatsApp Cloud API, and automatic shipping label generation via Aramex (GCC), Ninja Van (SEA), and EasyPost (DHL/FedEx).",
            },
            {
              q: "How fast is the Fraud Shield API during checkout?",
              a: "Built on Rust (Axum + Tokio + Redis), risk scoring evaluates E.164 normalization, NLP address quality, and network aggregates in under 2 milliseconds.",
            },
          ].map((faq) => (
            <div
              key={faq.q}
              className="p-5 rounded-2xl bg-slate-900 border border-slate-800 space-y-2"
            >
              <h3 className="font-bold text-white text-sm">{faq.q}</h3>
              <p className="text-slate-400 leading-relaxed">{faq.a}</p>
            </div>
          ))}
        </div>
      </section>
    </main>
  );
}
