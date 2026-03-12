import type { Metadata } from "next";
import { notFound } from "next/navigation";
import { locales, defaultLocale, dictionaries, type Locale } from "@/lib/i18n";
import { LocaleProvider } from "@/lib/locale-context";
import { Providers } from "../providers";

export async function generateMetadata({ params }: { params: Promise<{ locale: string }> }): Promise<Metadata> {
  const { locale } = await params;
  const t = dictionaries[(locale as Locale) || defaultLocale] || dictionaries[defaultLocale];
  return {
    title: t["meta.home.title"],
    description: t["meta.home.desc"],
    openGraph: {
      title: t["meta.home.title"],
      description: t["meta.home.ogDesc"],
      type: "website",
    },
  };
}

export default async function LocaleLayout({
  children,
  params,
}: {
  children: React.ReactNode;
  params: Promise<{ locale: string }>;
}) {
  const { locale } = await params;

  if (!locales.includes(locale as Locale)) {
    notFound();
  }

  return (
    <LocaleProvider locale={locale as Locale}>
      <Providers>{children}</Providers>
    </LocaleProvider>
  );
}
