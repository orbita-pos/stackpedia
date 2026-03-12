import type { Metadata } from "next";
import { dictionaries, defaultLocale, type Locale } from "@/lib/i18n";

export async function generateMetadata({ params }: { params: Promise<{ locale: string }> }): Promise<Metadata> {
  const { locale } = await params;
  const dict = dictionaries[(locale as Locale) || defaultLocale] ?? dictionaries[defaultLocale];
  const title = dict["meta.compare.title"];
  const description = dict["meta.compare.desc"];
  return {
    title,
    description,
    openGraph: { title, description },
  };
}

export default function CompareLayout({ children }: { children: React.ReactNode }) {
  return <>{children}</>;
}
