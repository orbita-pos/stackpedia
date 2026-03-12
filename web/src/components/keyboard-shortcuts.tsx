"use client";

import { useEffect, useState, useCallback } from "react";
import { useRouter } from "next/navigation";
import { useAuth } from "@/lib/auth-context";

export function KeyboardShortcuts() {
  const router = useRouter();
  const { closeAuthModal, showAuthModal } = useAuth();
  const [showHelp, setShowHelp] = useState(false);

  const handler = useCallback(
    (e: KeyboardEvent) => {
      const tag = (e.target as HTMLElement).tagName;
      const isInput = tag === "INPUT" || tag === "TEXTAREA" || tag === "SELECT";

      if (e.key === "Escape") {
        if (showAuthModal) closeAuthModal();
        setShowHelp(false);
        return;
      }

      if (isInput) return;

      if (e.key === "/") {
        e.preventDefault();
        const searchInput = document.querySelector<HTMLInputElement>(
          'input[placeholder*="search"]'
        );
        searchInput?.focus();
      } else if (e.key === "n") {
        router.push("/new");
      } else if (e.key === "?") {
        setShowHelp((p) => !p);
      }
    },
    [router, closeAuthModal, showAuthModal]
  );

  useEffect(() => {
    window.addEventListener("keydown", handler);
    return () => window.removeEventListener("keydown", handler);
  }, [handler]);

  return (
    <>
      <button
        onClick={() => setShowHelp((p) => !p)}
        className="fixed bottom-4 right-4 z-30 flex h-7 w-7 items-center justify-center border border-neutral-200 dark:border-neutral-800 bg-neutral-100 dark:bg-neutral-900 text-xs text-neutral-500 dark:text-neutral-400 hover:text-neutral-900 dark:hover:text-white hover:border-orange-500/50"
        aria-label="keyboard shortcuts"
      >
        ?
      </button>
      {showHelp && (
        <div className="fixed bottom-14 right-4 z-30 border border-neutral-200 dark:border-neutral-800 bg-white dark:bg-[#0a0a0a] p-4 text-xs">
          <p className="mb-2 font-bold text-neutral-500 dark:text-neutral-400">keyboard shortcuts</p>
          <div className="space-y-1 text-neutral-500 dark:text-neutral-400">
            <p>
              <kbd className="mr-2 border border-neutral-200 dark:border-neutral-800 bg-neutral-100 dark:bg-neutral-900 px-1.5 py-0.5 text-neutral-900 dark:text-neutral-300">/</kbd>
              focus search
            </p>
            <p>
              <kbd className="mr-2 border border-neutral-200 dark:border-neutral-800 bg-neutral-100 dark:bg-neutral-900 px-1.5 py-0.5 text-neutral-900 dark:text-neutral-300">n</kbd>
              new stack
            </p>
            <p>
              <kbd className="mr-2 border border-neutral-200 dark:border-neutral-800 bg-neutral-100 dark:bg-neutral-900 px-1.5 py-0.5 text-neutral-900 dark:text-neutral-300">esc</kbd>
              close modal
            </p>
          </div>
        </div>
      )}
    </>
  );
}
