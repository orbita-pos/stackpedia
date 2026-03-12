"use client";

import { createContext, useContext, useState, useEffect, useCallback, type ReactNode } from "react";
import { getMe, join as apiJoin, recover as apiRecover } from "./api";
import type { User } from "./types";

interface AuthState {
  user: User | null;
  loading: boolean;
  recoveryCode: string | null;
  showAuthModal: boolean;
  login: (nickname: string) => Promise<void>;
  recoverAccount: (code: string) => Promise<void>;
  logout: () => void;
  clearRecoveryCode: () => void;
  openAuthModal: () => void;
  closeAuthModal: () => void;
  requireAuth: (cb: () => void) => void;
  getSavedRecoveryCode: () => string | null;
}

const AuthContext = createContext<AuthState | null>(null);

const LS_USER_KEY = "stackpedia_user";
const LS_RECOVERY_KEY = "stackpedia_recovery_code";

export function AuthProvider({ children }: { children: ReactNode }) {
  const [user, setUser] = useState<User | null>(null);
  const [loading, setLoading] = useState(true);
  const [recoveryCode, setRecoveryCode] = useState<string | null>(null);
  const [showAuthModal, setShowAuthModal] = useState(false);
  const [pendingAction, setPendingAction] = useState<(() => void) | null>(null);

  useEffect(() => {
    getMe()
      .then((u) => {
        setUser(u);
        // Sync to localStorage
        localStorage.setItem(LS_USER_KEY, JSON.stringify(u));
      })
      .catch(() => {
        // Cookie expired — try localStorage backup
        const saved = localStorage.getItem(LS_USER_KEY);
        if (saved) {
          try {
            const parsed = JSON.parse(saved);
            // We have the user info cached but no valid cookie
            // Try to recover using saved recovery code
            const savedCode = localStorage.getItem(LS_RECOVERY_KEY);
            if (savedCode) {
              apiRecover(savedCode)
                .then((u) => {
                  setUser(u);
                  localStorage.setItem(LS_USER_KEY, JSON.stringify(u));
                })
                .catch(() => {
                  // Recovery failed, clear stale data
                  localStorage.removeItem(LS_USER_KEY);
                  localStorage.removeItem(LS_RECOVERY_KEY);
                });
            }
          } catch {
            localStorage.removeItem(LS_USER_KEY);
          }
        }
      })
      .finally(() => setLoading(false));
  }, []);

  const login = useCallback(async (nickname: string) => {
    const res = await apiJoin(nickname);
    const u = { user_id: res.user_id, nickname: res.nickname };
    setUser(u);
    setRecoveryCode(res.recovery_code);
    // Save to localStorage
    localStorage.setItem(LS_USER_KEY, JSON.stringify(u));
    localStorage.setItem(LS_RECOVERY_KEY, res.recovery_code);
  }, []);

  const recoverAccount = useCallback(async (code: string) => {
    const u = await apiRecover(code);
    setUser(u);
    localStorage.setItem(LS_USER_KEY, JSON.stringify(u));
    localStorage.setItem(LS_RECOVERY_KEY, code);
  }, []);

  const logout = useCallback(() => {
    setUser(null);
    setRecoveryCode(null);
    localStorage.removeItem(LS_USER_KEY);
    localStorage.removeItem(LS_RECOVERY_KEY);
  }, []);

  const clearRecoveryCode = useCallback(() => {
    setRecoveryCode(null);
    setShowAuthModal(false);
    if (pendingAction) {
      pendingAction();
      setPendingAction(null);
    }
  }, [pendingAction]);

  const openAuthModal = useCallback(() => setShowAuthModal(true), []);
  const closeAuthModal = useCallback(() => {
    setShowAuthModal(false);
    setPendingAction(null);
  }, []);

  const requireAuth = useCallback(
    (cb: () => void) => {
      if (user) {
        cb();
      } else {
        setPendingAction(() => cb);
        setShowAuthModal(true);
      }
    },
    [user]
  );

  const getSavedRecoveryCode = useCallback(() => {
    return localStorage.getItem(LS_RECOVERY_KEY);
  }, []);

  return (
    <AuthContext.Provider
      value={{
        user,
        loading,
        recoveryCode,
        showAuthModal,
        login,
        recoverAccount,
        logout,
        clearRecoveryCode,
        openAuthModal,
        closeAuthModal,
        requireAuth,
        getSavedRecoveryCode,
      }}
    >
      {children}
    </AuthContext.Provider>
  );
}

export function useAuth() {
  const ctx = useContext(AuthContext);
  if (!ctx) throw new Error("useAuth must be used within AuthProvider");
  return ctx;
}
