import React from "react";

export default function LiveOrdersPage() {
  const orders = [
    {
      id: "ORD-DXB-77801",
      channel: "SHOPIFY_UAE",
      customer: "Sara Al-Maktoum (+971509876543)",
      amount: "420.00 AED",
      payment: "COD",
      score: "9/100 (SAFE)",
      intervention: "ALLOW_SEAMLESS_COD",
      awb: "ARMX-AE-77801DXB",
      status: "AUTO_DISPATCHED",
    },
    {
      id: "ORD-KSA-99412",
      channel: "SALLA_KSA",
      customer: "Faisal Al-Saud (+966554433221)",
      amount: "513.00 SAR (5% Off Switch)",
      payment: "TABBY_4X_PREPAID",
      score: "38/100 (MODERATE)",
      intervention: "WHATSAPP_FLOW_CONVERTED_PREPAID",
      awb: "ARMX-SA-PREPAID99",
      status: "CONVERTED_AND_DISPATCHED",
    },
    {
      id: "ORD-MY-33109",
      channel: "TIKTOK_SHOP_MY",
      customer: "Aiman Hakim (+60123456789)",
      amount: "185.00 MYR",
      payment: "COD",
      score: "12/100 (SAFE)",
      intervention: "ALLOW_SEAMLESS_COD",
      awb: "NJVMY33109KL",
      status: "AUTO_DISPATCHED",
    },
  ];

  return (
    <main className="max-w-7xl mx-auto p-6 md:p-10 space-y-6">
      <div>
        <h1 className="text-2xl font-bold text-white">
          📦 Live Omnichannel Orders & Carrier AWBs
        </h1>
        <p className="text-sm text-slate-400 mt-1">
          Real-time order stream across Shopify, Salla, Zid, WhatsApp Cloud API, and TikTok Shop
        </p>
      </div>

      <div className="overflow-x-auto rounded-2xl border border-slate-800 bg-slate-900">
        <table className="w-full text-left text-xs">
          <thead className="bg-slate-950 text-slate-400 border-b border-slate-800">
            <tr>
              <th className="p-4">Order ID</th>
              <th className="p-4">Channel</th>
              <th className="p-4">Customer (E.164)</th>
              <th className="p-4">Total & Method</th>
              <th className="p-4">Fraud Shield</th>
              <th className="p-4">Intervention</th>
              <th className="p-4">Carrier AWB</th>
            </tr>
          </thead>
          <tbody className="divide-y divide-slate-800">
            {orders.map((o) => (
              <tr key={o.id} className="hover:bg-slate-800/40">
                <td className="p-4 font-mono font-bold text-white">{o.id}</td>
                <td className="p-4 text-indigo-400 font-semibold">{o.channel}</td>
                <td className="p-4 text-slate-200">{o.customer}</td>
                <td className="p-4">
                  <div className="font-semibold text-white">{o.amount}</div>
                  <div className="text-emerald-400">{o.payment}</div>
                </td>
                <td className="p-4 font-semibold text-emerald-300">{o.score}</td>
                <td className="p-4 font-mono text-amber-300">{o.intervention}</td>
                <td className="p-4 font-mono text-emerald-400 font-bold">
                  {o.awb}
                </td>
              </tr>
            ))}
          </tbody>
        </table>
      </div>
    </main>
  );
}
