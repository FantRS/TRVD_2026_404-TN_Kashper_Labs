"use client";

import Link from "next/link";
import { useRouter } from "next/navigation";
import { useState } from "react";

import { useAuth } from "@/lib/auth";

export function AuthForm({ mode }: { mode: "login" | "register" }) {
  const router = useRouter();
  const { signIn, signUp } = useAuth();
  const [error, setError] = useState<string | null>(null);
  const [submitting, setSubmitting] = useState(false);
  const [form, setForm] = useState({
    email: "",
    password: "",
    full_name: "",
    phone: "",
  });

  const isLogin = mode === "login";

  async function handleSubmit(event: React.FormEvent<HTMLFormElement>) {
    event.preventDefault();
    setSubmitting(true);
    setError(null);

    try {
      if (isLogin) {
        await signIn({
          email: form.email,
          password: form.password,
        });
      } else {
        await signUp({
          email: form.email,
          password: form.password,
          full_name: form.full_name,
          phone: form.phone || undefined,
        });
      }

      router.push("/account");
    } catch (submitError) {
      setError(submitError instanceof Error ? submitError.message : "Не вдалося виконати запит.");
    } finally {
      setSubmitting(false);
    }
  }

  return (
    <section className="auth-shell">
      <div className="auth-copy">
        <p className="eyebrow">{isLogin ? "Повернення до справ" : "Квиток у особистий кабінет"}</p>
        <h1 className="display-sm">
          {isLogin
            ? "Увійдіть, щоб продовжити оформлення без повторного кола бюрократичного пекла."
            : "Створіть акаунт, щоб тримати всі замовлення під контролем, поки життя знову не підкине сюрпризів."}
        </h1>
        <p className="lead">
          У кабінеті можна зберігати контакти, переглядати замовлення, оплачувати покупки та
          обирати зручний час для послуги. Менше повторів, менше хаосу, менше відчуття, що все валиться разом із настроєм.
        </p>
      </div>

      <form className="panel auth-panel" onSubmit={handleSubmit}>
        <div className="panel-header">
          <h2>{isLogin ? "Вхід" : "Реєстрація"}</h2>
          <span className="muted">{isLogin ? "Швидко назад у процес" : "10000 крд на перший крок у невеселий сервіс"}</span>
        </div>

        {!isLogin ? (
          <label className="field">
            <span>ПІБ</span>
            <input
              className="input"
              required
              minLength={3}
              value={form.full_name}
              onChange={(event) => setForm((current) => ({ ...current, full_name: event.target.value }))}
            />
          </label>
        ) : null}

        <label className="field">
          <span>Email</span>
          <input
            className="input"
            type="email"
            required
            value={form.email}
            onChange={(event) => setForm((current) => ({ ...current, email: event.target.value }))}
          />
        </label>

        <label className="field">
          <span>Пароль</span>
          <input
            className="input"
            type="password"
            required
            minLength={8}
            value={form.password}
            onChange={(event) => setForm((current) => ({ ...current, password: event.target.value }))}
          />
        </label>

        {!isLogin ? (
          <label className="field">
            <span>Телефон</span>
            <input
              className="input"
              placeholder="+380..."
              value={form.phone}
              onChange={(event) => setForm((current) => ({ ...current, phone: event.target.value }))}
            />
          </label>
        ) : null}

        {error ? <div className="panel subtle muted">{error}</div> : null}

        <button type="submit" className="button wide" disabled={submitting}>
          {submitting ? "Секунду, підшиваємо чорну папку..." : isLogin ? "Увійти без драми" : "Створити акаунт"}
        </button>

        <p className="muted">
          {isLogin ? "Ще не зареєструвалися в цій сумній історії?" : "Вже маєте акаунт у нашому тихому клубі?"}{" "}
          <Link href={isLogin ? "/register" : "/login"}>{isLogin ? "Реєстрація" : "Вхід"}</Link>
        </p>
      </form>
    </section>
  );
}
