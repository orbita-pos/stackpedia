"use client";

import type { ToolResponse } from "@/lib/types";
import { summarizeCosts } from "@/lib/cost-parser";

export function CostSummary({ tools }: { tools: ToolResponse[] }) {
  const summary = summarizeCosts(tools);

  if (summary.tools.length === 0) return null;

  // Don't show if everything is unknown
  if (summary.unknownCount === summary.tools.length) return null;

  return (
    <section className="mt-8">
      <h2 className="mb-4 text-sm font-bold uppercase tracking-wider text-neutral-500 dark:text-neutral-600">
        cost breakdown
      </h2>
      <div className="border border-neutral-200 dark:border-neutral-800">
        {/* Summary row */}
        <div className="flex items-center justify-between p-4 border-b border-neutral-200 dark:border-neutral-800">
          <div className="flex items-center gap-4 text-xs text-neutral-500 flex-wrap">
            {summary.freeCount > 0 && (
              <span className="text-green-500">{summary.freeCount} free</span>
            )}
            {summary.paidCount > 0 && (
              <span className="text-blue-500">{summary.paidCount} paid</span>
            )}
            {summary.variableCount > 0 && (
              <span className="text-yellow-500">{summary.variableCount} variable</span>
            )}
            {summary.unknownCount > 0 && (
              <span>{summary.unknownCount} unlisted</span>
            )}
          </div>
          <div className="text-right">
            {summary.total !== null ? (
              <span className="text-sm font-bold text-neutral-900 dark:text-white">
                ${summary.total.toFixed(2)}
                <span className="text-xs text-neutral-500 font-normal">/mo</span>
              </span>
            ) : (
              <span className="text-sm text-yellow-500">
                ${summary.tools.reduce((s, t) => s + (t.parsed.monthly ?? 0), 0).toFixed(2)}+
                <span className="text-xs text-neutral-500 font-normal">/mo</span>
              </span>
            )}
          </div>
        </div>

        {/* Per-tool breakdown */}
        {summary.tools.map(({ name, parsed }) => (
          <div
            key={name}
            className="flex items-center justify-between px-4 py-2 border-b border-neutral-200 dark:border-neutral-800 last:border-b-0"
          >
            <span className="text-xs text-neutral-600 dark:text-neutral-400">{name}</span>
            <span className="text-xs">
              {parsed.type === "free" && (
                <span className="text-green-500">free</span>
              )}
              {parsed.type === "fixed" && (
                <span className="text-neutral-900 dark:text-white">${parsed.monthly!.toFixed(2)}/mo</span>
              )}
              {parsed.type === "variable" && (
                <span className="text-yellow-500">{parsed.raw}</span>
              )}
              {parsed.type === "unknown" && (
                <span className="text-neutral-400 dark:text-neutral-600">&mdash;</span>
              )}
            </span>
          </div>
        ))}
      </div>
    </section>
  );
}
