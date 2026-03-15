import { UserRole } from "@/lib/types";

export function formatCredits(value: number) {
  return `${new Intl.NumberFormat("uk-UA", {
    minimumFractionDigits: 0,
    maximumFractionDigits: 2,
  }).format(value)} крд`;
}

export function formatDateTime(value: string) {
  return new Intl.DateTimeFormat("uk-UA", {
    dateStyle: "medium",
    timeStyle: "short",
  }).format(new Date(value));
}

export function formatDate(value: string) {
  return new Intl.DateTimeFormat("uk-UA", {
    dateStyle: "full",
  }).format(new Date(value));
}

export function roleLabel(role: UserRole) {
  switch (role) {
    case "admin":
      return "Адміністратор";
    case "employee":
      return "Працівник";
    default:
      return "Покупець";
  }
}

export function orderStatusLabel(status: string) {
  switch (status) {
    case "draft":
      return "Чернетка";
    case "new":
      return "Нове";
    case "awaiting_payment":
      return "Очікує оплату";
    case "confirmed":
      return "Підтверджено";
    case "in_progress":
      return "Виконується";
    case "completed":
      return "Завершено";
    case "needs_revision":
      return "Потребує уточнення";
    case "cancelled":
      return "Скасовано";
    default:
      return status;
  }
}

export function toRfc3339(localDateTime: string) {
  return new Date(localDateTime).toISOString();
}

export function dateTimeLocalFromIso(value: string) {
  const date = new Date(value);
  const offset = date.getTimezoneOffset();
  const local = new Date(date.getTime() - offset * 60_000);
  return local.toISOString().slice(0, 16);
}
