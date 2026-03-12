"use client";

import type { ReactNode } from "react";
import { ThemeProvider } from "@/lib/theme-context";
import { AuthProvider } from "@/lib/auth-context";
import { ToastProvider } from "@/lib/toast-context";
import { Header } from "@/components/header";
import { AuthModal } from "@/components/auth-modal";
import { KeyboardShortcuts } from "@/components/keyboard-shortcuts";

export function Providers({ children }: { children: ReactNode }) {
  return (
    <ThemeProvider>
      <AuthProvider>
        <ToastProvider>
          <Header />
          <AuthModal />
          <KeyboardShortcuts />
          <main className="pt-12">{children}</main>
        </ToastProvider>
      </AuthProvider>
    </ThemeProvider>
  );
}
