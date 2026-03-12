import type { Metadata } from "next";
import { StackDetailClient } from "./stack-detail-client";
import { dictionaries, defaultLocale, type Locale } from "@/lib/i18n";

const API = process.env.NEXT_PUBLIC_API_URL || "http://localhost:3001";

async function getStackData(id: string) {
  try {
    const res = await fetch(`${API}/api/stacks/${id}`, { cache: "no-store" });
    if (!res.ok) return null;
    return res.json();
  } catch {
    return null;
  }
}

export async function generateMetadata({ params }: { params: Promise<{ id: string; locale: string }> }): Promise<Metadata> {
  const { id, locale } = await params;
  const dict = dictionaries[(locale as Locale)] || dictionaries[defaultLocale];
  const stack = await getStackData(id);
  if (!stack) {
    return { title: `${dict["common.stackNotFound"]} — Stackpedia` };
  }
  const title = `${stack.project_name} stack — Stackpedia`;
  const description = stack.description;
  return {
    title,
    description,
    openGraph: { title, description },
  };
}

export default function StackDetailPage({ params }: { params: Promise<{ id: string }> }) {
  return <StackDetailClient params={params} />;
}
