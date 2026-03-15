"use client";

import Link from "next/link";
import { useEffect, useState } from "react";

import { getProducts, getServices } from "@/lib/api";
import { ProductItem, ServiceItem } from "@/lib/types";
import { formatCredits } from "@/lib/utils";

export function HomeCatalogPreview() {
  const [services, setServices] = useState<ServiceItem[]>([]);
  const [products, setProducts] = useState<ProductItem[]>([]);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    let cancelled = false;

    async function loadPreview() {
      try {
        const [servicesResponse, productsResponse] = await Promise.all([
          getServices({ per_page: 3, only_active: true }),
          getProducts({ per_page: 3, only_active: true }),
        ]);

        if (!cancelled) {
          setServices(servicesResponse.data);
          setProducts(productsResponse.data);
        }
      } catch (loadError) {
        if (!cancelled) {
          setError(loadError instanceof Error ? loadError.message : "Не вдалося завантажити каталог.");
        }
      }
    }

    void loadPreview();

    return () => {
      cancelled = true;
    };
  }, []);

  return (
    <section className="section">
      <div className="section-heading">
        <div>
          <p className="eyebrow">Що ми пропонуємо</p>
          <h2 className="section-title">Усе необхідне для організації прощання в одному каталозі.</h2>
        </div>
        <Link href="/catalog" className="button ghost">
          Переглянути весь каталог
        </Link>
      </div>

      {error ? <p className="panel muted">{error}</p> : null}

      <div className="card-grid two-columns">
        <div className="panel">
          <div className="panel-header">
            <h3>Послуги</h3>
            <span className="muted">Кремація та супровід</span>
          </div>
          <div className="stack-list">
            {services.length ? (
              services.map((service) => (
                <article key={service.id} className="mini-card">
                  <div>
                    <h4>{service.name}</h4>
                    <p className="muted">
                      {service.description || "Коротко й по суті: усе, що треба для гідного фіналу без зайвої метушні."}
                    </p>
                  </div>
                  <div className="mini-card-meta">
                    <span>{formatCredits(service.base_price)}</span>
                    <small>{service.duration_minutes} хв</small>
                  </div>
                </article>
              ))
            ) : (
              <div className="empty-state">
                <h4>Каталог послуг ще наповнюється</h4>
                <p className="muted">Незабаром тут з’являться всі доступні варіанти. Смерть не чекає, а каталог ще трохи доробляється.</p>
              </div>
            )}
          </div>
        </div>

        <div className="panel">
          <div className="panel-header">
            <h3>Товари</h3>
            <span className="muted">Домовини, урни, атрибутика</span>
          </div>
          <div className="stack-list">
            {products.length ? (
              products.map((product) => (
                <article key={product.id} className="mini-card">
                  <div>
                    <h4>{product.name}</h4>
                    <p className="muted">
                      {product.description || "Позиція без зайвих прикрас: практична, доречна і готова стати частиною останнього набору."}
                    </p>
                  </div>
                  <div className="mini-card-meta">
                    <span>{formatCredits(product.unit_price)}</span>
                    <small>Склад: {product.stock_qty}</small>
                  </div>
                </article>
              ))
            ) : (
              <div className="empty-state">
                <h4>Супутні товари ще не додані</h4>
                <p className="muted">Ми ще поповнюємо каталог, щоб усе потрібне можна було знайти в одному місці, а не бігати містом у траурному марафоні.</p>
              </div>
            )}
          </div>
        </div>
      </div>
    </section>
  );
}
