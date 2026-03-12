"use client";

import { LocaleLink } from "@/components/locale-link";
import { useT } from "@/lib/locale-context";
import type { StackSummary } from "@/lib/types";
import { SCALE_LABELS } from "@/lib/types";
import { relativeTime } from "@/lib/utils";
import { VoteButton } from "./vote-button";
import { PixelAvatar } from "./pixel-avatar";

export function StackCard({ stack }: { stack: StackSummary }) {
  const t = useT();
  return (
    <div className="flex border-b border-neutral-200 dark:border-neutral-800 py-4 hover:border-l-2 hover:border-l-orange-500">
      <VoteButton stackId={stack.id} initialUpvotes={stack.upvotes} />
      <div className="flex-1 min-w-0">
        <div className="flex items-baseline gap-2 sm:gap-3 flex-wrap">
          <LocaleLink
            href={`/stacks/${stack.id}`}
            className="text-sm sm:text-base text-neutral-900 dark:text-white font-bold hover:text-orange-500"
          >
            {stack.project_name}
          </LocaleLink>
          <span className="text-xs border border-neutral-200 dark:border-neutral-800 px-1.5 sm:px-2 py-0.5 text-neutral-500">
            {stack.category}
          </span>
          <span className="text-xs border border-neutral-200 dark:border-neutral-800 px-1.5 sm:px-2 py-0.5 text-neutral-500">
            {SCALE_LABELS[stack.scale] || stack.scale}
          </span>
        </div>
        <p className="mt-1 text-xs sm:text-sm text-neutral-600 dark:text-neutral-400 line-clamp-2 sm:truncate">{stack.description}</p>
        <div className="mt-2 flex items-center gap-2 text-xs text-neutral-500">
          <PixelAvatar nickname={stack.creator_nickname} size={14} />
          <span className="truncate">
            <LocaleLink href={`/u/${stack.creator_nickname}`} className="hover:text-orange-500">{stack.creator_nickname}</LocaleLink> &middot; {relativeTime(stack.created_at)}{" "}
            {stack.updated_at && <>&middot; updated {relativeTime(stack.updated_at)} </>}
            &middot; {stack.comment_count} comment{stack.comment_count !== 1 ? "s" : ""} &middot;{" "}
            {stack.tool_count} tool{stack.tool_count !== 1 ? "s" : ""}
          </span>
        </div>
      </div>
    </div>
  );
}

export function StackCardSkeleton() {
  return (
    <div className="flex border-b border-neutral-200 dark:border-neutral-800 py-4 animate-pulse">
      <div className="mr-4 flex flex-col items-center gap-1 min-w-[40px]">
        <div className="w-4 h-4 bg-neutral-200 dark:bg-neutral-800" />
        <div className="w-6 h-4 bg-neutral-200 dark:bg-neutral-800" />
        <div className="w-4 h-4 bg-neutral-200 dark:bg-neutral-800" />
      </div>
      <div className="flex-1">
        <div className="h-5 w-48 bg-neutral-200 dark:bg-neutral-800 mb-2" />
        <div className="h-4 w-full max-w-md bg-neutral-200 dark:bg-neutral-800 mb-2" />
        <div className="h-3 w-40 bg-neutral-200 dark:bg-neutral-800" />
      </div>
    </div>
  );
}
