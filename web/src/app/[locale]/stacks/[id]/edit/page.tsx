"use client";

import { useState, useEffect, use } from "react";
import { LocaleLink } from "@/components/locale-link";
import { useT, useLocaleRouter } from "@/lib/locale-context";
import { getStack, updateStack } from "@/lib/api";
import type { StackDetail, CreateToolInput } from "@/lib/types";
import { CATEGORIES, SCALES, TOOL_CATEGORIES, VERDICTS, SCALE_LABELS } from "@/lib/types";
import { useAuth } from "@/lib/auth-context";
import { Footer } from "@/components/footer";

export default function EditStackPage({ params }: { params: Promise<{ id: string }> }) {
  const { id } = use(params);
  const router = useLocaleRouter();
  const t = useT();
  const { user } = useAuth();
  const [stack, setStack] = useState<StackDetail | null>(null);
  const [loading, setLoading] = useState(true);
  const [saving, setSaving] = useState(false);
  const [error, setError] = useState("");

  // Form state
  const [projectName, setProjectName] = useState("");
  const [description, setDescription] = useState("");
  const [category, setCategory] = useState("");
  const [url, setUrl] = useState("");
  const [scale, setScale] = useState("");
  const [lessons, setLessons] = useState("");
  const [tools, setTools] = useState<CreateToolInput[]>([]);

  useEffect(() => {
    getStack(id)
      .then((data) => {
        setStack(data);
        setProjectName(data.project_name);
        setDescription(data.description);
        setCategory(data.category);
        setUrl(data.url || "");
        setScale(data.scale);
        setLessons(data.lessons || "");
        setTools(
          data.tools.map((tl) => ({
            name: tl.name,
            category: tl.category,
            why: tl.why,
            cost: tl.cost || undefined,
            verdict: tl.verdict,
          }))
        );
        document.title = `Edit ${data.project_name} — Stackpedia`;
      })
      .catch((e) => setError(e.message))
      .finally(() => setLoading(false));
  }, [id]);

  const addTool = () => {
    setTools([...tools, { name: "", category: "backend", why: "", verdict: "good" }]);
  };

  const removeTool = (index: number) => {
    setTools(tools.filter((_, i) => i !== index));
  };

  const updateTool = (index: number, field: keyof CreateToolInput, value: string) => {
    setTools(tools.map((tl, i) => (i === index ? { ...tl, [field]: value } : tl)));
  };

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    if (!user || !stack) return;

    setSaving(true);
    setError("");
    try {
      await updateStack(id, {
        project_name: projectName,
        description,
        category,
        url: url || undefined,
        scale,
        lessons: lessons || undefined,
        tools: tools.filter((tl) => tl.name.trim()),
      });
      router.push(`/stacks/${id}`);
    } catch (e) {
      setError(e instanceof Error ? e.message : "failed to update");
    } finally {
      setSaving(false);
    }
  };

  if (loading) {
    return (
      <div className="mx-auto max-w-3xl px-4 pt-8">
        <div className="animate-pulse">
          <div className="h-8 w-48 bg-neutral-200 dark:bg-neutral-800 mb-4" />
          <div className="h-10 bg-neutral-200 dark:bg-neutral-800 mb-3" />
          <div className="h-10 bg-neutral-200 dark:bg-neutral-800 mb-3" />
          <div className="h-10 bg-neutral-200 dark:bg-neutral-800 mb-3" />
        </div>
      </div>
    );
  }

  if (!stack) {
    return (
      <div className="mx-auto max-w-3xl px-4 pt-8 text-center">
        <p className="text-red-400">{error || t("common.stackNotFound")}</p>
        <LocaleLink href="/stacks" className="mt-4 inline-block text-sm text-orange-500 hover:underline">
          {t("edit.backStacks")}
        </LocaleLink>
      </div>
    );
  }

  if (!user || user.user_id !== stack.creator_id) {
    return (
      <div className="mx-auto max-w-3xl px-4 pt-8 text-center">
        <p className="text-red-400">{t("edit.notOwner")}</p>
        <LocaleLink href={`/stacks/${id}`} className="mt-4 inline-block text-sm text-orange-500 hover:underline">
          {t("edit.backStack")}
        </LocaleLink>
      </div>
    );
  }

  return (
    <div className="mx-auto max-w-3xl px-4 pt-8 pb-16">
      <div className="flex items-center justify-between mb-6">
        <h1 className="text-xl font-bold text-neutral-900 dark:text-white">{t("edit.title")}</h1>
        <LocaleLink href={`/stacks/${id}`} className="text-sm text-neutral-600 dark:text-neutral-400 hover:text-neutral-900 dark:hover:text-white">
          {t("edit.cancel")}
        </LocaleLink>
      </div>

      {error && <p className="mb-4 text-sm text-red-400">{error}</p>}

      <form onSubmit={handleSubmit} className="space-y-4">
        <div>
          <label className="mb-1 block text-xs text-neutral-500">{t("edit.projectName")}</label>
          <input
            value={projectName}
            onChange={(e) => setProjectName(e.target.value)}
            maxLength={100}
            required
            className="w-full border border-neutral-200 dark:border-neutral-800 bg-neutral-50 dark:bg-neutral-900 px-3 py-2 text-sm text-neutral-900 dark:text-white outline-none focus:border-orange-500"
          />
        </div>

        <div>
          <label className="mb-1 block text-xs text-neutral-500">{t("new.description")}</label>
          <input
            value={description}
            onChange={(e) => setDescription(e.target.value)}
            maxLength={200}
            required
            className="w-full border border-neutral-200 dark:border-neutral-800 bg-neutral-50 dark:bg-neutral-900 px-3 py-2 text-sm text-neutral-900 dark:text-white outline-none focus:border-orange-500"
          />
        </div>

        <div className="grid grid-cols-2 gap-4">
          <div>
            <label className="mb-1 block text-xs text-neutral-500">{t("new.category")}</label>
            <select
              value={category}
              onChange={(e) => setCategory(e.target.value)}
              className="w-full border border-neutral-200 dark:border-neutral-800 bg-neutral-50 dark:bg-neutral-900 px-3 py-2 text-sm text-neutral-900 dark:text-white outline-none focus:border-orange-500"
            >
              {CATEGORIES.map((c) => (
                <option key={c} value={c}>{c}</option>
              ))}
            </select>
          </div>
          <div>
            <label className="mb-1 block text-xs text-neutral-500">{t("new.scale")}</label>
            <select
              value={scale}
              onChange={(e) => setScale(e.target.value)}
              className="w-full border border-neutral-200 dark:border-neutral-800 bg-neutral-50 dark:bg-neutral-900 px-3 py-2 text-sm text-neutral-900 dark:text-white outline-none focus:border-orange-500"
            >
              {SCALES.map((s) => (
                <option key={s} value={s}>{SCALE_LABELS[s] || s}</option>
              ))}
            </select>
          </div>
        </div>

        <div>
          <label className="mb-1 block text-xs text-neutral-500">{t("new.projectUrl")} ({t("new.optional")})</label>
          <input
            value={url}
            onChange={(e) => setUrl(e.target.value)}
            placeholder="https://..."
            className="w-full border border-neutral-200 dark:border-neutral-800 bg-neutral-50 dark:bg-neutral-900 px-3 py-2 text-sm text-neutral-900 dark:text-white outline-none focus:border-orange-500"
          />
        </div>

        <div>
          <label className="mb-1 block text-xs text-neutral-500">{t("new.lessonsLabel")} ({t("new.optional")})</label>
          <textarea
            value={lessons}
            onChange={(e) => setLessons(e.target.value)}
            maxLength={500}
            rows={3}
            className="w-full border border-neutral-200 dark:border-neutral-800 bg-neutral-50 dark:bg-neutral-900 px-3 py-2 text-sm text-neutral-900 dark:text-white outline-none focus:border-orange-500 resize-none"
          />
        </div>

        {/* Tools */}
        <div>
          <div className="flex items-center justify-between mb-2">
            <label className="text-xs text-neutral-500">{t("new.tools")}</label>
            <button
              type="button"
              onClick={addTool}
              className="text-xs text-orange-500 hover:text-orange-500"
            >
              {t("new.addTool")}
            </button>
          </div>
          <div className="space-y-3">
            {tools.map((tool, i) => (
              <div key={i} className="border border-neutral-200 dark:border-neutral-800 p-3 space-y-2">
                <div className="flex items-center gap-2">
                  <input
                    value={tool.name}
                    onChange={(e) => updateTool(i, "name", e.target.value)}
                    placeholder={t("edit.toolName")}
                    className="flex-1 border border-neutral-200 dark:border-neutral-800 bg-neutral-50 dark:bg-neutral-900 px-2 py-1 text-sm text-neutral-900 dark:text-white outline-none focus:border-orange-500"
                  />
                  <select
                    value={tool.category}
                    onChange={(e) => updateTool(i, "category", e.target.value)}
                    className="border border-neutral-200 dark:border-neutral-800 bg-neutral-50 dark:bg-neutral-900 px-2 py-1 text-sm text-neutral-900 dark:text-white outline-none"
                  >
                    {TOOL_CATEGORIES.map((c) => (
                      <option key={c} value={c}>{c}</option>
                    ))}
                  </select>
                  <select
                    value={tool.verdict}
                    onChange={(e) => updateTool(i, "verdict", e.target.value)}
                    className="border border-neutral-200 dark:border-neutral-800 bg-neutral-50 dark:bg-neutral-900 px-2 py-1 text-sm text-neutral-900 dark:text-white outline-none"
                  >
                    {VERDICTS.map((v) => (
                      <option key={v} value={v}>{v}</option>
                    ))}
                  </select>
                  <button
                    type="button"
                    onClick={() => removeTool(i)}
                    className="text-xs text-red-400 hover:text-red-300"
                  >
                    {t("common.remove")}
                  </button>
                </div>
                <input
                  value={tool.why}
                  onChange={(e) => updateTool(i, "why", e.target.value)}
                  placeholder={t("edit.whyTool")}
                  className="w-full border border-neutral-200 dark:border-neutral-800 bg-neutral-50 dark:bg-neutral-900 px-2 py-1 text-sm text-neutral-900 dark:text-white outline-none focus:border-orange-500"
                />
                <input
                  value={tool.cost || ""}
                  onChange={(e) => updateTool(i, "cost", e.target.value)}
                  placeholder={t("edit.cost")}
                  className="w-full border border-neutral-200 dark:border-neutral-800 bg-neutral-50 dark:bg-neutral-900 px-2 py-1 text-sm text-neutral-900 dark:text-white outline-none focus:border-orange-500"
                />
              </div>
            ))}
          </div>
        </div>

        <button
          type="submit"
          disabled={saving || !projectName.trim() || !description.trim() || tools.length === 0}
          className="w-full bg-orange-500 py-2 text-sm font-bold text-black hover:bg-orange-600 dark:hover:bg-orange-400 disabled:opacity-50"
        >
          {saving ? t("edit.saving") : t("edit.save")}
        </button>
      </form>

      <div className="mt-16">
        <Footer />
      </div>
    </div>
  );
}
