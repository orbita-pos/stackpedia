"use client";

import { useState, useEffect, use } from "react";
import { LocaleLink } from "@/components/locale-link";
import { useT } from "@/lib/locale-context";
import { getUserProfile, updateProfile } from "@/lib/api";
import type { UserProfileResponse } from "@/lib/types";
import { relativeTime } from "@/lib/utils";
import { StackCard, StackCardSkeleton } from "@/components/stack-card";
import { PixelAvatar } from "@/components/pixel-avatar";
import { Footer } from "@/components/footer";
import { useAuth } from "@/lib/auth-context";

export function UserProfileClient({ params }: { params: Promise<{ nickname: string }> }) {
  const { nickname } = use(params);
  const t = useT();
  const { user } = useAuth();
  const [profile, setProfile] = useState<UserProfileResponse | null>(null);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState("");

  // Sponsor edit state
  const [editingSponsor, setEditingSponsor] = useState(false);
  const [sponsorInput, setSponsorInput] = useState("");
  const [savingSponsor, setSavingSponsor] = useState(false);
  const [sponsorSaved, setSponsorSaved] = useState(false);

  const isOwnProfile = user && profile && user.nickname === profile.nickname;

  useEffect(() => {
    getUserProfile(nickname)
      .then(setProfile)
      .catch((e) => setError(e.message))
      .finally(() => setLoading(false));
  }, [nickname]);

  const handleSaveSponsor = async () => {
    setSavingSponsor(true);
    try {
      await updateProfile({ sponsor_url: sponsorInput || "" });
      setProfile((prev) =>
        prev ? { ...prev, sponsor_url: sponsorInput || null } : prev
      );
      setEditingSponsor(false);
      setSponsorSaved(true);
      setTimeout(() => setSponsorSaved(false), 2000);
    } catch {
      // ignore
    } finally {
      setSavingSponsor(false);
    }
  };

  if (loading) {
    return (
      <div className="mx-auto max-w-3xl px-4 pt-8">
        <div className="animate-pulse">
          <div className="flex items-center gap-4 mb-6">
            <div className="w-16 h-16 bg-neutral-200 dark:bg-neutral-800" />
            <div>
              <div className="h-6 w-32 bg-neutral-200 dark:bg-neutral-800 mb-2" />
              <div className="h-4 w-48 bg-neutral-200 dark:bg-neutral-800" />
            </div>
          </div>
          {Array.from({ length: 3 }).map((_, i) => (
            <StackCardSkeleton key={i} />
          ))}
        </div>
      </div>
    );
  }

  if (error || !profile) {
    return (
      <div className="mx-auto max-w-3xl px-4 pt-8 text-center">
        <p className="text-red-400">{error || t("common.userNotFound")}</p>
        <LocaleLink href="/stacks" className="mt-4 inline-block text-sm text-orange-500 hover:underline">
          {t("common.back")}
        </LocaleLink>
      </div>
    );
  }

  return (
    <div className="mx-auto max-w-3xl px-4 pt-8 pb-16">
      {/* Profile header */}
      <div className="flex items-center gap-4 mb-8">
        <PixelAvatar nickname={profile.nickname} size={64} />
        <div>
          <h1 className="text-xl font-bold text-neutral-900 dark:text-white sm:text-2xl">
            {profile.nickname}
          </h1>
          <p className="text-sm text-neutral-500">
            {t("profile.joined")} {relativeTime(profile.created_at)} &middot;{" "}
            {profile.stack_count} stack{profile.stack_count !== 1 ? "s" : ""}
          </p>

          {/* Sponsor link display */}
          {profile.sponsor_url && !editingSponsor && (
            <a
              href={profile.sponsor_url}
              target="_blank"
              rel="noopener noreferrer"
              className="mt-1 inline-flex items-center gap-1.5 text-sm text-pink-500 hover:text-pink-400 transition-colors"
            >
              <span>♥</span>
              <span>{t("profile.sponsor")}</span>
            </a>
          )}

          {/* Sponsor saved confirmation */}
          {sponsorSaved && !editingSponsor && (
            <span className="ml-2 text-xs text-green-500">{t("profile.sponsorSaved")}</span>
          )}
        </div>
      </div>

      {/* Own profile: sponsor edit */}
      {isOwnProfile && (
        <div className="mb-8">
          {editingSponsor ? (
            <div className="border border-neutral-200 dark:border-neutral-800 p-4">
              <label className="block text-xs font-bold uppercase tracking-wider text-neutral-500 mb-2">
                {t("profile.sponsorEdit")}
              </label>
              <input
                type="url"
                value={sponsorInput}
                onChange={(e) => setSponsorInput(e.target.value)}
                placeholder={t("profile.sponsorPlaceholder")}
                className="w-full border border-neutral-300 dark:border-neutral-700 bg-white dark:bg-neutral-900 px-3 py-2 text-sm outline-none focus:border-orange-500 mb-1"
              />
              <p className="text-xs text-neutral-500 mb-3">
                {t("profile.sponsorHint")}
              </p>
              <div className="flex gap-2">
                <button
                  onClick={handleSaveSponsor}
                  disabled={savingSponsor}
                  className="border border-orange-500 text-orange-500 px-4 py-1 text-sm hover:bg-orange-500/10 transition-colors disabled:opacity-50"
                >
                  {savingSponsor ? "..." : t("profile.sponsorSave")}
                </button>
                <button
                  onClick={() => setEditingSponsor(false)}
                  className="border border-neutral-300 dark:border-neutral-700 text-neutral-500 px-4 py-1 text-sm hover:bg-neutral-100 dark:hover:bg-neutral-800 transition-colors"
                >
                  {t("common.cancel")}
                </button>
              </div>
            </div>
          ) : (
            <button
              onClick={() => {
                setSponsorInput(profile.sponsor_url || "");
                setEditingSponsor(true);
              }}
              className="text-sm text-neutral-500 hover:text-orange-500 transition-colors border border-neutral-300 dark:border-neutral-700 px-3 py-1 hover:border-orange-500"
            >
              {profile.sponsor_url ? `♥ ${t("profile.sponsorEdit")}` : `+ ${t("profile.sponsorEdit")}`}
            </button>
          )}
        </div>
      )}

      {/* Stacks */}
      <h2 className="mb-4 text-sm font-bold uppercase tracking-wider text-neutral-500">
        {t("profile.stacks")} ({profile.stacks.length})
      </h2>
      {profile.stacks.length === 0 ? (
        <p className="py-12 text-center text-sm text-neutral-500">
          {t("profile.noStacks")}
        </p>
      ) : (
        profile.stacks.map((stack) => (
          <StackCard key={stack.id} stack={stack} />
        ))
      )}

      <div className="mt-16">
        <Footer />
      </div>
    </div>
  );
}
