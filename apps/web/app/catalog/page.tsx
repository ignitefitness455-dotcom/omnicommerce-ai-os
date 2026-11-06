import React from "react";

export default function CatalogMarginManagerPage() {
  const products = [
    {
      sku: "OMNI-GCC-ABAYA-902",
      titleEn: "Oud Royal Velvet Abaya — Limited Edition",
      titleAr: "عباية مخمل ملكي فاخرة",
      price: "420.00 AED",
      cogs: "240.00 AED",
      minMargin: "28.0%",
      maxAiDiscount: "12.0%",
      stock: 18,
    },
    {
      sku: "OMNI-KSA-OUD-501",
      titleEn: "Royal Cambodian Oud Elixir 100ml",
      titleAr: "دهن عود كمبودي ملكي",
      price: "540.00 SAR",
      cogs: "290.00 SAR",
      minMargin: "30.0%",
      maxAiDiscount: "10.0%",
      stock: 32,
    },
    {
      sku: "OMNI-SEA-SILK-310",
      titleEn: "Kuala Lumpur Silk Kurung Set",
      titleAr: "—",
      price: "185.00 MYR",
      cogs: "98.00 MYR",
      minMargin: "26.0%",
      maxAiDiscount: "12.0%",
      stock: 25,
    },
  ];

  return (
    <main className="max-w-7xl mx-auto p-6 md:p-10 space-y-6">
      <div>
        <h1 className="text-2xl font-bold text-white">
          🏷️ Product Catalog & Secret AI Margin Guardrails
        </h1>
        <p className="text-sm text-slate-400 mt-1">
          Configure secret Cost of Goods Sold (COGS) and minimum net margin floors so the AI Haggling Engine never undercuts your profit
        </p>
      </div>

      <div className="grid grid-cols-1 md:grid-cols-3 gap-5">
        {products.map((p) => (
          <div
            key={p.sku}
            className="p-5 rounded-2xl bg-slate-900 border border-slate-800 space-y-3 text-xs"
          >
            <div className="flex items-center justify-between">
              <span className="font-mono text-indigo-400 font-bold">{p.sku}</span>
              <span className="px-2 py-0.5 rounded bg-emerald-500/10 text-emerald-400 border border-emerald-500/30">
                Pinecone Indexed
              </span>
            </div>
            <div className="text-base font-bold text-white">{p.titleEn}</div>
            <div className="text-slate-400" dir="rtl">
              {p.titleAr}
            </div>
            <div className="grid grid-cols-2 gap-2 pt-2 border-t border-slate-800">
              <div>
                <span className="text-slate-400">Retail Price:</span>
                <div className="font-bold text-white">{p.price}</div>
              </div>
              <div>
                <span className="text-slate-400">Secret COGS:</span>
                <div className="font-bold text-amber-400">{p.cogs}</div>
              </div>
              <div>
                <span className="text-slate-400">Min Margin Floor:</span>
                <div className="font-bold text-emerald-400">{p.minMargin}</div>
              </div>
              <div>
                <span className="text-slate-400">Max AI Discount:</span>
                <div className="font-bold text-fuchsia-400">
                  {p.maxAiDiscount}
                </div>
              </div>
            </div>
          </div>
        ))}
      </div>
    </main>
  );
}
