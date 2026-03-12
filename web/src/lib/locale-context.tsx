"use client";

import { createContext, useContext, type ReactNode } from "react";
import { useRouter } from "next/navigation";
import { getT, type Locale } from "./i18n";

interface LocaleContextValue {
  locale: Locale;
  t: (key: string) => string;
}

const LocaleContext = createContext<LocaleContextValue>({
  locale: "en",
  t: (key) => key,
});

export function LocaleProvider({ locale, children }: { locale: Locale; children: ReactNode }) {
  const t = getT(locale);
  return <LocaleContext.Provider value={{ locale, t }}>{children}</LocaleContext.Provider>;
}

export function useLocale() {
  return useContext(LocaleContext).locale;
}

export function useT() {
  return useContext(LocaleContext).t;
}

/** Hook that wraps next/navigation router to auto-prefix paths with locale */
export function useLocaleRouter() {
  const router = useRouter();
  const locale = useLocale();

  const prefix = (path: string) => (path.startsWith("/") ? `/${locale}${path}` : path);

  return {
    push: (path: string, options?: Parameters<typeof router.push>[1]) =>
      router.push(prefix(path), options),
    replace: (path: string, options?: Parameters<typeof router.replace>[1]) =>
      router.replace(prefix(path), options),
    back: () => router.back(),
    forward: () => router.forward(),
    refresh: () => router.refresh(),
    prefetch: (path: string) => router.prefetch(prefix(path)),
  };
}
