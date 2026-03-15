"use client";

import Link from "next/link";
import { usePathname, useRouter } from "next/navigation";

import { useAuth } from "@/lib/auth";
import { formatCredits, roleLabel } from "@/lib/utils";

const navItems = [
  { href: "/", label: "Головна" },
  { href: "/catalog", label: "Каталог" },
  { href: "/schedule", label: "Розклад" },
  { href: "/account", label: "Кабінет" },
];

export function SiteHeader() {
  const pathname = usePathname();
  const router = useRouter();
  const { user, isAuthenticated, signOut, isHydrated } = useAuth();

  return (
    <header className="site-header">
      <div className="site-shell header-inner">
        <Link href="/" className="brand">
          <span className="brand-mark" aria-hidden="true">
            <svg viewBox="0 0 48 48" className="brand-mark__icon" role="img">
              <defs>
                <linearGradient id="brand-cross-gradient" x1="0%" y1="0%" x2="100%" y2="100%">
                  <stop offset="0%" stopColor="#f8e7a8" />
                  <stop offset="45%" stopColor="#d4af37" />
                  <stop offset="100%" stopColor="#8f6b12" />
                </linearGradient>
              </defs>
              <path
                d="M24 4.5c1.8 0 3.2 1.4 3.2 3.2v10.4h8.8c1.8 0 3.2 1.4 3.2 3.2v5.4c0 1.8-1.4 3.2-3.2 3.2h-8.8v10.1c0 1.8-1.4 3.2-3.2 3.2h-5.4c-1.8 0-3.2-1.4-3.2-3.2V30H6.6c-1.8 0-3.2-1.4-3.2-3.2v-5.4c0-1.8 1.4-3.2 3.2-3.2h8.8V7.7c0-1.8 1.4-3.2 3.2-3.2Z"
                fill="url(#brand-cross-gradient)"
              />
              <path
                d="M24 4.5c1.8 0 3.2 1.4 3.2 3.2v10.4h8.8c1.8 0 3.2 1.4 3.2 3.2v5.4c0 1.8-1.4 3.2-3.2 3.2h-8.8v10.1c0 1.8-1.4 3.2-3.2 3.2h-5.4c-1.8 0-3.2-1.4-3.2-3.2V30H6.6c-1.8 0-3.2-1.4-3.2-3.2v-5.4c0-1.8 1.4-3.2 3.2-3.2h8.8V7.7c0-1.8 1.4-3.2 3.2-3.2Z"
                fill="none"
                stroke="rgba(255, 244, 208, 0.58)"
                strokeWidth="1.2"
              />
            </svg>
          </span>
          <span>
            <strong>CrematoryShop</strong>
            <small>Повага, порядок і нуль недобитих цвяхів</small>
          </span>
        </Link>

        <nav className="nav-links" aria-label="Основна навігація">
          {navItems.map((item) => (
            <Link
              key={item.href}
              href={item.href}
              className={pathname === item.href ? "nav-link active" : "nav-link"}
            >
              {item.label}
            </Link>
          ))}
        </nav>

        <div className="header-actions">
          {isHydrated && isAuthenticated && user ? (
            <>
              {user.role === "user" ? (
                <div className="wallet-chip">{formatCredits(user.wallet_balance)}</div>
              ) : (
                <div className="wallet-chip subtle">{roleLabel(user.role)}</div>
              )}
              <button
                type="button"
                className="button ghost"
                onClick={() => {
                  void signOut().then(() => router.push("/"));
                }}
              >
                Вийти
              </button>
            </>
          ) : (
            <>
              <Link href="/login" className="button ghost">
                Увійти
              </Link>
              <Link href="/register" className="button">
                Створити акаунт
              </Link>
            </>
          )}
        </div>
      </div>
    </header>
  );
}
