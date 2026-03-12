"use client";

import Link from "next/link";
import { useLocale } from "@/lib/locale-context";
import type { ComponentProps } from "react";

type LocaleLinkProps = ComponentProps<typeof Link>;

export function LocaleLink({ href, ...props }: LocaleLinkProps) {
  const locale = useLocale();
  const hrefStr = typeof href === "string" ? href : href.pathname || "";
  const localizedHref = hrefStr.startsWith("/") ? `/${locale}${hrefStr}` : hrefStr;
  return <Link href={localizedHref} {...props} />;
}
