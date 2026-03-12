"use client";

import { useState } from "react";
import { useAuth } from "@/lib/auth-context";
import { useT } from "@/lib/locale-context";

export function AuthModal() {
  const { showAuthModal, recoveryCode, login, recoverAccount, clearRecoveryCode, closeAuthModal } = useAuth();
  const t = useT();
  const [mode, setMode] = useState<"join" | "recover">("join");
  const [nickname, setNickname] = useState("");
  const [code, setCode] = useState("");
  const [error, setError] = useState("");
  const [submitting, setSubmitting] = useState(false);
  const [copied, setCopied] = useState(false);

  if (!showAuthModal) return null;

  const copyCode = async () => {
    if (!recoveryCode) return;
    try {
      await navigator.clipboard.writeText(recoveryCode);
      setCopied(true);
      setTimeout(() => setCopied(false), 2000);
    } catch {
      // fallback
    }
  };

  // Recovery code display screen
  if (recoveryCode) {
    return (
      <div className="fixed inset-0 z-50 flex items-center justify-center bg-black/80">
        <div className="w-full max-w-md border border-neutral-200 dark:border-neutral-800 bg-white dark:bg-[#0a0a0a] p-6 mx-4">
          <div className="mb-2 text-center">
            <span className="inline-block h-3 w-3 bg-orange-500 mb-4" />
          </div>
          <h2 className="mb-2 text-lg font-bold text-neutral-900 dark:text-white text-center">{t("auth.welcomeTitle")}</h2>
          <p className="mb-6 text-sm text-neutral-600 dark:text-neutral-400 text-center">
            {t("auth.welcomeDesc")}
          </p>

          <div className="mb-2 relative border border-orange-300 dark:border-orange-500/30 bg-orange-50 dark:bg-orange-500/5 p-5 text-center">
            <code className="font-mono text-xl text-orange-600 dark:text-orange-400 tracking-wider">{recoveryCode}</code>
            <button
              onClick={copyCode}
              className="absolute right-2 top-1/2 -translate-y-1/2 text-xs text-orange-400/60 hover:text-orange-500 border border-orange-300/20 dark:border-orange-500/20 px-2 py-1"
            >
              {copied ? t("common.copied") : t("common.copy")}
            </button>
          </div>

          <p className="mb-6 text-xs text-neutral-500 dark:text-neutral-600 text-center">
            {t("auth.noRecovery")}
          </p>

          <button
            onClick={clearRecoveryCode}
            className="w-full bg-orange-500 px-4 py-2.5 text-sm font-bold text-white hover:bg-orange-600"
          >
            {t("auth.savedIt")}
          </button>
        </div>
      </div>
    );
  }

  const handleJoin = async (e: React.FormEvent) => {
    e.preventDefault();
    const trimmed = nickname.trim();
    if (!trimmed) return;
    if (trimmed.length > 30) {
      setError("nickname must be 1-30 chars");
      return;
    }
    setSubmitting(true);
    setError("");
    try {
      await login(trimmed);
    } catch (err) {
      const msg = err instanceof Error ? err.message : "failed to join";
      setError(msg.includes("nickname already taken") ? t("auth.nicknameTaken") : msg);
    } finally {
      setSubmitting(false);
    }
  };

  const handleRecover = async (e: React.FormEvent) => {
    e.preventDefault();
    const trimmed = code.trim();
    if (!trimmed) return;
    setSubmitting(true);
    setError("");
    try {
      await recoverAccount(trimmed);
      closeAuthModal();
    } catch (err) {
      setError(err instanceof Error ? err.message : "invalid recovery code");
    } finally {
      setSubmitting(false);
    }
  };

  return (
    <div className="fixed inset-0 z-50 flex items-center justify-center bg-black/80">
      <div className="w-full max-w-md border border-neutral-200 dark:border-neutral-800 bg-white dark:bg-[#0a0a0a] p-6 mx-4">
        <div className="mb-4 flex items-center justify-between">
          <h2 className="text-lg font-bold text-neutral-900 dark:text-white">
            {mode === "join" ? t("auth.joinTitle") : t("auth.recoverTitle")}
          </h2>
          <button onClick={closeAuthModal} className="text-neutral-500 hover:text-neutral-900 dark:hover:text-white text-xl leading-none">
            &times;
          </button>
        </div>

        {/* Tabs */}
        <div className="flex mb-5 border border-neutral-200 dark:border-neutral-800">
          <button
            onClick={() => { setMode("join"); setError(""); }}
            className={`flex-1 px-3 py-2 text-sm ${
              mode === "join"
                ? "bg-neutral-50 dark:bg-neutral-900 text-orange-500 border-b-2 border-orange-500"
                : "text-neutral-500 hover:text-neutral-900 dark:hover:text-white"
            }`}
          >
            {t("auth.newHere")}
          </button>
          <button
            onClick={() => { setMode("recover"); setError(""); }}
            className={`flex-1 px-3 py-2 text-sm ${
              mode === "recover"
                ? "bg-neutral-50 dark:bg-neutral-900 text-orange-500 border-b-2 border-orange-500"
                : "text-neutral-500 hover:text-neutral-900 dark:hover:text-white"
            }`}
          >
            {t("auth.haveCode")}
          </button>
        </div>

        {mode === "join" ? (
          <>
            <p className="mb-4 text-sm text-neutral-600 dark:text-neutral-400">
              {t("auth.joinDesc")}
            </p>
            <form onSubmit={handleJoin}>
              <input
                type="text"
                value={nickname}
                onChange={(e) => setNickname(e.target.value)}
                placeholder={t("auth.nickPlaceholder")}
                className="mb-3 w-full border border-neutral-200 dark:border-neutral-800 bg-neutral-50 dark:bg-neutral-900 px-3 py-2 text-sm text-neutral-900 dark:text-white placeholder-neutral-400 dark:placeholder-neutral-600 outline-none focus:border-orange-500"
                autoFocus
                maxLength={30}
              />
              {error && <p className="mb-3 text-sm text-red-400">{error}</p>}
              <button
                type="submit"
                disabled={submitting}
                className="w-full bg-orange-500 px-4 py-2.5 text-sm font-bold text-white hover:bg-orange-600 disabled:opacity-50"
              >
                {submitting ? "..." : t("auth.goAnonymous")}
              </button>
            </form>
            <p className="mt-4 text-xs text-neutral-500 dark:text-neutral-600 text-center">
              {t("auth.codeHint")}
            </p>
          </>
        ) : (
          <>
            <p className="mb-4 text-sm text-neutral-600 dark:text-neutral-400">
              {t("auth.recoverDesc")}
            </p>
            <form onSubmit={handleRecover}>
              <input
                type="text"
                value={code}
                onChange={(e) => setCode(e.target.value)}
                placeholder={t("auth.codePlaceholder")}
                className="mb-3 w-full border border-neutral-200 dark:border-neutral-800 bg-neutral-50 dark:bg-neutral-900 px-3 py-2 text-sm text-neutral-900 dark:text-white placeholder-neutral-400 dark:placeholder-neutral-600 outline-none focus:border-orange-500 font-mono"
                autoFocus
              />
              {error && <p className="mb-3 text-sm text-red-400">{error}</p>}
              <button
                type="submit"
                disabled={submitting}
                className="w-full bg-orange-500 px-4 py-2.5 text-sm font-bold text-white hover:bg-orange-600 disabled:opacity-50"
              >
                {submitting ? "..." : t("auth.recover")}
              </button>
            </form>
          </>
        )}
      </div>
    </div>
  );
}
