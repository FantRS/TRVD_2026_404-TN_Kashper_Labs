"use client";

import {
  createContext,
  useContext,
  useEffect,
  useState,
  type ReactNode,
} from "react";

import { getCurrentUser, login, logout, register } from "@/lib/api";
import {
  clearStoredToken,
  clearStoredUserSnapshot,
  getStoredToken,
  getStoredUserSnapshot,
  setStoredToken,
  setStoredUserSnapshot,
} from "@/lib/storage";
import type { AuthUser, LoginPayload, RegisterPayload } from "@/lib/types";

interface AuthContextValue {
  token: string | null;
  user: AuthUser | null;
  isHydrated: boolean;
  isAuthenticated: boolean;
  signIn: (payload: LoginPayload) => Promise<void>;
  signUp: (payload: RegisterPayload) => Promise<void>;
  signOut: () => Promise<void>;
  refreshUser: () => Promise<AuthUser | null>;
}

const AuthContext = createContext<AuthContextValue | null>(null);

function clearSession() {
  clearStoredToken();
  clearStoredUserSnapshot();
}

export function AuthProvider({ children }: { children: ReactNode }) {
  const [token, setToken] = useState<string | null>(null);
  const [user, setUser] = useState<AuthUser | null>(null);
  const [isHydrated, setIsHydrated] = useState(false);

  useEffect(() => {
    const storedToken = getStoredToken();
    const storedUser = getStoredUserSnapshot<AuthUser>();

    if (!storedToken) {
      setIsHydrated(true);
      return;
    }

    setToken(storedToken);
    if (storedUser) {
      setUser(storedUser);
    }

    void getCurrentUser(storedToken)
      .then((response) => {
        setUser(response.user);
        setStoredUserSnapshot(response.user);
      })
      .catch(() => {
        clearSession();
        setToken(null);
        setUser(null);
      })
      .finally(() => {
        setIsHydrated(true);
      });
  }, []);

  async function refreshUser() {
    if (!token) {
      return null;
    }

    const response = await getCurrentUser(token);
    setUser(response.user);
    setStoredUserSnapshot(response.user);
    return response.user;
  }

  async function signIn(payload: LoginPayload) {
    const response = await login(payload);
    setToken(response.token);
    setUser(response.user);
    setStoredToken(response.token);
    setStoredUserSnapshot(response.user);
  }

  async function signUp(payload: RegisterPayload) {
    const response = await register(payload);
    setToken(response.token);
    setUser(response.user);
    setStoredToken(response.token);
    setStoredUserSnapshot(response.user);
  }

  async function signOut() {
    const currentToken = token;

    clearSession();
    setToken(null);
    setUser(null);

    if (currentToken) {
      try {
        await logout(currentToken);
      } catch {
        // The local session is already cleared, so we can silently ignore API logout errors.
      }
    }
  }

  return (
    <AuthContext.Provider
      value={{
        token,
        user,
        isHydrated,
        isAuthenticated: Boolean(token && user),
        signIn,
        signUp,
        signOut,
        refreshUser,
      }}
    >
      {children}
    </AuthContext.Provider>
  );
}

export function useAuth() {
  const context = useContext(AuthContext);

  if (!context) {
    throw new Error("useAuth must be used within AuthProvider");
  }

  return context;
}
