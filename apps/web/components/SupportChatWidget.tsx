"use client";

import React, { useState } from "react";
import { MessageSquare, Send, X } from "lucide-react";

interface ChatMessage {
  sender: "ai" | "user";
  text: string;
}

const PRESET_ANSWERS: Record<string, string> = {
  pricing:
    "We offer 3 transparent SaaS tiers: Growth ($79/mo), Scale ($249/mo for high-velocity GCC/SEA brands), and Enterprise ($699/mo). Annual billing saves 20%!",
  shopify:
    "Connect Shopify or Salla in under 2 minutes! Go to Merchant Onboarding, authorize OAuth, and webhooks auto-verify every COD order via HMAC-SHA256.",
  waba:
    "OmniCommerce AI OS uses Meta's official WhatsApp Cloud API (v20.0) and native WhatsApp Flows for 1-tap address pin confirmation and 5% COD-to-Prepaid nudges.",
  gdpr:
    "100% GDPR & Saudi PDPL compliant! Customer phone numbers are normalized to E.164 and hashed with Salted HMAC-SHA256. Raw PII is never shared between merchants.",
};

export function SupportChatWidget() {
  const [open, setOpen] = useState(false);
  const [input, setInput] = useState("");
  const [messages, setMessages] = useState<ChatMessage[]>([
    {
      sender: "ai",
      text: "Marhaba! 👋 I'm the OmniCommerce Enterprise AI Concierge. Ask me about Pricing, Shopify/Salla setup, Meta WABA approval, or GDPR/Saudi PDPL compliance.",
    },
  ]);

  function handleSend(customPrompt?: string) {
    const query = (customPrompt ?? input).trim();
    if (!query) return;

    const lower = query.toLowerCase();
    let reply =
      "Great question! Our solutions engineering team in Dubai & Singapore can enable that directly on your Scale or Enterprise plan. Would you like to test the ROI Calculator on the Landing Page?";

    if (lower.includes("price") || lower.includes("cost") || lower.includes("plan")) {
      reply = PRESET_ANSWERS.pricing;
    } else if (lower.includes("shopify") || lower.includes("salla") || lower.includes("zid")) {
      reply = PRESET_ANSWERS.shopify;
    } else if (lower.includes("whatsapp") || lower.includes("meta") || lower.includes("waba")) {
      reply = PRESET_ANSWERS.waba;
    } else if (lower.includes("gdpr") || lower.includes("pdpl") || lower.includes("privacy")) {
      reply = PRESET_ANSWERS.gdpr;
    }

    setMessages((prev) => [
      ...prev,
      { sender: "user", text: query },
      { sender: "ai", text: reply },
    ]);
    if (!customPrompt) setInput("");
  }

  return (
    <div className="fixed bottom-5 right-5 z-50">
      {open ? (
        <div className="w-80 sm:w-96 rounded-2xl bg-slate-900 border border-slate-700 shadow-2xl overflow-hidden flex flex-col">
          <div className="bg-emerald-600 px-4 py-3 flex items-center justify-between text-slate-950">
            <div>
              <div className="font-bold text-sm">OmniCommerce 24/7 AI Support</div>
              <div className="text-[11px] font-medium opacity-90">
                GCC • SEA • Global Enterprise Concierge
              </div>
            </div>
            <button
              onClick={() => setOpen(false)}
              className="p-1 rounded-lg hover:bg-emerald-500/40"
              aria-label="Close support chat"
            >
              <X className="w-4 h-4" />
            </button>
          </div>

          <div className="p-3 space-y-2.5 max-h-72 overflow-y-auto bg-slate-950 text-xs">
            {messages.map((m, idx) => (
              <div
                key={idx}
                className={`p-2.5 rounded-xl leading-relaxed ${
                  m.sender === "ai"
                    ? "bg-slate-900 border border-slate-800 text-slate-200 mr-6"
                    : "bg-indigo-600 text-white ml-6"
                }`}
              >
                {m.text}
              </div>
            ))}
          </div>

          <div className="px-3 py-2 bg-slate-900 border-t border-slate-800 flex flex-wrap gap-1.5">
            <button
              onClick={() => handleSend("Pricing plans?")}
              className="text-[10px] px-2 py-1 rounded bg-slate-800 hover:bg-slate-700 text-emerald-300"
            >
              Pricing
            </button>
            <button
              onClick={() => handleSend("How does Shopify & Salla work?")}
              className="text-[10px] px-2 py-1 rounded bg-slate-800 hover:bg-slate-700 text-indigo-300"
            >
              Shopify/Salla
            </button>
            <button
              onClick={() => handleSend("GDPR & Saudi PDPL privacy?")}
              className="text-[10px] px-2 py-1 rounded bg-slate-800 hover:bg-slate-700 text-amber-300"
            >
              GDPR/PDPL
            </button>
          </div>

          <form
            onSubmit={(e) => {
              e.preventDefault();
              handleSend();
            }}
            className="p-2.5 bg-slate-900 border-t border-slate-800 flex gap-2"
          >
            <input
              value={input}
              onChange={(e) => setInput(e.target.value)}
              placeholder="Ask anything..."
              className="flex-1 px-3 py-1.5 rounded-lg bg-slate-950 border border-slate-700 text-xs text-white focus:outline-none focus:border-emerald-500"
            />
            <button
              type="submit"
              className="px-3 py-1.5 rounded-lg bg-emerald-500 hover:bg-emerald-400 text-slate-950 font-bold"
              aria-label="Send message"
            >
              <Send className="w-3.5 h-3.5" />
            </button>
          </form>
        </div>
      ) : (
        <button
          onClick={() => setOpen(true)}
          className="flex items-center gap-2 px-4 py-3 rounded-full bg-emerald-500 hover:bg-emerald-400 text-slate-950 font-bold text-xs shadow-xl transition"
        >
          <MessageSquare className="w-4 h-4" />
          <span>24/7 Live AI Support</span>
        </button>
      )}
    </div>
  );
}
