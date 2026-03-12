"use client";

import { useState, useEffect, use } from "react";
import { LocaleLink } from "@/components/locale-link";
import { useT, useLocaleRouter } from "@/lib/locale-context";
import { getStack, createComment, deleteStack, bookmarkStack, unbookmarkStack, isBookmarked } from "@/lib/api";
import type { StackDetail, CommentResponse } from "@/lib/types";
import { VERDICT_COLORS, SCALE_LABELS } from "@/lib/types";
import { relativeTime } from "@/lib/utils";
import { VoteButton } from "@/components/vote-button";
import { CostSummary } from "@/components/cost-summary";
import { Footer } from "@/components/footer";
import { PixelAvatar } from "@/components/pixel-avatar";
import { useAuth } from "@/lib/auth-context";
import { useToast } from "@/lib/toast-context";

export function StackDetailClient({ params }: { params: Promise<{ id: string }> }) {
  const { id } = use(params);
  const router = useLocaleRouter();
  const t = useT();
  const { user, requireAuth } = useAuth();
  const { toast } = useToast();
  const [stack, setStack] = useState<StackDetail | null>(null);
  const [comments, setComments] = useState<CommentResponse[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState("");
  const [commentText, setCommentText] = useState("");
  const [posting, setPosting] = useState(false);
  const [linkCopied, setLinkCopied] = useState(false);
  const [showDeleteConfirm, setShowDeleteConfirm] = useState(false);
  const [deleting, setDeleting] = useState(false);
  const [bookmarked, setBookmarked] = useState(false);
  const [exporting, setExporting] = useState(false);

  useEffect(() => {
    getStack(id)
      .then((data) => {
        setStack(data);
        setComments(data.comments);
        if (user) {
          isBookmarked(id).then(setBookmarked).catch(() => {});
        }
      })
      .catch((e) => setError(e.message))
      .finally(() => setLoading(false));
  }, [id, user]);

  const handleComment = () => {
    requireAuth(async () => {
      if (!commentText.trim() || posting) return;
      setPosting(true);
      try {
        const comment = await createComment(id, commentText.trim());
        setComments((prev) => [comment, ...prev]);
        setCommentText("");
        toast(t("toast.commentPosted"), "success");
      } catch (e) {
        setError(e instanceof Error ? e.message : "failed to post comment");
      } finally {
        setPosting(false);
      }
    });
  };

  const handleCopyLink = () => {
    navigator.clipboard.writeText(window.location.href);
    setLinkCopied(true);
    toast(t("common.copied"), "success");
    setTimeout(() => setLinkCopied(false), 2000);
  };

  const handleDelete = async () => {
    setDeleting(true);
    try {
      await deleteStack(id);
      router.push("/stacks");
    } catch (e) {
      setError(e instanceof Error ? e.message : "failed to delete stack");
      setDeleting(false);
      setShowDeleteConfirm(false);
    }
  };

  const handleToggleBookmark = async () => {
    try {
      if (bookmarked) {
        await unbookmarkStack(id);
        setBookmarked(false);
        toast(t("toast.bookmarkRemoved"), "info");
      } else {
        await bookmarkStack(id);
        setBookmarked(true);
        toast(t("toast.bookmarkAdded"), "success");
      }
    } catch {
      toast(t("toast.bookmarkError"), "error");
    }
  };

  const handleExportImage = async () => {
    if (!stack || exporting) return;
    setExporting(true);
    try {
      const canvas = document.createElement("canvas");
      canvas.width = 1200;
      canvas.height = 630;
      const ctx = canvas.getContext("2d")!;

      // Background
      ctx.fillStyle = "#0a0a0a";
      ctx.fillRect(0, 0, 1200, 630);

      // Branding
      ctx.fillStyle = "#f97316";
      ctx.fillRect(40, 40, 16, 16);
      ctx.fillStyle = "#ffffff";
      ctx.font = "bold 18px monospace";
      ctx.fillText("stackpedia", 66, 54);

      // Project name
      ctx.fillStyle = "#ffffff";
      ctx.font = "bold 36px monospace";
      const nameText = stack.project_name.length > 40 ? stack.project_name.slice(0, 40) + "..." : stack.project_name;
      ctx.fillText(nameText, 40, 120);

      // Description
      ctx.fillStyle = "#a3a3a3";
      ctx.font = "16px monospace";
      const descText = stack.description.length > 80 ? stack.description.slice(0, 80) + "..." : stack.description;
      ctx.fillText(descText, 40, 160);

      // Category + scale badges
      ctx.fillStyle = "#404040";
      ctx.fillRect(40, 190, 100, 28);
      ctx.fillStyle = "#a3a3a3";
      ctx.font = "13px monospace";
      ctx.fillText(stack.category, 50, 209);

      ctx.fillStyle = "#404040";
      ctx.fillRect(150, 190, 100, 28);
      ctx.fillStyle = "#a3a3a3";
      ctx.fillText(SCALE_LABELS[stack.scale] || stack.scale, 160, 209);

      // Tools list
      const verdictColors: Record<string, string> = {
        love: "#22c55e",
        good: "#3b82f6",
        meh: "#eab308",
        regret: "#ef4444",
      };
      const tools = stack.tools.slice(0, 8);
      tools.forEach((tool, i) => {
        const y = 250 + i * 40;
        // Verdict dot
        ctx.fillStyle = verdictColors[tool.verdict] || "#737373";
        ctx.fillRect(40, y + 4, 10, 10);
        // Tool name
        ctx.fillStyle = "#ffffff";
        ctx.font = "bold 15px monospace";
        ctx.fillText(tool.name, 60, y + 14);
        // Category
        ctx.fillStyle = "#737373";
        ctx.font = "13px monospace";
        ctx.fillText(tool.category, 260, y + 14);
        // Verdict
        ctx.fillStyle = verdictColors[tool.verdict] || "#737373";
        ctx.fillText(tool.verdict, 400, y + 14);
      });

      // Creator + bottom
      ctx.fillStyle = "#737373";
      ctx.font = "14px monospace";
      ctx.fillText(`by ${stack.creator_nickname}`, 40, 590);
      if (stack.url) {
        ctx.fillText(stack.url.length > 50 ? stack.url.slice(0, 50) + "..." : stack.url, 40, 610);
      }

      // Upvotes
      ctx.fillStyle = "#f97316";
      ctx.font = "bold 14px monospace";
      ctx.fillText(`▲ ${stack.upvotes}`, 1080, 54);

      canvas.toBlob((blob) => {
        if (!blob) return;
        const url = URL.createObjectURL(blob);
        const a = document.createElement("a");
        a.href = url;
        a.download = `${stack.project_name.replace(/[^a-z0-9]/gi, "-").toLowerCase()}-stack.png`;
        a.click();
        URL.revokeObjectURL(url);
      }, "image/png");
    } finally {
      setExporting(false);
    }
  };

  if (loading) {
    return (
      <div className="mx-auto max-w-3xl px-4 pt-8">
        <div className="animate-pulse">
          <div className="h-8 w-64 bg-neutral-200 dark:bg-neutral-800 mb-3" />
          <div className="h-4 w-full max-w-md bg-neutral-200 dark:bg-neutral-800 mb-2" />
          <div className="h-3 w-32 bg-neutral-200 dark:bg-neutral-800 mb-8" />
          <div className="h-4 w-20 bg-neutral-200 dark:bg-neutral-800 mb-3" />
          {Array.from({ length: 4 }).map((_, i) => (
            <div key={i} className="h-16 bg-neutral-200 dark:bg-neutral-800 mb-px" />
          ))}
          <div className="h-4 w-24 bg-neutral-200 dark:bg-neutral-800 mt-8 mb-3" />
          <div className="h-20 bg-neutral-200 dark:bg-neutral-800" />
        </div>
      </div>
    );
  }

  if (error || !stack) {
    return (
      <div className="mx-auto max-w-3xl px-4 pt-8 text-center">
        <p className="text-red-400">{error || t("common.stackNotFound")}</p>
        <LocaleLink href="/stacks" className="mt-4 inline-block text-sm text-orange-500 hover:underline">
          {t("common.back")}
        </LocaleLink>
      </div>
    );
  }

  return (
    <div className="mx-auto max-w-3xl px-4 pt-8 pb-16">
      {/* Forked-from indicator */}
      {stack.forked_from && (
        <div className="mb-2 text-xs text-neutral-500">
          {t("stack.forkedFrom")}{" "}
          <LocaleLink href={`/stacks/${stack.forked_from}`} className="text-orange-500 hover:underline">
            {stack.forked_from}
          </LocaleLink>
        </div>
      )}

      {/* Header */}
      <div className="flex items-start">
        <VoteButton stackId={stack.id} initialUpvotes={stack.upvotes} />
        <div className="flex-1 min-w-0">
          <h1 className="text-xl font-bold text-neutral-900 dark:text-white sm:text-2xl">{stack.project_name}</h1>
          <p className="mt-1 text-sm text-neutral-600 dark:text-neutral-400 sm:text-base">{stack.description}</p>
          <div className="mt-3 flex items-center gap-2 flex-wrap">
            <span className="border border-neutral-200 dark:border-neutral-800 px-2 py-0.5 text-xs text-neutral-500">
              {stack.category}
            </span>
            <span className="border border-neutral-200 dark:border-neutral-800 px-2 py-0.5 text-xs text-neutral-500">
              {SCALE_LABELS[stack.scale] || stack.scale}
            </span>
            {stack.url && (
              <a
                href={stack.url}
                target="_blank"
                rel="noopener noreferrer"
                className="text-xs text-orange-500 hover:underline truncate max-w-[200px]"
              >
                {stack.url}
              </a>
            )}
          </div>
          <div className="mt-2 flex items-center gap-2 text-xs text-neutral-500">
            <PixelAvatar nickname={stack.creator_nickname} size={16} />
            <span>
              by{" "}
              <LocaleLink href={`/u/${stack.creator_nickname}`} className="hover:text-orange-500">
                {stack.creator_nickname}
              </LocaleLink>{" "}
              &middot; {relativeTime(stack.created_at)}
              {stack.updated_at && <> &middot; {t("stack.updated")} {relativeTime(stack.updated_at)}</>}
            </span>
          </div>
          {/* Actions row */}
          <div className="mt-2 flex items-center gap-2 flex-wrap">
            <button
              onClick={handleCopyLink}
              className="inline-block border border-neutral-200 dark:border-neutral-800 px-3 py-1 text-xs text-neutral-600 dark:text-neutral-400 hover:text-neutral-900 dark:hover:text-white hover:border-orange-500/50"
            >
              {linkCopied ? t("common.copied") + "!" : t("stack.copyLink")}
            </button>
            <button
              onClick={handleExportImage}
              disabled={exporting}
              className="inline-block border border-neutral-200 dark:border-neutral-800 px-3 py-1 text-xs text-neutral-600 dark:text-neutral-400 hover:text-neutral-900 dark:hover:text-white hover:border-orange-500/50 disabled:opacity-50"
            >
              {exporting ? t("stack.exporting") : t("stack.exportImage")}
            </button>
            {user && (
              <button
                onClick={handleToggleBookmark}
                className="inline-block border border-neutral-200 dark:border-neutral-800 px-3 py-1 text-xs text-neutral-600 dark:text-neutral-400 hover:text-neutral-900 dark:hover:text-white hover:border-orange-500/50"
              >
                {bookmarked ? t("stack.bookmarked") : t("stack.bookmark")}
              </button>
            )}
            {(!user || user.user_id !== stack.creator_id) && (
              <button
                onClick={() => router.push(`/new?fork=${stack.id}`)}
                className="inline-block border border-neutral-200 dark:border-neutral-800 px-3 py-1 text-xs text-neutral-600 dark:text-neutral-400 hover:text-neutral-900 dark:hover:text-white hover:border-orange-500/50"
              >
                {t("stack.forkStack")}
              </button>
            )}
            {user && user.user_id === stack.creator_id && (
              <>
                <LocaleLink
                  href={`/stacks/${stack.id}/edit`}
                  className="inline-block border border-neutral-200 dark:border-neutral-800 px-3 py-1 text-xs text-neutral-600 dark:text-neutral-400 hover:text-neutral-900 dark:hover:text-white hover:border-orange-500/50"
                >
                  {t("stack.editStack")}
                </LocaleLink>
                {!showDeleteConfirm ? (
                  <button
                    onClick={() => setShowDeleteConfirm(true)}
                    className="inline-block border border-neutral-200 dark:border-neutral-800 px-3 py-1 text-xs text-red-400 hover:text-red-500 hover:border-red-500/50"
                  >
                    {t("stack.delete")}
                  </button>
                ) : (
                  <div className="flex items-center gap-2">
                    <span className="text-xs text-red-400">{t("stack.deleteConfirm")}</span>
                    <button
                      onClick={handleDelete}
                      disabled={deleting}
                      className="bg-red-500 px-3 py-1 text-xs font-bold text-white hover:bg-red-600 disabled:opacity-50"
                    >
                      {deleting ? "..." : t("stack.yesDelete")}
                    </button>
                    <button
                      onClick={() => setShowDeleteConfirm(false)}
                      className="border border-neutral-200 dark:border-neutral-800 px-3 py-1 text-xs text-neutral-600 dark:text-neutral-400 hover:text-neutral-900 dark:hover:text-white"
                    >
                      {t("common.cancel")}
                    </button>
                  </div>
                )}
              </>
            )}
          </div>
        </div>
      </div>

      {/* Tools */}
      <section className="mt-8">
        <h2 className="mb-4 text-sm font-bold uppercase tracking-wider text-neutral-500">
          {t("stack.tools")} ({stack.tools.length})
        </h2>
        <div className="border border-neutral-200 dark:border-neutral-800">
          {stack.tools.map((tool, i) => (
            <div
              key={tool.id}
              className={`flex flex-col gap-2 p-4 ${
                i < stack.tools.length - 1 ? "border-b border-neutral-200 dark:border-neutral-800" : ""
              }`}
            >
              <div className="flex items-center gap-2 sm:gap-3 flex-wrap">
                <LocaleLink
                  href={`/tools/${encodeURIComponent(tool.name)}`}
                  className="text-sm sm:text-base font-bold text-neutral-900 dark:text-white hover:text-orange-500"
                >
                  {tool.name}
                </LocaleLink>
                <span className="text-xs text-neutral-500">{tool.category}</span>
                <span
                  className={`border px-1.5 sm:px-2 py-0.5 text-xs ${VERDICT_COLORS[tool.verdict] || "text-neutral-600 dark:text-neutral-400"}`}
                >
                  {tool.verdict}
                </span>
                {tool.cost && (
                  <span className="text-xs text-neutral-500">{tool.cost}</span>
                )}
              </div>
              <p className="text-xs sm:text-sm text-neutral-600 dark:text-neutral-400">{tool.why}</p>
            </div>
          ))}
        </div>
      </section>

      {/* Cost breakdown */}
      <CostSummary tools={stack.tools} />

      {/* Lessons */}
      {stack.lessons && (
        <section className="mt-8">
          <h2 className="mb-4 text-sm font-bold uppercase tracking-wider text-neutral-500">
            {t("stack.lessonsLearned")}
          </h2>
          <div className="border border-neutral-200 dark:border-neutral-800 p-4 text-sm text-neutral-700 dark:text-neutral-300">
            {stack.lessons}
          </div>
        </section>
      )}

      {/* Related tools */}
      <section className="mt-8">
        <h2 className="mb-3 text-sm font-bold uppercase tracking-wider text-neutral-500">
          {t("stack.exploreTools")}
        </h2>
        <div className="flex flex-wrap gap-2">
          {stack.tools.map((tool) => (
            <LocaleLink
              key={tool.id}
              href={`/tools/${encodeURIComponent(tool.name)}`}
              className="border border-neutral-200 dark:border-neutral-800 bg-neutral-50 dark:bg-neutral-900 px-3 py-1 text-xs text-neutral-600 dark:text-neutral-400 hover:text-neutral-900 dark:hover:text-white hover:border-orange-500/50"
            >
              {t("stack.whoUses")} {tool.name}?
            </LocaleLink>
          ))}
        </div>
      </section>

      {/* Comments */}
      <section className="mt-10">
        <h2 className="mb-4 text-sm font-bold uppercase tracking-wider text-neutral-500">
          {t("stack.comments")} ({comments.length})
        </h2>

        {/* Comment input */}
        <div className="mb-6">
          <textarea
            value={commentText}
            onChange={(e) => setCommentText(e.target.value)}
            placeholder={user ? t("stack.writePlaceholder") : t("stack.pickNickname")}
            maxLength={500}
            rows={3}
            className="w-full border border-neutral-200 dark:border-neutral-800 bg-neutral-50 dark:bg-neutral-900 px-3 py-2 text-sm text-neutral-900 dark:text-white placeholder-neutral-400 dark:placeholder-neutral-600 outline-none focus:border-orange-500 resize-none"
          />
          <div className="mt-2 flex items-center justify-between">
            <span className="text-xs text-neutral-500">{commentText.length}/500</span>
            <button
              onClick={handleComment}
              disabled={!commentText.trim() || posting}
              className="bg-orange-500 px-4 py-1.5 text-sm font-bold text-black hover:bg-orange-600 dark:hover:bg-orange-400 disabled:opacity-50"
            >
              {posting ? "..." : t("stack.post")}
            </button>
          </div>
        </div>

        {/* Comment list */}
        {comments.length === 0 ? (
          <p className="text-sm text-neutral-500">{t("stack.noComments")}</p>
        ) : (
          <div>
            {comments.map((comment) => (
              <div key={comment.id} className="border-b border-neutral-200 dark:border-neutral-800 py-3">
                <div className="flex items-center gap-2 text-xs text-neutral-500 mb-1">
                  <PixelAvatar nickname={comment.creator_nickname} size={16} />
                  <span>{comment.creator_nickname} &middot; {relativeTime(comment.created_at)}</span>
                </div>
                <p className="text-sm text-neutral-700 dark:text-neutral-300">{comment.content}</p>
              </div>
            ))}
          </div>
        )}
      </section>

      {/* History */}
      {stack.history && stack.history.length > 0 && (
        <section className="mt-10">
          <h2 className="mb-4 text-sm font-bold uppercase tracking-wider text-neutral-500">
            {t("stack.changeHistory")}
          </h2>
          <div className="border border-neutral-200 dark:border-neutral-800">
            {stack.history.map((entry) => (
              <div key={entry.id} className="border-b border-neutral-200 dark:border-neutral-800 last:border-b-0 px-4 py-2">
                <div className="flex items-center justify-between">
                  <span className="text-xs text-neutral-600 dark:text-neutral-400">{entry.detail || entry.change_type}</span>
                  <span className="text-xs text-neutral-500">{relativeTime(entry.created_at)}</span>
                </div>
              </div>
            ))}
          </div>
        </section>
      )}

      <div className="mt-16">
        <Footer />
      </div>
    </div>
  );
}
