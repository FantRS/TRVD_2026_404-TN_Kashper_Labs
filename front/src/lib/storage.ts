const TOKEN_KEY = "crematory.access_token";
const USER_KEY = "crematory.user_snapshot";

function canUseStorage() {
  return typeof window !== "undefined";
}

export function getStoredToken() {
  if (!canUseStorage()) {
    return null;
  }

  return window.localStorage.getItem(TOKEN_KEY);
}

export function setStoredToken(token: string) {
  if (!canUseStorage()) {
    return;
  }

  window.localStorage.setItem(TOKEN_KEY, token);
}

export function clearStoredToken() {
  if (!canUseStorage()) {
    return;
  }

  window.localStorage.removeItem(TOKEN_KEY);
}

export function getStoredUserSnapshot<T>() {
  if (!canUseStorage()) {
    return null;
  }

  const raw = window.localStorage.getItem(USER_KEY);
  if (!raw) {
    return null;
  }

  try {
    return JSON.parse(raw) as T;
  } catch {
    window.localStorage.removeItem(USER_KEY);
    return null;
  }
}

export function setStoredUserSnapshot<T>(user: T) {
  if (!canUseStorage()) {
    return;
  }

  window.localStorage.setItem(USER_KEY, JSON.stringify(user));
}

export function clearStoredUserSnapshot() {
  if (!canUseStorage()) {
    return;
  }

  window.localStorage.removeItem(USER_KEY);
}
