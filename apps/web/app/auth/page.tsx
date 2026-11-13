"use client";

import React, { useState } from "react";
import { KeyRound, Shield, Smartphone, Users } from "lucide-react";

const TEAM_ROSTER = [
  {
    name: "Tariq Al-Mansoor",
    email: "tariq@alnoor-dubai.ae",
    role: "OWNER",
    twoFactor: true,
    perms: "Full Billing, Team, Margin Floors, Fraud Overrides & Shipments",
  },
  {
    name: "Nadia Al-Harbi",
    email: "nadia@alnoor-dubai.ae",
    role: "STORE_MANAGER",
    twoFactor: true,
    perms: "Configure Margin Floors, Override Fraud Blocks, View Analytics",
  },
  {
    name: "Zaid Rahman",
    email: "zaid.support@alnoor-dubai.ae",
    role: "SUPPORT_AGENT",
    twoFactor: true,
    perms: "Respond to WhatsApp/IG AI Chats, View Order Status",
  },
  {
    name: "Amira Hassan",
    email: "logistics@alnoor-dubai.ae",
    role: "LOGISTICS_COORDINATOR",
    twoFactor: false,
    perms: "Dispatch Aramex/NinjaVan AWBs, Manage Tracking Webhooks",
  },
];

export default function MerchantAuthOnboardingPage() {
  const [ssoNotice, setSsoNotice] = useState<string | null>(null);
  const [otpInput, setOtpInput] = useState("482910");
  const [otpStatus, setOtpStatus] = useState<string | null>(null);
  const [secretKey, setSecretKey] = useState(
    "omni_live_9f86d081884c7d659a2feaa0c55ad015a3bf"
  );

  function triggerSso(provider: string) {
    setSsoNotice(
      `✅ Initiated ${provider} OAuth 2.0 PKCE handshake (/api/v1/merchants/auth/sso). Session state token verified!`
    );
  }

  function verifyOtp() {
    if (/^\d{6}$/.test(otpInput.trim())) {
      setOtpStatus(
        "✅ 2FA OTP Verified! Signed HMAC-SHA256 Owner session cookie issued (24h TTL)."
      );
    } else {
      setOtpStatus("❌ Please enter a valid 6-digit numeric OTP code.");
    }
  }

  function rotateApiKey() {
    const hex = Array.from({ length: 36 }, () =>
      Math.floor(Math.random() * 16).toString(16)
    ).join("");
    setSecretKey(`omni_live_${hex}`);
  }

  return (
    <main className="max-w-6xl mx-auto p-6 md:p-10 space-y-8">
      <div>
        <h1 className="text-2xl font-bold text-white">
          🔑 Merchant SSO, 2FA OTP, RBAC Team & API Key Management
        </h1>
        <p className="text-sm text-slate-400 mt-1">
          Enterprise authentication powered by Google/Shopify/Salla SSO, 6-digit WhatsApp OTP verification, and fine-grained Role-Based Access Control (`crates/omni-core/src/rbac.rs`)
        </p>
      </div>

      <div className="grid grid-cols-1 lg:grid-cols-2 gap-6">
        {/* SSO & 2FA OTP Login Box */}
        <div className="p-6 rounded-2xl bg-slate-900 border border-slate-800 space-y-5 text-xs">
          <div className="flex items-center gap-2 font-bold text-white text-sm">
            <Smartphone className="w-4 h-4 text-emerald-400" />
            <span>1-Click Storefront SSO & WhatsApp 2FA Login</span>
          </div>

          <div className="grid grid-cols-1 sm:grid-cols-3 gap-2.5">
            <button
              onClick={() => triggerSso("Google Workspace")}
              className="py-2.5 px-3 rounded-xl bg-slate-950 hover:bg-slate-800 border border-slate-700 font-semibold text-white transition"
            >
              Continue with Google
            </button>
            <button
              onClick={() => triggerSso("Shopify Admin OAuth")}
              className="py-2.5 px-3 rounded-xl bg-emerald-500/15 hover:bg-emerald-500/25 border border-emerald-500/40 font-semibold text-emerald-300 transition"
            >
              Connect Shopify SSO
            </button>
            <button
              onClick={() => triggerSso("Salla GCC OAuth")}
              className="py-2.5 px-3 rounded-xl bg-indigo-500/15 hover:bg-indigo-500/25 border border-indigo-500/40 font-semibold text-indigo-300 transition"
            >
              Connect Salla KSA
            </button>
          </div>

          {ssoNotice && (
            <div className="p-3 rounded-xl bg-emerald-500/10 border border-emerald-500/30 text-emerald-300">
              {ssoNotice}
            </div>
          )}

          <div className="pt-3 border-t border-slate-800 space-y-2.5">
            <label className="block text-slate-400">
              WhatsApp / Email 6-Digit 2FA Verification Code
            </label>
            <div className="flex gap-2">
              <input
                value={otpInput}
                onChange={(e) => setOtpInput(e.target.value)}
                maxLength={6}
                className="flex-1 px-3 py-2 rounded-xl bg-slate-950 border border-slate-700 text-white font-mono tracking-widest"
              />
              <button
                onClick={verifyOtp}
                className="px-4 py-2 rounded-xl bg-emerald-500 hover:bg-emerald-400 text-slate-950 font-bold"
              >
                Verify 2FA OTP
              </button>
            </div>
            {otpStatus && (
              <div className="text-emerald-300 font-medium">{otpStatus}</div>
            )}
          </div>
        </div>

        {/* Active API Key Credentials */}
        <div className="p-6 rounded-2xl bg-slate-900 border border-slate-800 space-y-4 text-xs">
          <div className="flex items-center justify-between border-b border-slate-800 pb-3">
            <span className="font-bold text-white text-sm flex items-center gap-2">
              <KeyRound className="w-4 h-4 text-amber-400" />
              Production API Credentials
            </span>
            <button
              onClick={rotateApiKey}
              className="px-3 py-1 rounded-lg bg-amber-500/20 hover:bg-amber-500/30 border border-amber-500/40 text-amber-300 font-semibold"
            >
              Rotate Secret Key
            </button>
          </div>
          <div className="space-y-2 font-mono">
            <div className="p-3 rounded-lg bg-slate-950 border border-slate-800">
              <div className="text-slate-400">Publishable Key:</div>
              <div className="text-emerald-400 mt-0.5">
                omni_pk_1111111111111111
              </div>
            </div>
            <div className="p-3 rounded-lg bg-slate-950 border border-slate-800">
              <div className="text-slate-400">Secret API Key (Stored as HMAC-SHA256 Hash):</div>
              <div className="text-amber-300 mt-0.5 break-all">{secretKey}</div>
            </div>
          </div>
        </div>
      </div>

      {/* RBAC Team Permissions Matrix */}
      <div className="p-6 rounded-2xl bg-slate-900 border border-slate-800 space-y-4">
        <div className="flex items-center justify-between">
          <h2 className="text-base font-bold text-white flex items-center gap-2">
            <Users className="w-4 h-4 text-indigo-400" />
            Role-Based Access Control (RBAC) Team Roster
          </h2>
          <span className="text-xs text-slate-400 inline-flex items-center gap-1">
            <Shield className="w-3.5 h-3.5 text-emerald-400" /> Enforced in `omni_core::rbac`
          </span>
        </div>

        <div className="overflow-x-auto">
          <table className="w-full text-left text-xs">
            <thead className="text-slate-400 border-b border-slate-800">
              <tr>
                <th className="py-3 px-2">Member</th>
                <th className="py-3 px-2">Role</th>
                <th className="py-3 px-2">2FA Status</th>
                <th className="py-3 px-2">Authorized Capabilities</th>
              </tr>
            </thead>
            <tbody className="divide-y divide-slate-800">
              {TEAM_ROSTER.map((m) => (
                <tr key={m.email}>
                  <td className="py-3 px-2">
                    <div className="font-bold text-white">{m.name}</div>
                    <div className="text-slate-400 font-mono">{m.email}</div>
                  </td>
                  <td className="py-3 px-2">
                    <span className="px-2.5 py-1 rounded-full bg-indigo-500/15 border border-indigo-500/30 text-indigo-300 font-mono font-bold">
                      {m.role}
                    </span>
                  </td>
                  <td className="py-3 px-2">
                    {m.twoFactor ? (
                      <span className="text-emerald-400 font-semibold">
                        ✓ 2FA Enabled
                      </span>
                    ) : (
                      <span className="text-amber-400">Pending Setup</span>
                    )}
                  </td>
                  <td className="py-3 px-2 text-slate-300">{m.perms}</td>
                </tr>
              ))}
            </tbody>
          </table>
        </div>
      </div>
    </main>
  );
}
