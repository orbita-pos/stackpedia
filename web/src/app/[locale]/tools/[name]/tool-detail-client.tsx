"use client";

import { useState, useEffect, use } from "react";
import { LocaleLink } from "@/components/locale-link";
import { useT } from "@/lib/locale-context";
import { getTool, listTools, getToolPairs, getToolAlternatives } from "@/lib/api";
import type { ToolDetailEntry, ToolDirectoryEntry, ToolPairingEntry, ToolAlternative } from "@/lib/types";
import { VERDICT_COLORS, VERDICT_DOT } from "@/lib/types";
import { Footer } from "@/components/footer";

export function ToolDetailClient({ params }: { params: Promise<{ name: string }> }) {
  const { name } = use(params);
  const toolName = decodeURIComponent(name);
  const t = useT();

  const [entries, setEntries] = useState<ToolDetailEntry[]>([]);
  const [meta, setMeta] = useState<ToolDirectoryEntry | null>(null);
  const [pairs, setPairs] = useState<ToolPairingEntry[]>([]);
  const [alternatives, setAlternatives] = useState<ToolAlternative[]>([]);
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    Promise.all([
      getTool(toolName).catch(() => []),
      listTools().catch(() => []),
      getToolPairs(toolName).catch(() => []),
      getToolAlternatives(toolName).catch(() => []),
    ]).then(([e, allTools, p, alt]) => {
      setEntries(e);
      setPairs(p);
      setAlternatives(alt);
      const found = allTools.find(
        (tl) => tl.name.toLowerCase() === toolName.toLowerCase()
      );
      setMeta(found || null);
      setLoading(false);
    });
  }, [toolName]);

  const verdictCounts = entries.reduce(
    (acc, e) => {
      acc[e.verdict] = (acc[e.verdict] || 0) + 1;
      return acc;
    },
    {} as Record<string, number>
  );
  const totalVerdicts = entries.length || 1;

  if (loading) {
    return (
      <div className="mx-auto max-w-3xl px-4 pt-8">
        <div className="animate-pulse">
          <div className="h-8 w-48 bg-neutral-200 dark:bg-neutral-800 mb-3" />
          <div className="h-4 w-32 bg-neutral-200 dark:bg-neutral-800 mb-8" />
          <div className="h-3 w-full bg-neutral-200 dark:bg-neutral-800 mb-2" />
          <div className="h-3 w-full bg-neutral-200 dark:bg-neutral-800 mb-8" />
          <div className="h-4 w-24 bg-neutral-200 dark:bg-neutral-800 mb-3" />
          {Array.from({ length: 4 }).map((_, i) => (
            <div key={i} className="h-20 bg-neutral-200 dark:bg-neutral-800 mb-px" />
          ))}
        </div>
      </div>
    );
  }

  return (
    <div className="mx-auto max-w-3xl px-4 pt-8 pb-16">
      {/* Header */}
      <h1 className="text-2xl font-bold text-neutral-900 dark:text-white">{toolName}</h1>
      <div className="mt-1 flex items-center gap-3 text-sm text-neutral-500">
        {meta && <span>{meta.category}</span>}
        <span>
          {entries.length} stack{entries.length !== 1 ? "s" : ""} use this
        </span>
      </div>

      {/* Verdict bar */}
      {entries.length > 0 && (
        <div className="mt-6">
          <div className="mb-2 flex flex-wrap items-center gap-3 sm:gap-4 text-xs text-neutral-500">
            {(["love", "good", "meh", "regret"] as const).map((v) =>
              verdictCounts[v] ? (
                <span key={v} className="flex items-center gap-1">
                  <span className={`inline-block h-2 w-2 ${VERDICT_DOT[v]}`} />
                  {verdictCounts[v]} {v}
                </span>
              ) : null
            )}
          </div>
          <div className="flex h-3 overflow-hidden border border-neutral-200 dark:border-neutral-800">
            {(["love", "good", "meh", "regret"] as const).map((v) =>
              verdictCounts[v] ? (
                <div
                  key={v}
                  className={`${VERDICT_DOT[v]} transition-all duration-500`}
                  style={{ width: `${(verdictCounts[v] / totalVerdicts) * 100}%` }}
                />
              ) : null
            )}
          </div>
        </div>
      )}

      {/* Stack entries */}
      <div className="mt-8">
        <h2 className="mb-4 text-sm font-bold uppercase tracking-wider text-neutral-500">
          {t("tool.whatDevsSay")}
        </h2>
        {entries.length === 0 ? (
          <p className="py-12 text-center text-sm text-neutral-500">
            {t("tool.noUsage")}{" "}
            <LocaleLink href="/new" className="text-orange-500 hover:underline">
              {t("tool.beFirst")}
            </LocaleLink>
            .
          </p>
        ) : (
          <div className="border border-neutral-200 dark:border-neutral-800">
            {entries.map((entry, i) => (
              <div
                key={`${entry.stack_id}-${i}`}
                className={`p-4 ${i < entries.length - 1 ? "border-b border-neutral-200 dark:border-neutral-800" : ""}`}
              >
                <div className="flex items-center gap-3 flex-wrap mb-2">
                  <LocaleLink
                    href={`/stacks/${entry.stack_id}`}
                    className="font-bold text-neutral-900 dark:text-white hover:text-orange-500"
                  >
                    {entry.project_name}
                  </LocaleLink>
                  <span
                    className={`border px-2 py-0.5 text-xs ${VERDICT_COLORS[entry.verdict] || "text-neutral-600 dark:text-neutral-400"}`}
                  >
                    {entry.verdict}
                  </span>
                  {entry.cost && (
                    <span className="text-xs text-neutral-500">{entry.cost}</span>
                  )}
                </div>
                <p className="text-sm text-neutral-600 dark:text-neutral-400">{entry.why}</p>
              </div>
            ))}
          </div>
        )}
      </div>

      {/* Commonly paired with */}
      {pairs.length > 0 && (
        <div className="mt-8">
          <h2 className="mb-4 text-sm font-bold uppercase tracking-wider text-neutral-500">
            {t("tool.pairedWith")}
          </h2>
          <div className="flex flex-wrap gap-2">
            {pairs.map((p) => (
              <LocaleLink
                key={p.name}
                href={`/tools/${encodeURIComponent(p.name)}`}
                className="flex items-center gap-2 border border-neutral-200 dark:border-neutral-800 bg-neutral-50 dark:bg-neutral-900 px-3 py-1.5 text-xs text-neutral-700 dark:text-neutral-300 hover:border-orange-500/50 hover:text-neutral-900 dark:hover:text-white"
              >
                {p.name}
                <span className="text-neutral-500">{p.category}</span>
                <span className="text-orange-500/60">{p.pair_count}x</span>
              </LocaleLink>
            ))}
          </div>
        </div>
      )}

      {/* Alternatives */}
      {alternatives.length > 0 && (
        <div className="mt-8">
          <h2 className="mb-4 text-sm font-bold uppercase tracking-wider text-neutral-500">
            {t("tool.alternatives")}
          </h2>
          <div className="flex flex-wrap gap-2">
            {alternatives.map((alt) => (
              <LocaleLink
                key={alt.name}
                href={`/tools/${encodeURIComponent(alt.name)}`}
                className="flex items-center gap-2 border border-neutral-200 dark:border-neutral-800 bg-neutral-50 dark:bg-neutral-900 px-3 py-1.5 text-xs text-neutral-700 dark:text-neutral-300 hover:border-orange-500/50 hover:text-neutral-900 dark:hover:text-white"
              >
                {alt.name}
                <span className="text-neutral-500">{alt.category}</span>
                <span className="text-orange-500/60">
                  {alt.times_chosen}x {t("tool.timesChosen")}
                </span>
              </LocaleLink>
            ))}
          </div>
        </div>
      )}

      <div className="mt-16">
        <Footer />
      </div>
    </div>
  );
}
