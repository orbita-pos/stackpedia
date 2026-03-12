"use client";

import { useState, useEffect, useRef } from "react";
import { vote as apiVote, getMyVote } from "@/lib/api";
import { useAuth } from "@/lib/auth-context";
import { useToast } from "@/lib/toast-context";
import { useT } from "@/lib/locale-context";

export function VoteButton({
  stackId,
  initialUpvotes,
}: {
  stackId: string;
  initialUpvotes: number;
}) {
  const { user, requireAuth } = useAuth();
  const { toast } = useToast();
  const t = useT();
  const [upvotes, setUpvotes] = useState(initialUpvotes);
  const [myVote, setMyVote] = useState<"up" | "down" | null>(null);
  const [voting, setVoting] = useState(false);
  const upRef = useRef<HTMLButtonElement>(null);
  const downRef = useRef<HTMLButtonElement>(null);

  // Load user's existing vote on mount
  useEffect(() => {
    if (!user) {
      setMyVote(null);
      return;
    }
    getMyVote(stackId)
      .then((res) => {
        if (res.direction === "up") setMyVote("up");
        else if (res.direction === "down") setMyVote("down");
        else setMyVote(null);
      })
      .catch(() => {});
  }, [stackId, user]);

  useEffect(() => {
    setUpvotes(initialUpvotes);
  }, [initialUpvotes]);

  const pop = (ref: React.RefObject<HTMLButtonElement | null>) => {
    const el = ref.current;
    if (!el) return;
    el.classList.remove("vote-pop");
    void el.offsetWidth;
    el.classList.add("vote-pop");
  };

  const handleVote = (direction: "up" | "down") => {
    requireAuth(async () => {
      if (voting) return;

      if (myVote === direction) {
        toast(t("vote.already"), "info");
        return;
      }

      setVoting(true);
      pop(direction === "up" ? upRef : downRef);

      try {
        const res = await apiVote(stackId, direction);
        setUpvotes(res.upvotes);

        if (res.action === "removed") {
          setMyVote(null);
          toast(t("vote.removed"), "info");
        } else {
          setMyVote(direction);
          toast(direction === "up" ? t("vote.upvoted") : t("vote.downvoted"), "success");
        }
      } catch {
        toast(t("vote.error"), "error");
      } finally {
        setVoting(false);
      }
    });
  };

  return (
    <div className="flex flex-col items-center gap-1 mr-4 min-w-[40px]">
      <button
        ref={upRef}
        onClick={() => handleVote("up")}
        disabled={voting}
        className={`text-lg leading-none transition-colors disabled:opacity-50 ${
          myVote === "up"
            ? "text-orange-500"
            : "text-neutral-400 dark:text-neutral-500 hover:text-orange-500"
        }`}
        aria-label="upvote"
      >
        ▲
      </button>
      <span className={`text-sm font-bold ${
        myVote === "up"
          ? "text-orange-500"
          : myVote === "down"
            ? "text-red-400"
            : "text-neutral-900 dark:text-white"
      }`}>
        {upvotes}
      </span>
      <button
        ref={downRef}
        onClick={() => handleVote("down")}
        disabled={voting}
        className={`text-lg leading-none transition-colors disabled:opacity-50 ${
          myVote === "down"
            ? "text-red-400"
            : "text-neutral-400 dark:text-neutral-500 hover:text-red-400"
        }`}
        aria-label="downvote"
      >
        ▼
      </button>
    </div>
  );
}
