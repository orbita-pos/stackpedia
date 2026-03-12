"use client";

import { Suspense, useState, useEffect, useCallback, useRef } from "react";
import { useSearchParams } from "next/navigation";
import { LocaleLink } from "@/components/locale-link";
import { useT, useLocaleRouter } from "@/lib/locale-context";
import { listStacks, listTools } from "@/lib/api";
import type { StackSummary, ToolDirectoryEntry } from "@/lib/types";
import { CATEGORIES, SCALES, SCALE_LABELS } from "@/lib/types";
import { StackCard, StackCardSkeleton } from "@/components/stack-card";
import { Footer } from "@/components/footer";

export default function BrowseStacksPage() {
  return (
    <Suspense
      fallback={
        <div className="mx-auto max-w-3xl px-4 pt-8">
          <div className="h-8 w-48 bg-neutral-200 dark:bg-neutral-800 mb-6" />
          <div className="h-10 w-full bg-neutral-200 dark:bg-neutral-800 mb-6" />
          {Array.from({ length: 5 }).map((_, i) => (
            <StackCardSkeleton key={i} />
          ))}
        </div>
      }
    >
      <BrowseStacksContent />
    </Suspense>
  );
}

function BrowseStacksContent() {
  const searchParams = useSearchParams();
  const router = useLocaleRouter();
  const t = useT();

  const [stacks, setStacks] = useState<StackSummary[]>([]);
  const [loading, setLoading] = useState(true);
  const [page, setPage] = useState(1);
  const [hasMore, setHasMore] = useState(true);
  const [loadingMore, setLoadingMore] = useState(false);

  const [sort, setSort] = useState(searchParams.get("sort") || "new");
  const [category, setCategory] = useState(searchParams.get("category") || "all");
  const [scale, setScale] = useState(searchParams.get("scale") || "all");
  const [search, setSearch] = useState(searchParams.get("search") || "");
  const [searchInput, setSearchInput] = useState(searchParams.get("search") || "");
  const [selectedTools, setSelectedTools] = useState<string[]>(
    searchParams.get("tool")?.split(",").filter(Boolean) || []
  );
  const [allTools, setAllTools] = useState<ToolDirectoryEntry[]>([]);
  const debounceRef = useRef<ReturnType<typeof setTimeout>>(undefined);
  const sentinelRef = useRef<HTMLDivElement>(null);

  // Fetch tools for autocomplete
  useEffect(() => {
    listTools().then(setAllTools).catch(() => {});
  }, []);

  const fetchStacks = useCallback(
    async (p: number, append = false) => {
      if (append) setLoadingMore(true);
      else setLoading(true);

      try {
        const data = await listStacks({
          page: p,
          limit: 20,
          sort,
          category: category !== "all" ? category : undefined,
          scale: scale !== "all" ? scale : undefined,
          search: search || undefined,
          tool: selectedTools.length > 0 ? selectedTools.join(",") : undefined,
        });
        if (append) {
          setStacks((prev) => [...prev, ...data]);
        } else {
          setStacks(data);
        }
        setHasMore(data.length === 20);
      } catch {
        if (!append) setStacks([]);
      } finally {
        setLoading(false);
        setLoadingMore(false);
      }
    },
    [sort, category, scale, search, selectedTools]
  );

  useEffect(() => {
    setPage(1);
    fetchStacks(1);
  }, [fetchStacks]);

  // Sync URL params
  const updateUrl = useCallback(
    (overrides: Record<string, string | undefined>) => {
      const params = new URLSearchParams();
      const values: Record<string, string | undefined> = {
        sort,
        category: category !== "all" ? category : undefined,
        scale: scale !== "all" ? scale : undefined,
        search: search || undefined,
        tool: selectedTools.length > 0 ? selectedTools.join(",") : undefined,
        ...overrides,
      };
      Object.entries(values).forEach(([k, v]) => {
        if (v && v !== "new") params.set(k, v);
      });
      const qs = params.toString();
      router.replace(`/stacks${qs ? `?${qs}` : ""}`, { scroll: false });
    },
    [sort, category, scale, search, selectedTools, router]
  );

  // Debounced search
  const onSearchInput = (value: string) => {
    setSearchInput(value);
    clearTimeout(debounceRef.current);
    debounceRef.current = setTimeout(() => {
      setSearch(value);
      updateUrl({ search: value || undefined });
    }, 300);
  };

  const loadMore = () => {
    const next = page + 1;
    setPage(next);
    fetchStacks(next, true);
  };

  // Infinite scroll with IntersectionObserver
  useEffect(() => {
    const el = sentinelRef.current;
    if (!el) return;
    const observer = new IntersectionObserver(
      (entries) => {
        if (entries[0].isIntersecting && hasMore && !loadingMore) {
          loadMore();
        }
      },
      { rootMargin: "200px" }
    );
    observer.observe(el);
    return () => observer.disconnect();
  });

  const updateFilter = (key: string, value: string) => {
    if (key === "sort") setSort(value);
    if (key === "category") setCategory(value);
    if (key === "scale") setScale(value);
    updateUrl({ [key]: value === "all" || value === "new" ? undefined : value });
  };

  const addTool = (toolName: string) => {
    if (!toolName || selectedTools.includes(toolName)) return;
    const next = [...selectedTools, toolName];
    setSelectedTools(next);
    updateUrl({ tool: next.join(",") });
  };

  const removeTool = (toolName: string) => {
    const next = selectedTools.filter((tl) => tl !== toolName);
    setSelectedTools(next);
    updateUrl({ tool: next.length > 0 ? next.join(",") : undefined });
  };

  // Tools not yet selected, for the dropdown
  const availableTools = allTools.filter((tl) => !selectedTools.includes(tl.name));

  const sortKeys = ["new", "top"] as const;
  const sortLabels: Record<string, string> = {
    new: t("stacks.new"),
    top: t("stacks.top"),
  };

  return (
    <div className="mx-auto max-w-3xl px-4 pt-8 pb-16">
      <h1 className="mb-4 sm:mb-6 text-xl sm:text-2xl font-bold text-neutral-900 dark:text-white">{t("stacks.title")}</h1>

      {/* Filters */}
      <div className="mb-6 space-y-3 sm:space-y-0 sm:flex sm:flex-wrap sm:items-center sm:gap-4">
        {/* Row 1 on mobile: sort + selects */}
        <div className="flex items-center gap-2 sm:gap-4 flex-wrap">
          {/* Sort tabs */}
          <div className="flex border border-neutral-200 dark:border-neutral-800">
            {sortKeys.map((s) => (
              <button
                key={s}
                onClick={() => updateFilter("sort", s)}
                className={`px-3 py-1.5 text-sm ${
                  sort === s
                    ? "bg-neutral-50 dark:bg-neutral-900 text-orange-500 border-b-2 border-orange-500"
                    : "text-neutral-500 hover:text-neutral-900 dark:hover:text-white"
                }`}
              >
                {sortLabels[s]}
              </button>
            ))}
          </div>

          {/* Category */}
          <select
            value={category}
            onChange={(e) => updateFilter("category", e.target.value)}
            className="border border-neutral-200 dark:border-neutral-800 bg-neutral-50 dark:bg-neutral-900 px-2 sm:px-3 py-1.5 text-sm text-neutral-700 dark:text-neutral-300 outline-none flex-1 sm:flex-none min-w-0"
          >
            <option value="all">{t("stacks.allCategories")}</option>
            {CATEGORIES.map((c) => (
              <option key={c} value={c}>
                {c}
              </option>
            ))}
          </select>

          {/* Scale */}
          <select
            value={scale}
            onChange={(e) => updateFilter("scale", e.target.value)}
            className="border border-neutral-200 dark:border-neutral-800 bg-neutral-50 dark:bg-neutral-900 px-2 sm:px-3 py-1.5 text-sm text-neutral-700 dark:text-neutral-300 outline-none flex-1 sm:flex-none min-w-0"
          >
            <option value="all">{t("stacks.allScales")}</option>
            {SCALES.map((s) => (
              <option key={s} value={s}>
                {SCALE_LABELS[s]}
              </option>
            ))}
          </select>
        </div>

        {/* Search -- full width on mobile */}
        <input
          type="text"
          value={searchInput}
          onChange={(e) => onSearchInput(e.target.value)}
          placeholder={t("stacks.searchPlaceholder")}
          className="w-full sm:flex-1 sm:min-w-[180px] border border-neutral-200 dark:border-neutral-800 bg-neutral-50 dark:bg-neutral-900 px-3 py-1.5 text-sm text-neutral-900 dark:text-white placeholder-neutral-400 dark:placeholder-neutral-600 outline-none focus:border-orange-500"
        />
      </div>

      {/* Tool filter */}
      <div className="mb-6 flex items-center gap-2 flex-wrap">
        <select
          value=""
          onChange={(e) => {
            addTool(e.target.value);
            e.target.value = "";
          }}
          className="border border-neutral-200 dark:border-neutral-800 bg-neutral-50 dark:bg-neutral-900 px-2 sm:px-3 py-1.5 text-sm text-neutral-700 dark:text-neutral-300 outline-none"
        >
          <option value="">{t("stacks.filterTool")}</option>
          {availableTools.map((tl) => (
            <option key={tl.name} value={tl.name}>
              {tl.name}
            </option>
          ))}
        </select>
        {selectedTools.map((tool) => (
          <span
            key={tool}
            className="inline-flex items-center gap-1 border border-orange-500/30 bg-orange-500/10 px-2 py-1 text-xs text-orange-500"
          >
            {tool}
            <button
              onClick={() => removeTool(tool)}
              className="ml-1 text-orange-500/60 hover:text-orange-500"
            >
              &times;
            </button>
          </span>
        ))}
      </div>

      {/* Stack list */}
      {loading ? (
        Array.from({ length: 5 }).map((_, i) => <StackCardSkeleton key={i} />)
      ) : stacks.length === 0 ? (
        <div className="py-16 text-center">
          {search || selectedTools.length > 0 ? (
            <p className="text-neutral-500">
              {t("stacks.noResults")}
            </p>
          ) : (
            <p className="text-neutral-500">
              {t("stacks.noStacks")}{" "}
              <LocaleLink href="/new" className="text-orange-500 hover:underline">
                {t("stacks.shareStack")}
              </LocaleLink>
            </p>
          )}
        </div>
      ) : (
        <>
          {stacks.map((stack) => (
            <StackCard key={stack.id} stack={stack} />
          ))}
          {hasMore && <div ref={sentinelRef} className="h-1" />}
          {loadingMore && (
            <p className="mt-4 text-center text-sm text-neutral-500">
              {t("common.loading")}...
            </p>
          )}
        </>
      )}

      <div className="mt-16">
        <Footer />
      </div>
    </div>
  );
}
