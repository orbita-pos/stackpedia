import type { ToolResponse } from "./types";

export interface ParsedCost {
  type: "fixed" | "free" | "variable" | "unknown";
  monthly: number | null;
  raw: string;
}

export function parseCost(raw: string | null | undefined): ParsedCost {
  if (!raw || raw.trim() === "") {
    return { type: "unknown", monthly: null, raw: "" };
  }

  const s = raw.trim().toLowerCase();

  // Free
  if (s === "free" || s === "$0" || s === "$0/mo" || s === "0") {
    return { type: "free", monthly: 0, raw };
  }

  // Variable pricing like "2.9%+30c", "pay per use", "usage-based"
  if (s.includes("%") || s.includes("per use") || s.includes("usage")) {
    return { type: "variable", monthly: null, raw };
  }

  // Fixed monthly: "$20/mo", "$20/month", "$20"
  const match = s.match(/^\$?([\d,.]+)\s*(?:\/\s*mo(?:nth)?)?$/);
  if (match) {
    const amount = parseFloat(match[1].replace(/,/g, ""));
    if (!isNaN(amount)) {
      return { type: amount === 0 ? "free" : "fixed", monthly: amount, raw };
    }
  }

  // Yearly: "$240/yr", "$240/year"
  const yrMatch = s.match(/^\$?([\d,.]+)\s*\/\s*y(?:ea)?r$/);
  if (yrMatch) {
    const yearly = parseFloat(yrMatch[1].replace(/,/g, ""));
    if (!isNaN(yearly)) {
      const monthly = Math.round((yearly / 12) * 100) / 100;
      return { type: monthly === 0 ? "free" : "fixed", monthly, raw };
    }
  }

  return { type: "unknown", monthly: null, raw };
}

export interface CostSummaryData {
  total: number | null;
  tools: { name: string; parsed: ParsedCost }[];
  freeCount: number;
  paidCount: number;
  variableCount: number;
  unknownCount: number;
}

export function summarizeCosts(tools: ToolResponse[]): CostSummaryData {
  const parsed = tools.map((t) => ({
    name: t.name,
    parsed: parseCost(t.cost),
  }));

  let freeCount = 0;
  let paidCount = 0;
  let variableCount = 0;
  let unknownCount = 0;
  let total = 0;
  let hasVariable = false;

  for (const { parsed: p } of parsed) {
    switch (p.type) {
      case "free":
        freeCount++;
        break;
      case "fixed":
        paidCount++;
        total += p.monthly!;
        break;
      case "variable":
        variableCount++;
        hasVariable = true;
        break;
      default:
        unknownCount++;
        break;
    }
  }

  return {
    total: hasVariable ? null : total,
    tools: parsed,
    freeCount,
    paidCount,
    variableCount,
    unknownCount,
  };
}
