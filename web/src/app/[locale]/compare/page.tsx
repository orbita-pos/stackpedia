"use client";

import { useState, useEffect } from "react";
import { useT } from "@/lib/locale-context";
import { listTools, compareTools } from "@/lib/api";
import type { ToolDirectoryEntry, CompareResponse } from "@/lib/types";
import { VERDICT_DOT } from "@/lib/types";
import { Footer } from "@/components/footer";

export default function ComparePage() {
  const t = useT();
  const [allTools, setAllTools] = useState<ToolDirectoryEntry[]>([]);
  const [tool1, setTool1] = useState("");
  const [tool2, setTool2] = useState("");
  const [result, setResult] = useState<CompareResponse | null>(null);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState("");

  useEffect(() => {
    listTools()
      .then(setAllTools)
      .catch(() => {});
  }, []);

  const handleCompare = async () => {
    if (!tool1 || !tool2 || tool1 === tool2) {
      setError(t("compare.selectTwo"));
      return;
    }
    setLoading(true);
    setError("");
    setResult(null);
    try {
      const data = await compareTools(tool1, tool2);
      setResult(data);
    } catch (e) {
      setError(e instanceof Error ? e.message : "comparison failed");
    } finally {
      setLoading(false);
    }
  };

  return (
    <div className="mx-auto max-w-3xl px-4 pt-8 pb-16">
      <h1 className="text-2xl font-bold text-neutral-900 dark:text-white">{t("compare.title")}</h1>
      <p className="mt-1 text-sm text-neutral-500">
        {t("compare.subtitle")}
      </p>

      {/* Selectors */}
      <div className="mt-6 flex flex-col gap-3 sm:flex-row sm:items-end">
        <div className="flex-1">
          <label className="mb-1 block text-xs text-neutral-500">{t("compare.tool1")}</label>
          <select
            value={tool1}
            onChange={(e) => setTool1(e.target.value)}
            className="w-full border border-neutral-200 dark:border-neutral-800 bg-neutral-50 dark:bg-neutral-900 px-3 py-2 text-sm text-neutral-900 dark:text-white outline-none focus:border-orange-500"
          >
            <option value="">{t("compare.selectTool")}</option>
            {allTools.map((tl) => (
              <option key={tl.name} value={tl.name}>
                {tl.name} ({tl.stack_count})
              </option>
            ))}
          </select>
        </div>
        <span className="hidden sm:block text-neutral-500 pb-2">{t("compare.vs")}</span>
        <div className="flex-1">
          <label className="mb-1 block text-xs text-neutral-500">{t("compare.tool2")}</label>
          <select
            value={tool2}
            onChange={(e) => setTool2(e.target.value)}
            className="w-full border border-neutral-200 dark:border-neutral-800 bg-neutral-50 dark:bg-neutral-900 px-3 py-2 text-sm text-neutral-900 dark:text-white outline-none focus:border-orange-500"
          >
            <option value="">{t("compare.selectTool")}</option>
            {allTools.map((tl) => (
              <option key={tl.name} value={tl.name}>
                {tl.name} ({tl.stack_count})
              </option>
            ))}
          </select>
        </div>
        <button
          onClick={handleCompare}
          disabled={loading || !tool1 || !tool2}
          className="bg-orange-500 px-5 py-2 text-sm font-bold text-black hover:bg-orange-600 dark:hover:bg-orange-400 disabled:opacity-50 shrink-0"
        >
          {loading ? "..." : t("compare.compare")}
        </button>
      </div>

      {error && <p className="mt-3 text-sm text-red-400">{error}</p>}

      {/* Results */}
      {result && (
        <div className="mt-8">
          {/* Shared stacks */}
          <div className="mb-6 text-center">
            <span className="text-sm text-neutral-500">
              {t("compare.usedTogether")}{" "}
              <span className="text-neutral-900 dark:text-white font-bold">{result.shared_stacks}</span>{" "}
              stack{result.shared_stacks !== 1 ? "s" : ""}
            </span>
          </div>

          {/* Side by side */}
          <div className="grid grid-cols-1 gap-4 sm:grid-cols-2">
            {result.tools.map((tool) => {
              const total =
                tool.verdict_distribution.love +
                tool.verdict_distribution.good +
                tool.verdict_distribution.meh +
                tool.verdict_distribution.regret || 1;

              return (
                <div key={tool.name} className="border border-neutral-200 dark:border-neutral-800 p-4">
                  <h3 className="text-lg font-bold text-neutral-900 dark:text-white">{tool.name}</h3>
                  <p className="text-xs text-neutral-500 mb-3">
                    {tool.category} &middot; {tool.stack_count} stack
                    {tool.stack_count !== 1 ? "s" : ""}
                  </p>

                  {/* Verdict bar */}
                  <div className="mb-2 flex flex-wrap gap-2 text-xs text-neutral-500">
                    {(["love", "good", "meh", "regret"] as const).map((v) => {
                      const count = tool.verdict_distribution[v];
                      return count > 0 ? (
                        <span key={v} className="flex items-center gap-1">
                          <span className={`inline-block h-2 w-2 ${VERDICT_DOT[v]}`} />
                          {count} {v}
                        </span>
                      ) : null;
                    })}
                  </div>
                  <div className="flex h-3 overflow-hidden border border-neutral-200 dark:border-neutral-800 mb-4">
                    {(["love", "good", "meh", "regret"] as const).map((v) => {
                      const count = tool.verdict_distribution[v];
                      return count > 0 ? (
                        <div
                          key={v}
                          className={`${VERDICT_DOT[v]}`}
                          style={{ width: `${(count / total) * 100}%` }}
                        />
                      ) : null;
                    })}
                  </div>

                  {/* Sample whys */}
                  {tool.sample_whys.length > 0 && (
                    <div className="mb-3">
                      <p className="text-xs text-neutral-500 uppercase tracking-wider mb-1">
                        {t("compare.whatDevsSay")}
                      </p>
                      {tool.sample_whys.map((w, i) => (
                        <p key={i} className="text-xs text-neutral-600 dark:text-neutral-400 mb-1">
                          &ldquo;{w}&rdquo;
                        </p>
                      ))}
                    </div>
                  )}

                  {/* Costs */}
                  {tool.common_costs.length > 0 && (
                    <div>
                      <p className="text-xs text-neutral-500 uppercase tracking-wider mb-1">
                        {t("compare.pricing")}
                      </p>
                      <div className="flex flex-wrap gap-1">
                        {tool.common_costs.map((c, i) => (
                          <span
                            key={i}
                            className="border border-neutral-200 dark:border-neutral-800 px-2 py-0.5 text-xs text-neutral-600 dark:text-neutral-400"
                          >
                            {c}
                          </span>
                        ))}
                      </div>
                    </div>
                  )}
                </div>
              );
            })}
          </div>
        </div>
      )}

      <div className="mt-16">
        <Footer />
      </div>
    </div>
  );
}
