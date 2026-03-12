"use client";

import { useState, useRef, useEffect } from "react";
import Link from "next/link";
import { usePathname } from "next/navigation";
import { LocaleLink } from "@/components/locale-link";
import { useAuth } from "@/lib/auth-context";
import { useTheme } from "@/lib/theme-context";
import { useT, useLocale } from "@/lib/locale-context";
import { PixelAvatar } from "@/components/pixel-avatar";

function ThemeToggle() {
  const { theme, toggle } = useTheme();
  return (
    <button
      onClick={toggle}
      className="text-neutral-500 dark:text-neutral-400 hover:text-neutral-900 dark:hover:text-white p-1"
      aria-label="toggle theme"
    >
      {theme === "dark" ? (
        <svg width="16" height="16" viewBox="0 0 16 16" fill="none" stroke="currentColor" strokeWidth="1.5">
          <circle cx="8" cy="8" r="3.5" />
          <line x1="8" y1="1" x2="8" y2="3" />
          <line x1="8" y1="13" x2="8" y2="15" />
          <line x1="1" y1="8" x2="3" y2="8" />
          <line x1="13" y1="8" x2="15" y2="8" />
          <line x1="3.05" y1="3.05" x2="4.46" y2="4.46" />
          <line x1="11.54" y1="11.54" x2="12.95" y2="12.95" />
          <line x1="3.05" y1="12.95" x2="4.46" y2="11.54" />
          <line x1="11.54" y1="4.46" x2="12.95" y2="3.05" />
        </svg>
      ) : (
        <svg width="16" height="16" viewBox="0 0 16 16" fill="none" stroke="currentColor" strokeWidth="1.5">
          <path d="M13.5 9.5a5.5 5.5 0 0 1-7-7 5.5 5.5 0 1 0 7 7z" />
        </svg>
      )}
    </button>
  );
}

function LocaleSwitcher() {
  const locale = useLocale();
  const pathname = usePathname();
  const rest = pathname.replace(/^\/(en|es)/, "");

  return (
    <span className="flex items-center gap-1">
      <Link
        href={`/en${rest}`}
        className={locale === "en" ? "text-xs text-orange-500" : "text-xs text-neutral-500 hover:text-neutral-900 dark:hover:text-white"}
      >
        EN
      </Link>
      <span className="text-xs text-neutral-400">|</span>
      <Link
        href={`/es${rest}`}
        className={locale === "es" ? "text-xs text-orange-500" : "text-xs text-neutral-500 hover:text-neutral-900 dark:hover:text-white"}
      >
        ES
      </Link>
    </span>
  );
}

function NavLink({ href, children, onClick }: { href: string; children: React.ReactNode; onClick?: () => void }) {
  const active = useIsActive(href);
  return (
    <LocaleLink
      href={href}
      onClick={onClick}
      className={active
        ? "text-orange-500 font-bold"
        : "text-neutral-500 dark:text-neutral-400 hover:text-neutral-900 dark:hover:text-white"
      }
    >
      {children}
    </LocaleLink>
  );
}

function useIsActive(path: string) {
  const pathname = usePathname();
  const locale = useLocale();
  const localized = `/${locale}${path}`;
  return pathname === localized || pathname.startsWith(localized + "/");
}

export function Header() {
  const { user, openAuthModal, logout, getSavedRecoveryCode } = useAuth();
  const t = useT();
  const [menuOpen, setMenuOpen] = useState(false);
  const [profileOpen, setProfileOpen] = useState(false);
  const [showCode, setShowCode] = useState(false);
  const [copied, setCopied] = useState(false);
  const dropdownRef = useRef<HTMLDivElement>(null);

  useEffect(() => {
    const handler = (e: MouseEvent) => {
      if (dropdownRef.current && !dropdownRef.current.contains(e.target as Node)) {
        setProfileOpen(false);
        setShowCode(false);
      }
    };
    document.addEventListener("mousedown", handler);
    return () => document.removeEventListener("mousedown", handler);
  }, []);

  const savedCode = showCode ? getSavedRecoveryCode() : null;

  const copyCode = async () => {
    const code = getSavedRecoveryCode();
    if (!code) return;
    try {
      await navigator.clipboard.writeText(code);
      setCopied(true);
      setTimeout(() => setCopied(false), 2000);
    } catch {}
  };

  return (
    <header className="fixed top-0 z-40 w-full border-b border-neutral-200 dark:border-neutral-800 bg-white dark:bg-[#0a0a0a]">
      <div className="mx-auto flex h-12 max-w-5xl items-center justify-between px-4">
        <LocaleLink href="/" className="flex items-center gap-2 font-mono text-lg font-bold text-neutral-900 dark:text-white hover:text-orange-500 dark:hover:text-orange-400">
          <img src="/logo-96x96.png" alt="" width={24} height={24} className="inline-block" />
          stackpedia<span className="text-orange-500">.</span>
        </LocaleLink>

        {/* Desktop nav */}
        <nav className="hidden items-center gap-5 text-sm sm:flex">
          <NavLink href="/stacks">{t("nav.browse")}</NavLink>
          <NavLink href="/tools">{t("nav.tools")}</NavLink>
          <NavLink href="/compare">{t("nav.compare")}</NavLink>
          {user && <NavLink href="/bookmarks">{t("nav.bookmarks")}</NavLink>}
          <LocaleSwitcher />
          <ThemeToggle />
          <LocaleLink
            href="/new"
            className="bg-orange-500 px-3 py-1 text-white text-sm font-bold hover:bg-orange-600 dark:hover:bg-orange-400"
          >
            {t("nav.share")}
          </LocaleLink>

          {user ? (
            <div className="relative" ref={dropdownRef}>
              <button
                onClick={() => setProfileOpen((p) => !p)}
                className="flex items-center gap-2 text-neutral-500 dark:text-neutral-400 hover:text-neutral-900 dark:hover:text-white"
              >
                <PixelAvatar nickname={user.nickname} size={20} />
                {user.nickname}
              </button>

              {profileOpen && (
                <div className="absolute right-0 top-full mt-2 w-64 border border-neutral-200 dark:border-neutral-800 bg-white dark:bg-[#0a0a0a] shadow-lg">
                  <div className="border-b border-neutral-200 dark:border-neutral-800 px-4 py-3 flex items-center gap-3">
                    <PixelAvatar nickname={user.nickname} size={32} />
                    <div>
                      <p className="text-sm text-neutral-900 dark:text-white font-bold">{user.nickname}</p>
                      <p className="text-xs text-neutral-500">{t("nav.account.anonymous")}</p>
                    </div>
                  </div>

                  <LocaleLink
                    href={`/u/${user.nickname}`}
                    onClick={() => setProfileOpen(false)}
                    className="block px-4 py-3 text-sm text-neutral-500 dark:text-neutral-400 hover:text-neutral-900 dark:hover:text-white hover:bg-neutral-50 dark:hover:bg-neutral-900 border-b border-neutral-200 dark:border-neutral-800"
                  >
                    {t("nav.profile")}
                  </LocaleLink>

                  <div className="px-4 py-3 border-b border-neutral-200 dark:border-neutral-800">
                    <button
                      onClick={() => setShowCode((p) => !p)}
                      className="w-full text-left text-sm text-neutral-500 dark:text-neutral-400 hover:text-neutral-900 dark:hover:text-white"
                    >
                      {showCode ? t("nav.account.hideCode") : t("nav.account.showCode")}
                    </button>
                    {showCode && savedCode && (
                      <div className="mt-2 flex items-center gap-2">
                        <code className="flex-1 border border-orange-300 dark:border-orange-500/20 bg-orange-50 dark:bg-orange-500/5 px-2 py-1 text-xs text-orange-600 dark:text-orange-400 font-mono truncate">
                          {savedCode}
                        </code>
                        <button
                          onClick={copyCode}
                          className="text-xs text-orange-500/60 hover:text-orange-500 border border-orange-300 dark:border-orange-500/20 px-2 py-1"
                        >
                          {copied ? t("common.copied") : t("common.copy")}
                        </button>
                      </div>
                    )}
                    {showCode && !savedCode && (
                      <p className="mt-2 text-xs text-neutral-500">
                        {t("nav.account.codeNotSaved")}
                      </p>
                    )}
                  </div>

                  <button
                    onClick={() => {
                      logout();
                      setProfileOpen(false);
                    }}
                    className="w-full px-4 py-3 text-left text-sm text-red-500 hover:text-red-600 dark:hover:text-red-400 hover:bg-neutral-50 dark:hover:bg-neutral-900"
                  >
                    {t("nav.logout")}
                  </button>
                </div>
              )}
            </div>
          ) : (
            <button
              onClick={openAuthModal}
              className="text-neutral-500 dark:text-neutral-400 hover:text-neutral-900 dark:hover:text-white cursor-pointer"
            >
              {t("nav.join")}
            </button>
          )}
        </nav>

        {/* Mobile: theme toggle + hamburger */}
        <div className="flex items-center gap-3 sm:hidden">
          <ThemeToggle />
          <button
            onClick={() => setMenuOpen((p) => !p)}
            className="flex flex-col gap-1"
            aria-label="menu"
          >
            <span className={`block h-0.5 w-5 bg-neutral-500 dark:bg-neutral-400 transition-transform ${menuOpen ? "translate-y-1.5 rotate-45" : ""}`} />
            <span className={`block h-0.5 w-5 bg-neutral-500 dark:bg-neutral-400 transition-opacity ${menuOpen ? "opacity-0" : ""}`} />
            <span className={`block h-0.5 w-5 bg-neutral-500 dark:bg-neutral-400 transition-transform ${menuOpen ? "-translate-y-1.5 -rotate-45" : ""}`} />
          </button>
        </div>
      </div>

      {/* Mobile dropdown */}
      {menuOpen && (
        <nav className="flex flex-col border-t border-neutral-200 dark:border-neutral-800 bg-white dark:bg-[#0a0a0a] px-4 py-3 text-sm sm:hidden">
          <div className="py-2"><NavLink href="/stacks" onClick={() => setMenuOpen(false)}>{t("nav.browse")}</NavLink></div>
          <div className="py-2"><NavLink href="/tools" onClick={() => setMenuOpen(false)}>{t("nav.tools")}</NavLink></div>
          <div className="py-2"><NavLink href="/compare" onClick={() => setMenuOpen(false)}>{t("nav.compare")}</NavLink></div>
          {user && <div className="py-2"><NavLink href="/bookmarks" onClick={() => setMenuOpen(false)}>{t("nav.bookmarks")}</NavLink></div>}
          <LocaleLink href="/new" onClick={() => setMenuOpen(false)} className="py-2 text-orange-500 hover:text-orange-600 dark:hover:text-orange-400">
            {t("nav.share")}
          </LocaleLink>
          <div className="py-2">
            <LocaleSwitcher />
          </div>
          {user ? (
            <>
              <div className="border-t border-neutral-200 dark:border-neutral-800 mt-2 pt-2">
                <div className="py-2 flex items-center gap-2">
                  <PixelAvatar nickname={user.nickname} size={20} />
                  <span className="text-neutral-900 dark:text-white font-bold">{user.nickname}</span>
                </div>
                <LocaleLink
                  href={`/u/${user.nickname}`}
                  onClick={() => setMenuOpen(false)}
                  className="py-2 text-neutral-500 dark:text-neutral-400 hover:text-neutral-900 dark:hover:text-white block"
                >
                  {t("nav.profile")}
                </LocaleLink>
                <button
                  onClick={() => setShowCode((p) => !p)}
                  className="py-2 text-neutral-500 dark:text-neutral-400 hover:text-neutral-900 dark:hover:text-white w-full text-left"
                >
                  {showCode ? t("nav.account.hideCode") : t("nav.account.showCode")}
                </button>
                {showCode && (
                  <div className="py-2">
                    {getSavedRecoveryCode() ? (
                      <div className="flex items-center gap-2">
                        <code className="flex-1 border border-orange-300 dark:border-orange-500/20 bg-orange-50 dark:bg-orange-500/5 px-2 py-1 text-xs text-orange-600 dark:text-orange-400 font-mono truncate">
                          {getSavedRecoveryCode()}
                        </code>
                        <button
                          onClick={copyCode}
                          className="text-xs text-orange-500/60 hover:text-orange-500 border border-orange-300 dark:border-orange-500/20 px-2 py-1"
                        >
                          {copied ? t("common.copied") : t("common.copy")}
                        </button>
                      </div>
                    ) : (
                      <p className="text-xs text-neutral-500">{t("nav.account.codeNotSaved")}</p>
                    )}
                  </div>
                )}
                <button
                  onClick={() => {
                    logout();
                    setMenuOpen(false);
                  }}
                  className="py-2 text-red-500 hover:text-red-600 dark:hover:text-red-400 w-full text-left"
                >
                  {t("nav.logout")}
                </button>
              </div>
            </>
          ) : (
            <button
              onClick={() => {
                openAuthModal();
                setMenuOpen(false);
              }}
              className="py-2 text-neutral-500 dark:text-neutral-400 hover:text-neutral-900 dark:hover:text-white text-left"
            >
              {t("nav.join")}
            </button>
          )}
        </nav>
      )}
    </header>
  );
}
