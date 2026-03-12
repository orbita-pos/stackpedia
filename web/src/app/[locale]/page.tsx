"use client";

import { useState, useEffect, useRef } from "react";
import { useLocaleRouter, useT } from "@/lib/locale-context";
import { LocaleLink } from "@/components/locale-link";
import { getStats, listTools, listStacks, getTrending } from "@/lib/api";
import type { StatsResponse, ToolDirectoryEntry, StackSummary, StackDetail, TrendingResponse } from "@/lib/types";
import { VERDICT_DOT, VERDICT_COLORS, SCALE_LABELS } from "@/lib/types";
import { StackCard, StackCardSkeleton } from "@/components/stack-card";
import { Footer } from "@/components/footer";

// --- Typing effect ---
function useTypewriter(text: string, speed = 40) {
  const [displayed, setDisplayed] = useState("");
  useEffect(() => {
    let i = 0;
    setDisplayed("");
    const interval = setInterval(() => {
      i++;
      setDisplayed(text.slice(0, i));
      if (i >= text.length) clearInterval(interval);
    }, speed);
    return () => clearInterval(interval);
  }, [text, speed]);
  return displayed;
}

// --- Animated counter ---
function AnimatedCount({ target }: { target: number }) {
  const [count, setCount] = useState(0);
  useEffect(() => {
    if (target === 0) return;
    let current = 0;
    const duration = 1000;
    const steps = 30;
    const increment = target / steps;
    const interval = setInterval(() => {
      current += increment;
      if (current >= target) {
        setCount(target);
        clearInterval(interval);
      } else {
        setCount(Math.floor(current));
      }
    }, duration / steps);
    return () => clearInterval(interval);
  }, [target]);
  return <span className="count-fade text-orange-500">{count}</span>;
}

// --- Hardcoded example stack ---
const EXAMPLE_STACK: StackDetail = {
  id: "example",
  project_name: "Orbita POS",
  description: "Point-of-sale system for restaurants with real-time order management",
  category: "saas",
  url: null,
  scale: "thousands",
  lessons:
    "Should have invested in proper error monitoring from day one. We lost 2 days debugging a production issue that Sentry would have caught in 5 minutes.",
  upvotes: 12,
  creator_id: "",
  creator_nickname: "marcelo",
  created_at: new Date().toISOString(),
  updated_at: null,
  history: [],
  tools: [
    { id: "1", name: "next.js", category: "frontend", why: "Fullstack React with SSR. Nothing else comes close for developer productivity.", cost: "$0", verdict: "love" },
    { id: "2", name: "postgresql", category: "database", why: "Rock solid. Never had a single issue in 2 years.", cost: "$0", verdict: "love" },
    { id: "3", name: "vercel", category: "hosting", why: "Great DX but costs add up fast once you leave hobby tier.", cost: "$20/mo", verdict: "good" },
    { id: "4", name: "stripe", category: "payments", why: "Best payment API ever made. Documentation is perfect.", cost: "2.9%+30c", verdict: "love" },
    { id: "5", name: "tailwindcss", category: "frontend", why: "Can't imagine writing CSS any other way. Ship UI 10x faster.", cost: "$0", verdict: "love" },
  ],
  comments: [],
};

function FeaturedStack({ stack }: { stack: StackDetail }) {
  const t = useT();
  return (
    <div className="border border-neutral-200 dark:border-neutral-800 bg-neutral-50 dark:bg-neutral-900">
      <div className="p-4 sm:p-5">
        <div className="flex items-baseline gap-2 sm:gap-3 flex-wrap mb-2">
          <LocaleLink
            href={stack.id === "example" ? "#" : `/stacks/${stack.id}`}
            className="text-base sm:text-lg font-bold text-neutral-900 dark:text-white hover:text-orange-500"
          >
            {stack.project_name}
          </LocaleLink>
          <span className="text-xs border border-neutral-200 dark:border-neutral-800 px-2 py-0.5 text-neutral-500">
            {stack.category}
          </span>
          <span className="text-xs border border-neutral-200 dark:border-neutral-800 px-2 py-0.5 text-neutral-500">
            {SCALE_LABELS[stack.scale] || stack.scale}
          </span>
          <span className="text-xs text-neutral-500">▲ {stack.upvotes}</span>
        </div>
        <p className="text-sm text-neutral-600 dark:text-neutral-400 mb-4">{stack.description}</p>
        <div className="space-y-0">
          {stack.tools.slice(0, 5).map((tool) => (
            <div
              key={tool.id}
              className="flex flex-col gap-1 py-2 border-t border-neutral-200 dark:border-neutral-800 first:border-t-0 sm:flex-row sm:items-start sm:gap-3"
            >
              <div className="flex items-center gap-2 shrink-0">
                <span className="font-bold text-sm text-neutral-900 dark:text-white">{tool.name}</span>
                <span
                  className={`border px-1.5 py-0 text-xs shrink-0 ${VERDICT_COLORS[tool.verdict]}`}
                >
                  {tool.verdict}
                </span>
                {tool.cost && (
                  <span className="text-xs text-neutral-500 sm:hidden">{tool.cost}</span>
                )}
              </div>
              <span className="text-xs text-neutral-500 flex-1">{tool.why}</span>
              {tool.cost && (
                <span className="text-xs text-neutral-500 shrink-0 hidden sm:block">{tool.cost}</span>
              )}
            </div>
          ))}
        </div>
        {stack.lessons && (
          <div className="mt-3 border-t border-neutral-200 dark:border-neutral-800 pt-3">
            <p className="text-xs text-neutral-500 uppercase tracking-wider mb-1">lessons</p>
            <p className="text-xs text-neutral-600 dark:text-neutral-400">{stack.lessons}</p>
          </div>
        )}
      </div>
      <div className="border-t border-neutral-200 dark:border-neutral-800 px-4 sm:px-5 py-2 text-xs text-neutral-500">
        {t("home.by")} {stack.creator_nickname}
      </div>
    </div>
  );
}

export default function HomePage() {
  const router = useLocaleRouter();
  const t = useT();
  const [stats, setStats] = useState<StatsResponse | null>(null);
  const [tools, setTools] = useState<ToolDirectoryEntry[]>([]);
  const [stacks, setStacks] = useState<StackSummary[]>([]);
  const [trending, setTrending] = useState<TrendingResponse | null>(null);
  const [search, setSearch] = useState("");
  const [loading, setLoading] = useState(true);
  const [featuredStack, setFeaturedStack] = useState<StackDetail | null>(null);

  const subtitle = useTypewriter(t("home.subtitle"), 35);

  useEffect(() => {
    Promise.all([
      getStats().catch(() => ({ total_stacks: 0, total_tools: 0, total_users: 0 })),
      listTools().catch(() => []),
      listStacks({ sort: "top", limit: 6 }).catch(() => []),
      getTrending().catch(() => null),
    ]).then(([s, t, st, tr]) => {
      setStats(s);
      setTools(t.slice(0, 8));
      setStacks(st);
      setTrending(tr);
      setLoading(false);

      // Use top-voted stack as featured, or example if none
      if (st.length > 0) {
        import("@/lib/api").then(({ getStack }) => {
          getStack(st[0].id)
            .then(setFeaturedStack)
            .catch(() => setFeaturedStack(EXAMPLE_STACK));
        });
      } else {
        setFeaturedStack(EXAMPLE_STACK);
      }
    });
  }, []);

  const handleSearch = (e: React.FormEvent) => {
    e.preventDefault();
    if (search.trim()) {
      router.push(`/stacks?search=${encodeURIComponent(search.trim())}`);
    }
  };

  return (
    <div className="mx-auto max-w-3xl px-4">
      {/* Hero */}
      <div className="pt-16 pb-10 text-center sm:pt-24 sm:pb-16">
        <h1 className="text-3xl font-bold text-neutral-900 dark:text-white sm:text-4xl md:text-5xl flex items-center justify-center gap-3">
          <img src="/logo-96x96.png" alt="" width={40} height={40} className="inline-block sm:w-12 sm:h-12" />
          stackpedia
        </h1>
        <p className="mt-4 text-sm text-neutral-500 sm:text-base min-h-[1.5rem] sm:min-h-[1.75rem]">
          {subtitle}
          <span className="inline-block w-2 h-4 ml-0.5 bg-orange-500 animate-pulse align-text-bottom" />
        </p>
        {stats && (
          <div className="mt-4 flex items-center justify-center gap-3 text-sm text-neutral-500 sm:gap-4">
            <span>
              <AnimatedCount target={stats.total_stacks} />{" "}
              <span className="hidden sm:inline">{t("home.stacks")}</span>
              <span className="sm:hidden">{t("home.stacks")}</span>
            </span>
            <span className="text-neutral-400 dark:text-neutral-600">&middot;</span>
            <span>
              <AnimatedCount target={stats.total_tools} />{" "}
              <span className="hidden sm:inline">{t("home.tools")}</span>
              <span className="sm:hidden">{t("home.tools")}</span>
            </span>
            <span className="text-neutral-400 dark:text-neutral-600">&middot;</span>
            <span>
              <AnimatedCount target={stats.total_users} />{" "}
              <span className="hidden sm:inline">{t("home.developers")}</span>
              <span className="sm:hidden">{t("home.developers")}</span>
            </span>
          </div>
        )}

        {/* Search */}
        <form onSubmit={handleSearch} className="mt-6 sm:mt-8 flex justify-center">
          <input
            type="text"
            value={search}
            onChange={(e) => setSearch(e.target.value)}
            placeholder={t("home.searchPlaceholder")}
            className="w-full max-w-md border border-neutral-200 dark:border-neutral-800 bg-neutral-50 dark:bg-neutral-900 px-4 py-2.5 text-sm text-neutral-900 dark:text-white placeholder-neutral-400 dark:placeholder-neutral-600 outline-none focus:border-orange-500"
          />
          <button
            type="submit"
            className="bg-orange-500 px-4 sm:px-5 py-2.5 text-sm font-bold text-black hover:bg-orange-600 dark:hover:bg-orange-400 shrink-0"
          >
            {t("common.search")}
          </button>
        </form>
      </div>

      {/* Featured stack */}
      {featuredStack && (
        <section className="mb-10 sm:mb-12">
          <h2 className="mb-3 sm:mb-4 text-sm font-bold uppercase tracking-wider text-neutral-500">
            {featuredStack.id === "example" ? t("home.exampleStack") : t("home.featuredStack")}
          </h2>
          <FeaturedStack stack={featuredStack} />
        </section>
      )}

      {/* Trending tools */}
      {tools.length > 0 && (
        <section className="mb-10 sm:mb-12">
          <h2 className="mb-3 sm:mb-4 text-sm font-bold uppercase tracking-wider text-neutral-500">
            {t("home.trendingTools")}
          </h2>
          <div className="flex flex-wrap gap-2">
            {tools.map((tool) => (
              <LocaleLink
                key={tool.name}
                href={`/tools/${encodeURIComponent(tool.name)}`}
                className="flex items-center gap-2 border border-neutral-200 dark:border-neutral-800 bg-neutral-50 dark:bg-neutral-900 px-3 py-1.5 text-xs sm:text-sm text-neutral-700 dark:text-neutral-300 hover:border-orange-500/50 hover:text-neutral-900 dark:hover:text-white"
              >
                <span
                  className={`inline-block h-2 w-2 ${VERDICT_DOT[tool.avg_verdict] || "bg-neutral-500"}`}
                />
                {tool.name}
                <span className="text-neutral-500">{tool.stack_count}</span>
              </LocaleLink>
            ))}
          </div>
        </section>
      )}

      {/* Trending this week */}
      {trending && (trending.top_stacks.length > 0 || trending.hot_tools.length > 0 || trending.most_regretted.length > 0) && (
        <section className="mb-10 sm:mb-12">
          <h2 className="mb-3 sm:mb-4 text-sm font-bold uppercase tracking-wider text-neutral-500">
            {t("home.trendingWeek")}
          </h2>
          <div className="grid grid-cols-1 gap-4 sm:grid-cols-2">
            {/* Top stacks by recent votes */}
            {trending.top_stacks.length > 0 && (
              <div className="border border-neutral-200 dark:border-neutral-800 p-4">
                <p className="text-xs text-neutral-500 uppercase tracking-wider mb-3">{t("home.hotStacks")}</p>
                {trending.top_stacks.map((s) => (
                  <LocaleLink
                    key={s.id}
                    href={`/stacks/${s.id}`}
                    className="flex items-center justify-between py-1.5 text-sm text-neutral-700 dark:text-neutral-300 hover:text-neutral-900 dark:hover:text-white"
                  >
                    <span className="truncate">{s.project_name}</span>
                    <span className="text-orange-500 text-xs ml-2 shrink-0">+{s.recent_votes}</span>
                  </LocaleLink>
                ))}
              </div>
            )}

            <div className="space-y-4">
              {/* Hot tools */}
              {trending.hot_tools.length > 0 && (
                <div className="border border-neutral-200 dark:border-neutral-800 p-4">
                  <p className="text-xs text-neutral-500 uppercase tracking-wider mb-3">{t("home.hotTools")}</p>
                  <div className="flex flex-wrap gap-2">
                    {trending.hot_tools.map((t) => (
                      <LocaleLink
                        key={t.name}
                        href={`/tools/${encodeURIComponent(t.name)}`}
                        className="border border-neutral-200 dark:border-neutral-800 bg-neutral-50 dark:bg-neutral-900 px-2 py-1 text-xs text-neutral-700 dark:text-neutral-300 hover:border-orange-500/50 hover:text-neutral-900 dark:hover:text-white"
                      >
                        {t.name} <span className="text-neutral-500">{t.count}</span>
                      </LocaleLink>
                    ))}
                  </div>
                </div>
              )}

              {/* Most regretted */}
              {trending.most_regretted.length > 0 && (
                <div className="border border-neutral-200 dark:border-neutral-800 p-4">
                  <p className="text-xs text-neutral-500 uppercase tracking-wider mb-3">{t("home.mostRegretted")}</p>
                  <div className="flex flex-wrap gap-2">
                    {trending.most_regretted.map((t) => (
                      <LocaleLink
                        key={t.name}
                        href={`/tools/${encodeURIComponent(t.name)}`}
                        className="border border-red-400/20 bg-red-400/5 px-2 py-1 text-xs text-red-400 hover:border-red-400/40"
                      >
                        {t.name} <span className="text-red-400/50">{t.count}</span>
                      </LocaleLink>
                    ))}
                  </div>
                </div>
              )}
            </div>
          </div>
        </section>
      )}

      {/* Latest stacks */}
      <section className="mb-16">
        <div className="mb-3 sm:mb-4 flex items-center justify-between">
          <h2 className="text-sm font-bold uppercase tracking-wider text-neutral-500">
            {t("home.latestStacks")}
          </h2>
          <LocaleLink href="/stacks" className="text-sm text-orange-500 hover:text-orange-500">
            {t("home.viewAll")}
          </LocaleLink>
        </div>
        {loading
          ? Array.from({ length: 3 }).map((_, i) => <StackCardSkeleton key={i} />)
          : stacks.length > 0
            ? stacks.map((stack) => <StackCard key={stack.id} stack={stack} />)
            : (
                <p className="py-12 text-center text-sm text-neutral-500">
                  {t("home.noStacks")}{" "}
                  <LocaleLink href="/new" className="text-orange-500 hover:underline">
                    {t("home.shareStack")}
                  </LocaleLink>
                </p>
              )}
      </section>

      <Footer />
    </div>
  );
}
