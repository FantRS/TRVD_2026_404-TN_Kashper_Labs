"use client";

import { useEffect, useState } from "react";

import {
  createProduct,
  createProductCategory,
  createService,
  createServiceCategory,
  getOrdersReport,
  getPaymentsReport,
  getProductCategories,
  getProducts,
  getServiceCategories,
  getServices,
  getUsers,
  updateUserActiveState,
  updateUserRole,
} from "@/lib/api";
import type {
  AuthUser,
  Category,
  OrdersReport,
  PaginatedResponse,
  PaymentsReport,
  ProductItem,
  ServiceItem,
  UserAdmin,
  UserRole,
} from "@/lib/types";
import { formatCredits, roleLabel } from "@/lib/utils";

function lastThirtyDaysRange() {
  const dateTo = new Date();
  const dateFrom = new Date();
  dateFrom.setDate(dateTo.getDate() - 30);

  return {
    dateFrom: dateFrom.toISOString(),
    dateTo: dateTo.toISOString(),
  };
}

type AdminTab = "overview" | "admin";

export function AdminDashboard({
  user,
  token,
}: {
  user: AuthUser;
  token: string;
}) {
  const [activeTab, setActiveTab] = useState<AdminTab>("admin");
  const [loading, setLoading] = useState(true);
  const [submitting, setSubmitting] = useState(false);
  const [notice, setNotice] = useState<string | null>(null);
  const [error, setError] = useState<string | null>(null);
  const [serviceCategories, setServiceCategories] = useState<Category[]>([]);
  const [productCategories, setProductCategories] = useState<Category[]>([]);
  const [recentServices, setRecentServices] = useState<ServiceItem[]>([]);
  const [recentProducts, setRecentProducts] = useState<ProductItem[]>([]);
  const [usersPage, setUsersPage] = useState<PaginatedResponse<UserAdmin> | null>(null);
  const [ordersReport, setOrdersReport] = useState<OrdersReport | null>(null);
  const [paymentsReport, setPaymentsReport] = useState<PaymentsReport | null>(null);

  const [serviceCategoryForm, setServiceCategoryForm] = useState({
    name: "",
    description: "",
  });
  const [productCategoryForm, setProductCategoryForm] = useState({
    name: "",
    description: "",
  });
  const [serviceForm, setServiceForm] = useState({
    category_id: "",
    name: "",
    description: "",
    base_price: "0",
    duration_minutes: "60",
  });
  const [productForm, setProductForm] = useState({
    category_id: "",
    sku: "",
    name: "",
    description: "",
    unit_price: "0",
    stock_qty: "1",
  });

  async function loadAdminData() {
    setLoading(true);
    setError(null);

    try {
      const range = lastThirtyDaysRange();
      const [
        loadedServiceCategories,
        loadedProductCategories,
        servicesResponse,
        productsResponse,
        loadedUsers,
        loadedOrdersReport,
        loadedPaymentsReport,
      ] = await Promise.all([
        getServiceCategories(),
        getProductCategories(),
        getServices({ per_page: 5 }),
        getProducts({ per_page: 5 }),
        getUsers(token, { per_page: 8 }),
        getOrdersReport(range.dateFrom, range.dateTo, token),
        getPaymentsReport(range.dateFrom, range.dateTo, token),
      ]);

      setServiceCategories(loadedServiceCategories);
      setProductCategories(loadedProductCategories);
      setRecentServices(servicesResponse.data);
      setRecentProducts(productsResponse.data);
      setUsersPage(loadedUsers);
      setOrdersReport(loadedOrdersReport);
      setPaymentsReport(loadedPaymentsReport);

      if (!serviceForm.category_id && loadedServiceCategories[0]) {
        setServiceForm((current) => ({ ...current, category_id: loadedServiceCategories[0].id }));
      }

      if (!productForm.category_id && loadedProductCategories[0]) {
        setProductForm((current) => ({ ...current, category_id: loadedProductCategories[0].id }));
      }
    } catch (loadError) {
      setError(loadError instanceof Error ? loadError.message : "Адмінка не завантажилась. Навіть влада іноді стикається з бюрократією.");
    } finally {
      setLoading(false);
    }
  }

  useEffect(() => {
    void loadAdminData();
  }, [token]);

  async function handleCreateServiceCategory(event: React.FormEvent<HTMLFormElement>) {
    event.preventDefault();
    setSubmitting(true);
    setNotice(null);
    setError(null);

    try {
      const category = await createServiceCategory(
        {
          name: serviceCategoryForm.name,
          description: serviceCategoryForm.description || undefined,
        },
        token,
      );
      setServiceCategoryForm({ name: "", description: "" });
      setServiceCategories((current) => [...current, category]);
      setServiceForm((current) => ({ ...current, category_id: current.category_id || category.id }));
      setNotice(`Категорію послуг "${category.name}" створено. Ще один розділ у великій книзі неминучого.`);
    } catch (submitError) {
      setError(submitError instanceof Error ? submitError.message : "Не вдалося створити категорію послуг.");
    } finally {
      setSubmitting(false);
    }
  }

  async function handleCreateProductCategory(event: React.FormEvent<HTMLFormElement>) {
    event.preventDefault();
    setSubmitting(true);
    setNotice(null);
    setError(null);

    try {
      const category = await createProductCategory(
        {
          name: productCategoryForm.name,
          description: productCategoryForm.description || undefined,
        },
        token,
      );
      setProductCategoryForm({ name: "", description: "" });
      setProductCategories((current) => [...current, category]);
      setProductForm((current) => ({ ...current, category_id: current.category_id || category.id }));
      setNotice(`Категорію товарів "${category.name}" створено. Адміністрування смерті йде за планом.`);
    } catch (submitError) {
      setError(submitError instanceof Error ? submitError.message : "Не вдалося створити категорію товарів.");
    } finally {
      setSubmitting(false);
    }
  }

  async function handleCreateService(event: React.FormEvent<HTMLFormElement>) {
    event.preventDefault();
    setSubmitting(true);
    setNotice(null);
    setError(null);

    try {
      const service = await createService(
        {
          category_id: serviceForm.category_id,
          name: serviceForm.name,
          description: serviceForm.description || undefined,
          base_price: Number(serviceForm.base_price),
          duration_minutes: Number(serviceForm.duration_minutes),
        },
        token,
      );
      setServiceForm((current) => ({
        ...current,
        name: "",
        description: "",
        base_price: "0",
        duration_minutes: "60",
      }));
      setRecentServices((current) => [service, ...current].slice(0, 5));
      setNotice(`Послугу "${service.name}" створено. Ще одна опція для тих, кому вже точно нікуди поспішати.`);
    } catch (submitError) {
      setError(submitError instanceof Error ? submitError.message : "Не вдалося створити послугу.");
    } finally {
      setSubmitting(false);
    }
  }

  async function handleCreateProduct(event: React.FormEvent<HTMLFormElement>) {
    event.preventDefault();
    setSubmitting(true);
    setNotice(null);
    setError(null);

    try {
      const product = await createProduct(
        {
          category_id: productForm.category_id,
          sku: productForm.sku,
          name: productForm.name,
          description: productForm.description || undefined,
          unit_price: Number(productForm.unit_price),
          stock_qty: Number(productForm.stock_qty),
        },
        token,
      );
      setProductForm((current) => ({
        ...current,
        sku: "",
        name: "",
        description: "",
        unit_price: "0",
        stock_qty: "1",
      }));
      setRecentProducts((current) => [product, ...current].slice(0, 5));
      setNotice(`Товар "${product.name}" створено. Склад готовий до ще одного естетично сумного поповнення.`);
    } catch (submitError) {
      setError(submitError instanceof Error ? submitError.message : "Не вдалося створити товар.");
    } finally {
      setSubmitting(false);
    }
  }

  async function handleRoleChange(userId: string, role: UserRole) {
    setNotice(null);
    setError(null);

    try {
      const updatedUser = await updateUserRole(userId, role, token);
      setUsersPage((current) =>
        current
          ? {
              ...current,
              data: current.data.map((item) => (item.id === updatedUser.id ? updatedUser : item)),
            }
          : current,
      );
      setNotice(`Роль для ${updatedUser.email} змінено. Влада видана офіційно.`);
    } catch (submitError) {
      setError(submitError instanceof Error ? submitError.message : "Не вдалося оновити роль користувача.");
    }
  }

  async function handleActiveToggle(userId: string, isActive: boolean) {
    setNotice(null);
    setError(null);

    try {
      const updatedUser = await updateUserActiveState(userId, isActive, token);
      setUsersPage((current) =>
        current
          ? {
              ...current,
              data: current.data.map((item) => (item.id === updatedUser.id ? updatedUser : item)),
            }
          : current,
      );
      setNotice(
        isActive
          ? `Користувача ${updatedUser.email} повернуто до списку живих акаунтів.`
          : `Користувача ${updatedUser.email} акуратно вимкнено з процесу.`,
      );
    } catch (submitError) {
      setError(submitError instanceof Error ? submitError.message : "Не вдалося змінити активність користувача.");
    }
  }

  return (
    <div className="dashboard-stack">
      <section className="panel">
        <div className="section-heading compact">
          <div>
            <p className="eyebrow">Акаунт адміністратора</p>
            <h1 className="section-title">{user.full_name}</h1>
            <p className="lead">
              Тут керується каталог, користувачі й коротка аналітика. Іншими словами, саме тут
              адмін вирішує, які товари з’являться на вітрині й кому сьогодні дати владу над сумом.
            </p>
          </div>
          <div className="tabs">
            <button
              type="button"
              className={activeTab === "overview" ? "tab active" : "tab"}
              onClick={() => setActiveTab("overview")}
            >
              Огляд
            </button>
            <button
              type="button"
              className={activeTab === "admin" ? "tab active" : "tab"}
              onClick={() => setActiveTab("admin")}
            >
              Адмін
            </button>
          </div>
        </div>
      </section>

      {notice ? <div className="panel subtle">{notice}</div> : null}
      {error ? <div className="panel subtle muted">{error}</div> : null}

      {loading ? (
        <div className="panel">Завантажуємо адмінку... навіть владі потрібен момент, щоб розгорнути траурний штаб.</div>
      ) : activeTab === "overview" ? (
        <div className="dashboard-grid wide">
          <div className="stack-24">
            <section className="panel">
              <div className="panel-header">
                <h3>Останні 30 днів</h3>
                <span className="muted">Короткий підсумок по процесах</span>
              </div>
              <div className="metric-grid compact">
                <div className="metric-card">
                  <span>Усі замовлення</span>
                  <strong>{ordersReport?.total_orders ?? 0}</strong>
                </div>
                <div className="metric-card">
                  <span>Сума замовлень</span>
                  <strong>{formatCredits(ordersReport?.total_amount ?? 0)}</strong>
                </div>
                <div className="metric-card">
                  <span>Очікують оплату</span>
                  <strong>{ordersReport?.awaiting_payment_orders ?? 0}</strong>
                </div>
                <div className="metric-card">
                  <span>Завершені</span>
                  <strong>{ordersReport?.completed_orders ?? 0}</strong>
                </div>
                <div className="metric-card">
                  <span>Успішні оплати</span>
                  <strong>{paymentsReport?.total_payments ?? 0}</strong>
                </div>
                <div className="metric-card">
                  <span>Оплачено</span>
                  <strong>{formatCredits(paymentsReport?.paid_amount ?? 0)}</strong>
                </div>
              </div>
            </section>

            <section className="panel">
              <div className="panel-header">
                <h3>Останні послуги</h3>
                <span className="muted">Що зараз видно в каталозі</span>
              </div>
              <div className="stack-list">
                {recentServices.map((service) => (
                  <div key={service.id} className="detail-row">
                    <span>{service.name}</span>
                    <strong>{formatCredits(service.base_price)}</strong>
                  </div>
                ))}
              </div>
            </section>
          </div>

          <div className="stack-24">
            <section className="panel">
              <div className="panel-header">
                <h3>Останні товари</h3>
                <span className="muted">Вітрина під контролем</span>
              </div>
              <div className="stack-list">
                {recentProducts.map((product) => (
                  <div key={product.id} className="detail-row">
                    <span>{product.name}</span>
                    <strong>{formatCredits(product.unit_price)}</strong>
                  </div>
                ))}
              </div>
            </section>

            <section className="panel">
              <div className="panel-header">
                <h3>Користувачі</h3>
                <span className="muted">{usersPage?.meta.total_items ?? 0} у системі</span>
              </div>
              <div className="stack-list">
                {usersPage?.data.slice(0, 5).map((entry) => (
                  <div key={entry.id} className="detail-row">
                    <span>{entry.email}</span>
                    <strong>{roleLabel(entry.role)}</strong>
                  </div>
                ))}
              </div>
            </section>
          </div>
        </div>
      ) : (
        <div className="dashboard-grid wide">
          <div className="stack-24">
            <section className="panel">
              <div className="panel-header">
                <h3>Створити категорію послуг</h3>
                <span className="muted">Щоб каталог ріс, як список справ у понеділок</span>
              </div>
              <form className="form-grid" onSubmit={handleCreateServiceCategory}>
                <label className="field">
                  <span>Назва категорії</span>
                  <input
                    className="input"
                    required
                    value={serviceCategoryForm.name}
                    onChange={(event) =>
                      setServiceCategoryForm((current) => ({ ...current, name: event.target.value }))
                    }
                  />
                </label>
                <label className="field full-span">
                  <span>Опис</span>
                  <textarea
                    className="textarea"
                    value={serviceCategoryForm.description}
                    onChange={(event) =>
                      setServiceCategoryForm((current) => ({
                        ...current,
                        description: event.target.value,
                      }))
                    }
                  />
                </label>
                <button type="submit" className="button" disabled={submitting}>
                  Створити категорію послуг
                </button>
              </form>
            </section>

            <section className="panel">
              <div className="panel-header">
                <h3>Створити послугу</h3>
                <span className="muted">Додаємо ще один офіційний фінальний сценарій</span>
              </div>
              <form className="form-grid" onSubmit={handleCreateService}>
                <label className="field">
                  <span>Категорія</span>
                  <select
                    className="input"
                    required
                    value={serviceForm.category_id}
                    onChange={(event) =>
                      setServiceForm((current) => ({ ...current, category_id: event.target.value }))
                    }
                  >
                    <option value="">Оберіть категорію</option>
                    {serviceCategories.map((category) => (
                      <option key={category.id} value={category.id}>
                        {category.name}
                      </option>
                    ))}
                  </select>
                </label>
                <label className="field">
                  <span>Назва послуги</span>
                  <input
                    className="input"
                    required
                    value={serviceForm.name}
                    onChange={(event) =>
                      setServiceForm((current) => ({ ...current, name: event.target.value }))
                    }
                  />
                </label>
                <label className="field">
                  <span>Ціна</span>
                  <input
                    className="input"
                    type="number"
                    min="0"
                    required
                    value={serviceForm.base_price}
                    onChange={(event) =>
                      setServiceForm((current) => ({ ...current, base_price: event.target.value }))
                    }
                  />
                </label>
                <label className="field">
                  <span>Тривалість, хв</span>
                  <input
                    className="input"
                    type="number"
                    min="1"
                    required
                    value={serviceForm.duration_minutes}
                    onChange={(event) =>
                      setServiceForm((current) => ({
                        ...current,
                        duration_minutes: event.target.value,
                      }))
                    }
                  />
                </label>
                <label className="field full-span">
                  <span>Опис</span>
                  <textarea
                    className="textarea"
                    value={serviceForm.description}
                    onChange={(event) =>
                      setServiceForm((current) => ({ ...current, description: event.target.value }))
                    }
                  />
                </label>
                <button type="submit" className="button" disabled={submitting || !serviceForm.category_id}>
                  Створити послугу
                </button>
              </form>
            </section>
          </div>

          <div className="stack-24">
            <section className="panel">
              <div className="panel-header">
                <h3>Створити категорію товарів</h3>
                <span className="muted">Щоб домовини теж жили за каталогом</span>
              </div>
              <form className="form-grid" onSubmit={handleCreateProductCategory}>
                <label className="field">
                  <span>Назва категорії</span>
                  <input
                    className="input"
                    required
                    value={productCategoryForm.name}
                    onChange={(event) =>
                      setProductCategoryForm((current) => ({ ...current, name: event.target.value }))
                    }
                  />
                </label>
                <label className="field full-span">
                  <span>Опис</span>
                  <textarea
                    className="textarea"
                    value={productCategoryForm.description}
                    onChange={(event) =>
                      setProductCategoryForm((current) => ({
                        ...current,
                        description: event.target.value,
                      }))
                    }
                  />
                </label>
                <button type="submit" className="button" disabled={submitting}>
                  Створити категорію товарів
                </button>
              </form>
            </section>

            <section className="panel">
              <div className="panel-header">
                <h3>Створити товар</h3>
                <span className="muted">Поповнюємо склад похмурої, але потрібної краси</span>
              </div>
              <form className="form-grid" onSubmit={handleCreateProduct}>
                <label className="field">
                  <span>Категорія</span>
                  <select
                    className="input"
                    required
                    value={productForm.category_id}
                    onChange={(event) =>
                      setProductForm((current) => ({ ...current, category_id: event.target.value }))
                    }
                  >
                    <option value="">Оберіть категорію</option>
                    {productCategories.map((category) => (
                      <option key={category.id} value={category.id}>
                        {category.name}
                      </option>
                    ))}
                  </select>
                </label>
                <label className="field">
                  <span>SKU</span>
                  <input
                    className="input"
                    required
                    value={productForm.sku}
                    onChange={(event) =>
                      setProductForm((current) => ({ ...current, sku: event.target.value }))
                    }
                  />
                </label>
                <label className="field">
                  <span>Назва товару</span>
                  <input
                    className="input"
                    required
                    value={productForm.name}
                    onChange={(event) =>
                      setProductForm((current) => ({ ...current, name: event.target.value }))
                    }
                  />
                </label>
                <label className="field">
                  <span>Ціна</span>
                  <input
                    className="input"
                    type="number"
                    min="0"
                    required
                    value={productForm.unit_price}
                    onChange={(event) =>
                      setProductForm((current) => ({ ...current, unit_price: event.target.value }))
                    }
                  />
                </label>
                <label className="field">
                  <span>Кількість на складі</span>
                  <input
                    className="input"
                    type="number"
                    min="0"
                    required
                    value={productForm.stock_qty}
                    onChange={(event) =>
                      setProductForm((current) => ({ ...current, stock_qty: event.target.value }))
                    }
                  />
                </label>
                <label className="field full-span">
                  <span>Опис</span>
                  <textarea
                    className="textarea"
                    value={productForm.description}
                    onChange={(event) =>
                      setProductForm((current) => ({ ...current, description: event.target.value }))
                    }
                  />
                </label>
                <button type="submit" className="button" disabled={submitting || !productForm.category_id}>
                  Створити товар
                </button>
              </form>
            </section>
          </div>

          <section className="panel full-span">
            <div className="panel-header">
              <h3>Користувачі системи</h3>
              <span className="muted">Тут роздаємо ролі й вирішуємо, хто сьогодні в строю</span>
            </div>
            <div className="stack-list">
              {usersPage?.data.map((entry) => (
                <div key={entry.id} className="admin-user-row">
                  <div className="grow">
                    <strong>{entry.full_name}</strong>
                    <p className="muted">
                      {entry.email} · {entry.phone || "телефон не залишив, мабуть, і так не до дзвінків"}
                    </p>
                  </div>
                  <div className="admin-user-actions">
                    <select
                      className="input"
                      value={entry.role}
                      onChange={(event) => {
                        void handleRoleChange(entry.id, event.target.value as UserRole);
                      }}
                    >
                      <option value="user">Покупець</option>
                      <option value="employee">Працівник</option>
                      <option value="admin">Адмін</option>
                    </select>
                    <button
                      type="button"
                      className="button ghost"
                      onClick={() => {
                        void handleActiveToggle(entry.id, !entry.is_active);
                      }}
                    >
                      {entry.is_active ? "Приспати акаунт" : "Повернути до списку живих"}
                    </button>
                  </div>
                </div>
              ))}
            </div>
          </section>
        </div>
      )}
    </div>
  );
}
