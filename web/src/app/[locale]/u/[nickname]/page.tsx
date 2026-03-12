import type { Metadata } from "next";
import { UserProfileClient } from "./user-profile-client";
import { dictionaries, defaultLocale, type Locale } from "@/lib/i18n";

const API = process.env.NEXT_PUBLIC_API_URL || "http://localhost:3001";

async function getProfile(nickname: string) {
  try {
    const res = await fetch(`${API}/api/users/${encodeURIComponent(nickname)}`, { cache: "no-store" });
    if (!res.ok) return null;
    return res.json();
  } catch {
    return null;
  }
}

export async function generateMetadata({ params }: { params: Promise<{ nickname: string; locale: string }> }): Promise<Metadata> {
  const { nickname, locale } = await params;
  const dict = dictionaries[(locale as Locale) || defaultLocale] ?? dictionaries[defaultLocale];
  const profile = await getProfile(nickname);
  if (!profile) {
    return { title: `${dict["common.userNotFound"]} — Stackpedia` };
  }
  const title = `${profile.nickname} — ${profile.stack_count} stack${profile.stack_count !== 1 ? "s" : ""} — Stackpedia`;
  return {
    title,
    description: `${profile.nickname} has shared ${profile.stack_count} tech stack${profile.stack_count !== 1 ? "s" : ""} on Stackpedia.`,
    openGraph: { title },
  };
}

export default function UserProfilePage({ params }: { params: Promise<{ nickname: string }> }) {
  return <UserProfileClient params={params} />;
}
