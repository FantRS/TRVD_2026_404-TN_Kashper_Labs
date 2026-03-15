"use client";

import Link from "next/link";
import { startTransition, useDeferredValue, useEffect, useState } from "react";
import { useRouter } from "next/navigation";

import {
  addProductToCart,
  addServiceToCart,
  getProductCategories,
  getProducts,
  getServiceCategories,
  getServices,
} from "@/lib/api";
import { useAuth } from "@/lib/auth";
import { Category, PaginatedResponse, ProductItem, ServiceItem } from "@/lib/types";
import { formatCredits } from "@/lib/utils";

type CatalogKind = "services" | "products";

export function CatalogBrowser() {
  const router = useRouter();
  const { token, isAuthenticated, user } = useAuth();
  const [kind, setKind] = useState<CatalogKind>("services");
  const [search, setSearch] = useState("");
  const deferredSearch = useDeferredValue(search);
  const [categoryId, setCategoryId] = useState("");
  const [minPrice, setMinPrice] = useState("");
  const [maxPrice, setMaxPrice] = useState("");
  const [page, setPage] = useState(1);
  const [categories, setCategories] = useState<Category[]>([]);
  const [servicesPage, setServicesPage] = useState<PaginatedResponse<ServiceItem> | null>(null);
  const [productsPage, setProductsPage] = useState<PaginatedResponse<ProductItem> | null>(null);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const [message, setMessage] = useState<string | null>(null);
  const [pendingItemId, setPendingItemId] = useState<string | null>(null);
  const [quantities, setQuantities] = useState<Record<string, number>>({});

  useEffect(() => {
    let cancelled = false;

    async function loadCategories() {
      try {
        const response = kind === "services" ? await getServiceCategories() : await getProductCategories();
        if (!cancelled) {
          setCategories(response);
        }
      } catch (loadError) {
        if (!cancelled) {
          setError(loadError instanceof Error ? loadError.message : "Не вдалося завантажити категорії.");
        }
      }
    }

    void loadCategories();

    return () => {
      cancelled = true;
    };
  }, [kind]);

  useEffect(() => {
    let cancelled = false;

    async function loadCatalog() {
      setLoading(true);
      setError(null);

      try {
        if (kind === "services") {
          const response = await getServices({
            page,
            per_page: 8,
            search: deferredSearch || undefined,
            category_id: categoryId || undefined,
            min_price: minPrice ? Number(minPrice) : undefined,
            max_price: maxPrice ? Number(maxPrice) : undefined,
            only_active: true,
          });

          if (!cancelled) {
            setServicesPage(response);
          }
        } else {
          const response = await getProducts({
            page,
            per_page: 8,
            search: deferredSearch || undefined,
            category_id: categoryId || undefined,
            min_price: minPrice ? Number(minPrice) : undefined,
            max_price: maxPrice ? Number(maxPrice) : undefined,
            only_active: true,
          });

          if (!cancelled) {
            setProductsPage(response);
          }
        }
      } catch (loadError) {
        if (!cancelled) {
          setError(loadError instanceof Error ? loadError.message : "Не вдалося завантажити каталог.");
        }
      } finally {
        if (!cancelled) {
          setLoading(false);
        }
      }
    }

    void loadCatalog();

    return () => {
      cancelled = true;
    };
  }, [kind, page, deferredSearch, categoryId, minPrice, maxPrice]);

  const items = kind === "services" ? servicesPage?.data ?? [] : productsPage?.data ?? [];
  const meta = kind === "services" ? servicesPage?.meta : productsPage?.meta;

  async function handleAdd(referenceId: string) {
    if (!isAuthenticated || !token) {
      router.push("/login");
      return;
    }

    if (user?.role !== "user") {
      setMessage("Купувати через кабінет можуть лише звичайні покупці. Решта тут радше по роботі, ніж по домовинах.");
      return;
    }

    setPendingItemId(referenceId);
    setMessage(null);

    try {
      const quantity = quantities[referenceId] || 1;
      if (kind === "services") {
        await addServiceToCart(referenceId, quantity, token);
      } else {
        await addProductToCart(referenceId, quantity, token);
      }
      setMessage("Позицію додано до кошика. Ще один пункт у сумному, але добре організованому списку.");
    } catch (submitError) {
      setMessage(submitError instanceof Error ? submitError.message : "Не вдалося додати позицію. Навіть фінальні плани інколи буксують.");
    } finally {
      setPendingItemId(null);
    }
  }

  return (
    <div className="stack-24">
      <div className="panel filters-panel">
        <div className="tabs">
          <button
            type="button"
            className={kind === "services" ? "tab active" : "tab"}
            onClick={() => {
              startTransition(() => {
                setKind("services");
                setCategoryId("");
                setPage(1);
              });
            }}
          >
            Послуги
          </button>
          <button
            type="button"
            className={kind === "products" ? "tab active" : "tab"}
            onClick={() => {
              startTransition(() => {
                setKind("products");
                setCategoryId("");
                setPage(1);
              });
            }}
          >
            Товари
          </button>
        </div>

        <div className="form-grid catalog-filters">
          <label className="field">
            <span>Пошук</span>
            <input
              className="input"
              placeholder="Наприклад: урна, домовина, прощання"
              value={search}
              onChange={(event) => {
                setSearch(event.target.value);
                startTransition(() => setPage(1));
              }}
            />
          </label>

          <label className="field">
            <span>Категорія</span>
            <select
              className="input"
              value={categoryId}
              onChange={(event) => {
                setCategoryId(event.target.value);
                startTransition(() => setPage(1));
              }}
            >
              <option value="">Усе сумне й потрібне</option>
              {categories.map((category) => (
                <option key={category.id} value={category.id}>
                  {category.name}
                </option>
              ))}
            </select>
          </label>

          <label className="field">
            <span>Мін. ціна</span>
            <input
              className="input"
              type="number"
              min="0"
              value={minPrice}
              onChange={(event) => {
                setMinPrice(event.target.value);
                startTransition(() => setPage(1));
              }}
            />
          </label>

          <label className="field">
            <span>Макс. ціна</span>
            <input
              className="input"
              type="number"
              min="0"
              value={maxPrice}
              onChange={(event) => {
                setMaxPrice(event.target.value);
                startTransition(() => setPage(1));
              }}
            />
          </label>
        </div>
      </div>

      {message ? <div className="panel muted">{message}</div> : null}
      {error ? <div className="panel muted">{error}</div> : null}

      {loading ? (
        <div className="card-grid three-columns">
          {Array.from({ length: 6 }).map((_, index) => (
            <div key={index} className="catalog-card skeleton-card" />
          ))}
        </div>
      ) : items.length ? (
        <div className="card-grid three-columns">
          {items.map((item) => {
            const quantity = quantities[item.id] || 1;
            const price = kind === "services" ? (item as ServiceItem).base_price : (item as ProductItem).unit_price;

            return (
              <article key={item.id} className="catalog-card">
                <div className="catalog-card__head">
                <p className="eyebrow">{kind === "services" ? "Послуга" : "Товар"}</p>
                <h3>{item.name}</h3>
              </div>

              <p className="muted grow">
                  {item.description || "Опис ще не додано, але річ уже чекає свого не найвеселішого зоряного часу."}
              </p>

                <div className="catalog-card__meta">
                  <span className="price">{formatCredits(price)}</span>
                  <span className="muted">
                    {kind === "services"
                      ? `${(item as ServiceItem).duration_minutes} хв`
                      : `На складі ${(item as ProductItem).stock_qty}`}
                  </span>
                </div>

                <div className="catalog-card__footer">
                  <label className="quantity-box">
                    <span>К-сть</span>
                    <input
                      type="number"
                      min="1"
                      className="input"
                      value={quantity}
                      onChange={(event) =>
                        setQuantities((current) => ({
                          ...current,
                          [item.id]: Math.max(1, Number(event.target.value) || 1),
                        }))
                      }
                    />
                  </label>

                  {isAuthenticated && user?.role === "user" ? (
                    <button
                      type="button"
                      className="button"
                      disabled={pendingItemId === item.id}
                      onClick={() => {
                        void handleAdd(item.id);
                      }}
                    >
                      {pendingItemId === item.id ? "Додаємо в траурний список..." : "До кошика"}
                    </button>
                ) : (
                  <Link href={isAuthenticated ? "/account" : "/login"} className="button ghost">
                      {isAuthenticated ? "До кабінету рішень" : "Увійти, щоб замовити"}
                  </Link>
                )}
              </div>
            </article>
          );
          })}
        </div>
      ) : (
        <div className="empty-state panel">
          <h3>За цими фільтрами поки нічого не знайшлося</h3>
          <p className="muted">
            Спробуй інший пошук або повернись пізніше. Навіть у таких справах асортимент інколи бере паузу.
          </p>
        </div>
      )}

      {meta ? (
        <div className="pagination-row">
          <button
            type="button"
            className="button ghost"
            disabled={meta.current_page <= 1}
            onClick={() => startTransition(() => setPage((current) => Math.max(1, current - 1)))}
          >
            Крок назад
          </button>
          <span className="muted">
            Сторінка {meta.current_page} з {meta.total_pages}
          </span>
          <button
            type="button"
            className="button ghost"
            disabled={meta.current_page >= meta.total_pages}
            onClick={() => startTransition(() => setPage((current) => current + 1))}
          >
            Рухаємося далі
          </button>
        </div>
      ) : null}
    </div>
  );
}
