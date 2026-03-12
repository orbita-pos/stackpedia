"use client";

import { useState, useEffect } from "react";
import { useT } from "@/lib/locale-context";
import { useAuth } from "@/lib/auth-context";
import { getMyBookmarks } from "@/lib/api";
import type { StackSummary } from "@/lib/types";
import { StackCard, StackCardSkeleton } from "@/components/stack-card";
import { Footer } from "@/components/footer";

export default function BookmarksPage() {
  const t = useT();
  const { user, openAuthModal } = useAuth();
  const [stacks, setStacks] = useState<StackSummary[]>([]);
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    if (!user) {
      setLoading(false);
      return;
    }
    getMyBookmarks()
      .then(setStacks)
      .catch(() => {})
      .finally(() => setLoading(false));
  }, [user]);

  return (
    <div className="mx-auto max-w-3xl px-4 pt-8 pb-16">
      <h1 className="mb-6 text-xl sm:text-2xl font-bold text-neutral-900 dark:text-white">
        {t("bookmarks.title")}
      </h1>

      {!user ? (
        <div className="py-16 text-center">
          <p className="text-neutral-500 mb-4">{t("bookmarks.joinFirst")}</p>
          <button
            onClick={openAuthModal}
            className="bg-orange-500 px-4 py-2 text-sm font-bold text-white hover:bg-orange-600"
          >
            {t("nav.join")}
          </button>
        </div>
      ) : loading ? (
        Array.from({ length: 3 }).map((_, i) => <StackCardSkeleton key={i} />)
      ) : stacks.length === 0 ? (
        <div className="py-16 text-center">
          <p className="text-neutral-500">{t("bookmarks.empty")}</p>
        </div>
      ) : (
        stacks.map((stack) => <StackCard key={stack.id} stack={stack} />)
      )}

      <div className="mt-16">
        <Footer />
      </div>
    </div>
  );
}
