"use client";

import { Suspense, useState, useEffect, useRef } from "react";
import { useSearchParams } from "next/navigation";
import { LocaleLink } from "@/components/locale-link";
import { useT, useLocaleRouter } from "@/lib/locale-context";
import { listTools } from "@/lib/api";
import type { ToolDirectoryEntry } from "@/lib/types";
import { VERDICT_DOT, TOOL_CATEGORIES } from "@/lib/types";
import { Footer } from "@/components/footer";

export default function ToolsDirectoryPage() {
  return (
    <Suspense
      fallback={
        <div className="mx-auto max-w-3xl px-4 pt-8">
          <div className="h-8 w-48 bg-neutral-200 dark:bg-neutral-800 mb-6" />
          <div className="h-10 w-full bg-neutral-200 dark:bg-neutral-800 mb-8" />
          <div className="animate-pulse space-y-8">
            {Array.from({ length: 4 }).map((_, i) => (
              <div key={i}>
                <div className="h-4 w-24 bg-neutral-200 dark:bg-neutral-800 mb-3" />
                <div className="grid grid-cols-1 gap-px sm:grid-cols-2">
                  {Array.from({ length: 3 }).map((_, j) => (
                    <div key={j} className="h-10 bg-neutral-200 dark:bg-neutral-800" />
                  ))}
                </div>
              </div>
            ))}
          </div>
        </div>
      }
    >
      <ToolsDirectoryContent />
    </Suspense>
  );
}

function ToolsDirectoryContent() {
  const searchParams = useSearchParams();
  const router = useLocaleRouter();
  const t = useT();
  const [tools, setTools] = useState<ToolDirectoryEntry[]>([]);
  const [loading, setLoading] = useState(true);
  const [search, setSearch] = useState(searchParams.get("q") || "");
  const [searchInput, setSearchInput] = useState(searchParams.get("q") || "");
  const debounceRef = useRef<ReturnType<typeof setTimeout>>(undefined);

  useEffect(() => {
    listTools()
      .then(setTools)
      .catch(() => {})
      .finally(() => setLoading(false));
  }, []);

  const onSearchInput = (value: string) => {
    setSearchInput(value);
    clearTimeout(debounceRef.current);
    debounceRef.current = setTimeout(() => {
      setSearch(value);
      const params = new URLSearchParams();
      if (value) params.set("q", value);
      router.replace(`/tools${params.toString() ? `?${params.toString()}` : ""}`, { scroll: false });
    }, 300);
  };

  const filtered = search
    ? tools.filter((tl) => tl.name.toLowerCase().includes(search.toLowerCase()))
    : tools;

  const grouped = TOOL_CATEGORIES.reduce(
    (acc, cat) => {
      const items = filtered.filter((tl) => tl.category === cat);
      if (items.length > 0) acc[cat] = items;
      return acc;
    },
    {} as Record<string, ToolDirectoryEntry[]>
  );

  return (
    <div className="mx-auto max-w-3xl px-4 pt-8 pb-16">
      <h1 className="mb-2 text-2xl font-bold text-neutral-900 dark:text-white">{t("tools.dir.title")}</h1>
      <p className="mb-6 text-sm text-neutral-500">
        {t("tools.dir.subtitle")}
      </p>

      <input
        type="text"
        value={searchInput}
        onChange={(e) => onSearchInput(e.target.value)}
        placeholder={t("tools.dir.filterPlaceholder")}
        className="mb-8 w-full border border-neutral-200 dark:border-neutral-800 bg-neutral-50 dark:bg-neutral-900 px-4 py-2 text-sm text-neutral-900 dark:text-white placeholder-neutral-400 dark:placeholder-neutral-600 outline-none focus:border-orange-500"
      />

      {loading ? (
        <div className="animate-pulse space-y-8">
          {Array.from({ length: 4 }).map((_, i) => (
            <div key={i}>
              <div className="h-4 w-24 bg-neutral-200 dark:bg-neutral-800 mb-3" />
              <div className="grid grid-cols-1 gap-px sm:grid-cols-2">
                {Array.from({ length: 3 }).map((_, j) => (
                  <div key={j} className="h-10 bg-neutral-200 dark:bg-neutral-800" />
                ))}
              </div>
            </div>
          ))}
        </div>
      ) : Object.keys(grouped).length === 0 ? (
        <p className="py-16 text-center text-neutral-500">
          {search
            ? `${t("tools.dir.noMatch")} "${search}".`
            : t("tools.dir.noTools")}
        </p>
      ) : (
        <div className="space-y-8">
          {Object.entries(grouped).map(([category, items]) => (
            <section key={category}>
              <h2 className="mb-3 text-xs font-bold uppercase tracking-wider text-neutral-500">
                {category}
              </h2>
              <div className="grid grid-cols-1 gap-px border border-neutral-200 dark:border-neutral-800 bg-neutral-200 dark:bg-neutral-800 sm:grid-cols-2">
                {items.map((tool) => (
                  <LocaleLink
                    key={tool.name}
                    href={`/tools/${encodeURIComponent(tool.name)}`}
                    className="flex items-center justify-between bg-white dark:bg-[#0a0a0a] p-3 hover:bg-neutral-50 dark:hover:bg-neutral-900"
                  >
                    <div className="flex items-center gap-2">
                      <span
                        className={`inline-block h-2 w-2 ${VERDICT_DOT[tool.avg_verdict] || "bg-neutral-500"}`}
                      />
                      <span className="text-sm text-neutral-900 dark:text-white">{tool.name}</span>
                    </div>
                    <span className="text-xs text-neutral-500">
                      {tool.stack_count} stack{tool.stack_count !== 1 ? "s" : ""}
                    </span>
                  </LocaleLink>
                ))}
              </div>
            </section>
          ))}
        </div>
      )}

      <div className="mt-16">
        <Footer />
      </div>
    </div>
  );
}
