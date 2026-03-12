"use client";

import { Suspense, useState, useEffect } from "react";
import { useSearchParams } from "next/navigation";
import { useT, useLocaleRouter } from "@/lib/locale-context";
import { createStack, getStack } from "@/lib/api";
import { useAuth } from "@/lib/auth-context";
import {
  CATEGORIES,
  SCALES,
  SCALE_LABELS,
  TOOL_CATEGORIES,
  VERDICTS,
} from "@/lib/types";
import type { CreateToolInput } from "@/lib/types";
import { Footer } from "@/components/footer";

const emptyTool = (): CreateToolInput => ({
  name: "",
  category: "backend",
  why: "",
  cost: "",
  verdict: "good",
});

export default function NewStackPage() {
  return (
    <Suspense
      fallback={
        <div className="mx-auto max-w-2xl px-4 pt-8">
          <div className="h-8 w-48 bg-neutral-200 dark:bg-neutral-800 mb-2" />
          <div className="h-4 w-64 bg-neutral-200 dark:bg-neutral-800 mb-8" />
          <div className="animate-pulse space-y-6">
            {Array.from({ length: 4 }).map((_, i) => (
              <div key={i} className="h-12 bg-neutral-200 dark:bg-neutral-800" />
            ))}
          </div>
        </div>
      }
    >
      <NewStackContent />
    </Suspense>
  );
}

function NewStackContent() {
  const router = useLocaleRouter();
  const t = useT();
  const { user, requireAuth } = useAuth();
  const searchParams = useSearchParams();
  const forkId = searchParams.get("fork");

  const [projectName, setProjectName] = useState("");
  const [description, setDescription] = useState("");
  const [category, setCategory] = useState("saas");
  const [url, setUrl] = useState("");
  const [scale, setScale] = useState("hobby");
  const [tools, setTools] = useState<CreateToolInput[]>([emptyTool()]);
  const [lessons, setLessons] = useState("");
  const [error, setError] = useState("");
  const [submitting, setSubmitting] = useState(false);
  const [newToolIndex, setNewToolIndex] = useState<number | null>(null);
  const [forkedFrom, setForkedFrom] = useState<string | null>(null);

  useEffect(() => {
    if (!forkId) return;
    setForkedFrom(forkId);
    getStack(forkId)
      .then((stack) => {
        setProjectName(`Fork of ${stack.project_name}`);
        setDescription(stack.description);
        setCategory(stack.category);
        setUrl(stack.url || "");
        setScale(stack.scale);
        setTools(
          stack.tools.map((tl) => ({
            name: tl.name,
            category: tl.category,
            why: tl.why,
            cost: tl.cost || "",
            verdict: tl.verdict,
          }))
        );
        setLessons(stack.lessons || "");
      })
      .catch(() => {});
  }, [forkId]);

  const updateTool = (index: number, field: keyof CreateToolInput, value: string) => {
    setTools((prev) =>
      prev.map((tl, i) => (i === index ? { ...tl, [field]: value } : tl))
    );
  };

  const removeTool = (index: number) => {
    if (tools.length <= 1) return;
    setTools((prev) => prev.filter((_, i) => i !== index));
  };

  const addTool = () => {
    setNewToolIndex(tools.length);
    setTools((prev) => [...prev, emptyTool()]);
    setTimeout(() => setNewToolIndex(null), 300);
  };

  const validate = (): string | null => {
    if (!projectName.trim()) return "project name is required";
    if (projectName.length > 100) return "project name too long (max 100)";
    if (!description.trim()) return "description is required";
    if (description.length > 200) return "description too long (max 200)";
    if (tools.length === 0) return "add at least one tool";
    for (let i = 0; i < tools.length; i++) {
      if (!tools[i].name.trim()) return `tool ${i + 1}: name is required`;
      if (!tools[i].why.trim()) return `tool ${i + 1}: "why" is required`;
      if (tools[i].why.length > 300) return `tool ${i + 1}: "why" too long (max 300)`;
    }
    if (lessons.length > 500) return "lessons too long (max 500)";
    return null;
  };

  const handleSubmit = (e: React.FormEvent) => {
    e.preventDefault();

    const validationError = validate();
    if (validationError) {
      setError(validationError);
      return;
    }

    requireAuth(async () => {
      setSubmitting(true);
      setError("");
      try {
        const stack = await createStack({
          project_name: projectName.trim(),
          description: description.trim(),
          category,
          url: url.trim() || undefined,
          scale,
          tools: tools.map((tl) => ({
            ...tl,
            name: tl.name.trim(),
            why: tl.why.trim(),
            cost: tl.cost?.trim() || undefined,
          })),
          lessons: lessons.trim() || undefined,
          forked_from: forkedFrom || undefined,
        });
        router.push(`/stacks/${stack.id}`);
      } catch (err) {
        setError(err instanceof Error ? err.message : "failed to create stack");
      } finally {
        setSubmitting(false);
      }
    });
  };

  return (
    <div className="mx-auto max-w-2xl px-4 pt-8 pb-16">
      <h1 className="mb-2 text-2xl font-bold text-neutral-900 dark:text-white">{t("new.title")}</h1>
      <p className="mb-8 text-sm text-neutral-500">
        {t("new.subtitle")}
      </p>

      <form onSubmit={handleSubmit} className="space-y-6">
        {/* Project name */}
        <div>
          <label className="mb-1 block text-xs uppercase tracking-wider text-neutral-500">
            {t("new.projectName")} *
          </label>
          <input
            type="text"
            value={projectName}
            onChange={(e) => setProjectName(e.target.value)}
            placeholder="my saas app"
            maxLength={100}
            className="w-full border border-neutral-200 dark:border-neutral-800 bg-neutral-50 dark:bg-neutral-900 px-3 py-2 text-sm text-neutral-900 dark:text-white placeholder-neutral-400 dark:placeholder-neutral-600 outline-none focus:border-orange-500"
          />
        </div>

        {/* Description */}
        <div>
          <label className="mb-1 block text-xs uppercase tracking-wider text-neutral-500">
            {t("new.description")} * <span className="text-neutral-400 dark:text-neutral-600">({description.length}/200)</span>
          </label>
          <textarea
            value={description}
            onChange={(e) => setDescription(e.target.value)}
            placeholder="a brief description of your project"
            maxLength={200}
            rows={2}
            className="w-full border border-neutral-200 dark:border-neutral-800 bg-neutral-50 dark:bg-neutral-900 px-3 py-2 text-sm text-neutral-900 dark:text-white placeholder-neutral-400 dark:placeholder-neutral-600 outline-none focus:border-orange-500 resize-none"
          />
        </div>

        {/* Category + Scale */}
        <div className="grid grid-cols-1 gap-4 sm:grid-cols-2">
          <div>
            <label className="mb-1 block text-xs uppercase tracking-wider text-neutral-500">
              {t("new.category")} *
            </label>
            <select
              value={category}
              onChange={(e) => setCategory(e.target.value)}
              className="w-full border border-neutral-200 dark:border-neutral-800 bg-neutral-50 dark:bg-neutral-900 px-3 py-2 text-sm text-neutral-700 dark:text-neutral-300 outline-none focus:border-orange-500"
            >
              {CATEGORIES.map((c) => (
                <option key={c} value={c}>
                  {c}
                </option>
              ))}
            </select>
          </div>
          <div>
            <label className="mb-1 block text-xs uppercase tracking-wider text-neutral-500">
              {t("new.scale")} *
            </label>
            <select
              value={scale}
              onChange={(e) => setScale(e.target.value)}
              className="w-full border border-neutral-200 dark:border-neutral-800 bg-neutral-50 dark:bg-neutral-900 px-3 py-2 text-sm text-neutral-700 dark:text-neutral-300 outline-none focus:border-orange-500"
            >
              {SCALES.map((s) => (
                <option key={s} value={s}>
                  {SCALE_LABELS[s]}
                </option>
              ))}
            </select>
          </div>
        </div>

        {/* URL */}
        <div>
          <label className="mb-1 block text-xs uppercase tracking-wider text-neutral-500">
            {t("new.projectUrl")} <span className="text-neutral-400 dark:text-neutral-600">({t("new.optional")})</span>
          </label>
          <input
            type="text"
            value={url}
            onChange={(e) => setUrl(e.target.value)}
            placeholder="https://..."
            className="w-full border border-neutral-200 dark:border-neutral-800 bg-neutral-50 dark:bg-neutral-900 px-3 py-2 text-sm text-neutral-900 dark:text-white placeholder-neutral-400 dark:placeholder-neutral-600 outline-none focus:border-orange-500"
          />
        </div>

        {/* Tools */}
        <div>
          <label className="mb-3 block text-xs uppercase tracking-wider text-neutral-500">
            {t("new.tools")} * ({tools.length})
          </label>
          <div className="space-y-4">
            {tools.map((tool, i) => (
              <div
                key={i}
                className={`border border-neutral-200 dark:border-neutral-800 p-4 overflow-hidden ${i === newToolIndex ? "tool-slide-in" : ""}`}
              >
                <div className="mb-3 flex items-center justify-between">
                  <span className="text-xs text-neutral-500">{t("new.toolN")} {i + 1}</span>
                  {tools.length > 1 && (
                    <button
                      type="button"
                      onClick={() => removeTool(i)}
                      className="text-xs text-red-400 hover:text-red-300"
                    >
                      {t("common.remove")}
                    </button>
                  )}
                </div>
                <div className="grid grid-cols-1 gap-3 mb-3 sm:grid-cols-2">
                  <input
                    type="text"
                    value={tool.name}
                    onChange={(e) => updateTool(i, "name", e.target.value)}
                    placeholder="tool name (e.g. Turso)"
                    className="border border-neutral-200 dark:border-neutral-800 bg-neutral-50 dark:bg-neutral-900 px-3 py-1.5 text-sm text-neutral-900 dark:text-white placeholder-neutral-400 dark:placeholder-neutral-600 outline-none focus:border-orange-500"
                  />
                  <select
                    value={tool.category}
                    onChange={(e) => updateTool(i, "category", e.target.value)}
                    className="border border-neutral-200 dark:border-neutral-800 bg-neutral-50 dark:bg-neutral-900 px-3 py-1.5 text-sm text-neutral-700 dark:text-neutral-300 outline-none"
                  >
                    {TOOL_CATEGORIES.map((c) => (
                      <option key={c} value={c}>
                        {c}
                      </option>
                    ))}
                  </select>
                </div>
                <textarea
                  value={tool.why}
                  onChange={(e) => updateTool(i, "why", e.target.value)}
                  placeholder="why did you choose this? (max 300 chars)"
                  maxLength={300}
                  rows={2}
                  className="mb-3 w-full border border-neutral-200 dark:border-neutral-800 bg-neutral-50 dark:bg-neutral-900 px-3 py-1.5 text-sm text-neutral-900 dark:text-white placeholder-neutral-400 dark:placeholder-neutral-600 outline-none focus:border-orange-500 resize-none"
                />
                <div className="grid grid-cols-1 gap-3 sm:grid-cols-2">
                  <input
                    type="text"
                    value={tool.cost || ""}
                    onChange={(e) => updateTool(i, "cost", e.target.value)}
                    placeholder="cost (e.g. $20/mo, free)"
                    className="border border-neutral-200 dark:border-neutral-800 bg-neutral-50 dark:bg-neutral-900 px-3 py-1.5 text-sm text-neutral-900 dark:text-white placeholder-neutral-400 dark:placeholder-neutral-600 outline-none focus:border-orange-500"
                  />
                  <select
                    value={tool.verdict}
                    onChange={(e) => updateTool(i, "verdict", e.target.value)}
                    className="border border-neutral-200 dark:border-neutral-800 bg-neutral-50 dark:bg-neutral-900 px-3 py-1.5 text-sm text-neutral-700 dark:text-neutral-300 outline-none"
                  >
                    {VERDICTS.map((v) => (
                      <option key={v} value={v}>
                        {v}
                      </option>
                    ))}
                  </select>
                </div>
              </div>
            ))}
          </div>
          <button
            type="button"
            onClick={addTool}
            className="mt-3 w-full border border-dashed border-neutral-300 dark:border-neutral-700 py-2 text-sm text-neutral-500 hover:text-neutral-900 dark:hover:text-white hover:border-orange-500/50"
          >
            {t("new.addTool")}
          </button>
        </div>

        {/* Lessons */}
        <div>
          <label className="mb-1 block text-xs uppercase tracking-wider text-neutral-500">
            {t("new.lessonsLabel")}{" "}
            <span className="text-neutral-400 dark:text-neutral-600">({lessons.length}/500, {t("new.optional")})</span>
          </label>
          <textarea
            value={lessons}
            onChange={(e) => setLessons(e.target.value)}
            placeholder={t("new.lessonsPlaceholder")}
            maxLength={500}
            rows={3}
            className="w-full border border-neutral-200 dark:border-neutral-800 bg-neutral-50 dark:bg-neutral-900 px-3 py-2 text-sm text-neutral-900 dark:text-white placeholder-neutral-400 dark:placeholder-neutral-600 outline-none focus:border-orange-500 resize-none"
          />
        </div>

        {/* Error + Submit */}
        {error && <p className="text-sm text-red-400">{error}</p>}

        <button
          type="submit"
          disabled={submitting}
          className="w-full bg-orange-500 py-3 text-sm font-bold text-black hover:bg-orange-600 dark:hover:bg-orange-400 disabled:opacity-50"
        >
          {submitting ? t("new.publishing") : t("new.publish")}
        </button>
      </form>

      <div className="mt-16">
        <Footer />
      </div>
    </div>
  );
}
