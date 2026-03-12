import type { Metadata } from "next";
import { ToolDetailClient } from "./tool-detail-client";
import { dictionaries, defaultLocale, type Locale } from "@/lib/i18n";

const API = process.env.NEXT_PUBLIC_API_URL || "http://localhost:3001";

async function getToolEntries(name: string) {
  try {
    const res = await fetch(`${API}/api/tools/${encodeURIComponent(name)}`, { cache: "no-store" });
    if (!res.ok) return [];
    return res.json();
  } catch {
    return [];
  }
}

export async function generateMetadata({ params }: { params: Promise<{ name: string; locale: string }> }): Promise<Metadata> {
  const { name, locale } = await params;
  const dict = dictionaries[(locale as Locale) || defaultLocale] ?? dictionaries[defaultLocale];
  const toolName = decodeURIComponent(name);
  const entries = await getToolEntries(toolName);
  const count = entries.length;
  const title = `${toolName} — used in ${count} project${count !== 1 ? "s" : ""} — Stackpedia`;
  const description = `${dict["tool.whatDevsSay"]} ${toolName} — ${count} project${count !== 1 ? "s" : ""}`;
  return {
    title,
    description,
    openGraph: { title, description },
  };
}

export default function ToolPage({ params }: { params: Promise<{ name: string }> }) {
  return <ToolDetailClient params={params} />;
}
