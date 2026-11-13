import React from "react";

export default function LegalCompliancePage() {
  return (
    <main className="max-w-5xl mx-auto p-6 md:p-10 space-y-8">
      <div className="border-b border-slate-800 pb-6">
        <span className="text-xs font-bold uppercase tracking-wider text-emerald-400">
          Enterprise Trust, Privacy & App Store Compliance
        </span>
        <h1 className="text-3xl font-bold text-white mt-1">
          📜 Privacy Policy, GDPR / Saudi PDPL, Terms of Service & DPA
        </h1>
        <p className="text-sm text-slate-400 mt-1">
          Last Updated: November 2026 • Published for Meta WhatsApp Business API, Shopify App Store & Salla App Store Verification
        </p>
      </div>

      <section className="p-6 rounded-2xl bg-slate-900 border border-slate-800 space-y-3 text-xs leading-relaxed text-slate-300">
        <h2 className="text-lg font-bold text-white">
          1. Privacy Policy & Zero-Knowledge Fraud Network (GDPR & Saudi PDPL)
        </h2>
        <p>
          OmniCommerce AI OS processes shopper identifiers strictly under **Privacy-by-Design** architecture compliant with the European Union **General Data Protection Regulation (GDPR)**, the Kingdom of Saudi Arabia **Personal Data Protection Law (PDPL)**, and the **UAE Federal Decree-Law No. 45**.
        </p>
        <ul className="list-disc pl-5 space-y-1.5 text-slate-300">
          <li>
            <strong>Cryptographic PII Isolation:</strong> Cross-merchant COD Return-to-Origin (RTO) intelligence operates solely on normalized E.164 phone numbers transformed via non-reversible salted <code>HMAC-SHA256</code> fingerprints.
          </li>
          <li>
            <strong>Zero Cross-Tenant PII Exposure:</strong> Merchant A can never query, view, or export customer names, phone numbers, or street addresses belonging to Merchant B. Only anonymized risk scores (0–100) and intervention tiers are returned.
          </li>
          <li>
            <strong>Right to Erasure:</strong> Shoppers or merchants may request instant cryptographic purge of tenant order records via <code>privacy@omnicommerce.ai</code>.
          </li>
        </ul>
      </section>

      <section className="p-6 rounded-2xl bg-slate-900 border border-slate-800 space-y-3 text-xs leading-relaxed text-slate-300">
        <h2 className="text-lg font-bold text-white">
          2. Terms of Service & 99.95% Enterprise Uptime SLA
        </h2>
        <p>
          OmniCommerce AI OS provides automated social commerce orchestration, fraud scoring, and carrier webhook dispatching under a **99.95% monthly availability SLA** for Scale and Enterprise subscribers.
        </p>
        <ul className="list-disc pl-5 space-y-1.5">
          <li>
            <strong>Merchant Margin Guardrails:</strong> Merchants are responsible for maintaining accurate unit COGS and minimum margin floors inside their catalog dashboard.
          </li>
          <li>
            <strong>WhatsApp & Storefront Compliance:</strong> Automated WhatsApp Flows and 24-hour recovery messages strictly adhere to Meta WhatsApp Business Messaging Policies and regional opt-in laws.
          </li>
        </ul>
      </section>

      <section className="p-6 rounded-2xl bg-slate-900 border border-slate-800 space-y-3 text-xs leading-relaxed text-slate-300">
        <h2 className="text-lg font-bold text-white">
          3. Data Processing Agreement (DPA) & Subscription Refund Policy
        </h2>
        <p>
          A standard GDPR/PDPL Data Processing Addendum is automatically incorporated into every Growth ($79/mo), Scale ($249/mo), and Enterprise ($699/mo) subscription. All plans include a **14-day risk-free trial** and prorated cancellation support with zero lock-in fees.
        </p>
      </section>
    </main>
  );
}
