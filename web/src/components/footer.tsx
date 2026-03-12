"use client";

import { useT } from "@/lib/locale-context";

export function Footer() {
  const t = useT();
  return (
    <footer className="border-t border-neutral-200 dark:border-neutral-800 py-8 text-center text-xs text-neutral-500 dark:text-neutral-500">
      <p className="font-bold text-neutral-500 flex items-center justify-center gap-2">
        <img src="/logo-96x96.png" alt="" width={20} height={20} className="inline-block" />
        stackpedia<span className="text-orange-500">.</span>
      </p>
      <p className="mt-1">{t("footer.builtWith")}</p>
      <p className="mt-1">{t("footer.tagline")}</p>
    </footer>
  );
}
