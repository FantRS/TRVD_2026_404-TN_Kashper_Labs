"use client";

import Link from "next/link";
import { useEffect, useState } from "react";

import {
  addProductToCart,
  addServiceToCart,
  checkout,
  createAppointment,
  createPayment,
  getCart,
  getOrder,
  listOrders,
  removeProductFromCart,
  removeServiceFromCart,
} from "@/lib/api";
import { useAuth } from "@/lib/auth";
import { Order, OrderItem, OrderSummary } from "@/lib/types";
import {
  formatCredits,
  formatDateTime,
  orderStatusLabel,
  roleLabel,
} from "@/lib/utils";
import { AdminDashboard } from "@/components/admin-dashboard";
import { ScheduleExplorer } from "@/components/schedule-explorer";

export function AccountDashboard() {
  const { isHydrated, isAuthenticated, token, user, refreshUser } = useAuth();
  const [cart, setCart] = useState<Order | null>(null);
  const [orders, setOrders] = useState<OrderSummary[]>([]);
  const [selectedOrderId, setSelectedOrderId] = useState<string | null>(null);
  const [selectedOrder, setSelectedOrder] = useState<Order | null>(null);
  const [loading, setLoading] = useState(true);
  const [busy, setBusy] = useState(false);
  const [notice, setNotice] = useState<string | null>(null);
  const [error, setError] = useState<string | null>(null);
  const [selectedSlot, setSelectedSlot] = useState<string | null>(null);
  const [appointmentLocation, setAppointmentLocation] = useState("Київ, меморіальний сервісний центр");
  const [paymentComment, setPaymentComment] = useState("");
  const [checkoutForm, setCheckoutForm] = useState({
    contact_name: "",
    contact_phone: "",
    contact_email: "",
    delivery_address: "",
  });

  useEffect(() => {
    if (!user || checkoutForm.contact_email) {
      return;
    }

    setCheckoutForm({
      contact_name: user.full_name,
      contact_phone: user.phone || "",
      contact_email: user.email,
      delivery_address: "",
    });
  }, [user, checkoutForm.contact_email]);

  useEffect(() => {
    if (!token || !user || user.role !== "user") {
      setLoading(false);
      return;
    }

    const authToken = token;
    let cancelled = false;

    async function loadDashboard(preferredOrderId?: string | null) {
      setLoading(true);
      setError(null);

      try {
        const [cartResponse, orderSummaries] = await Promise.all([
          getCart(authToken),
          listOrders(authToken),
        ]);

        if (cancelled) {
          return;
        }

        setCart(cartResponse);
        setOrders(orderSummaries);

        const nextOrderId = preferredOrderId || orderSummaries[0]?.id || null;
        setSelectedOrderId(nextOrderId);

        if (nextOrderId) {
          const orderResponse = await getOrder(nextOrderId, authToken);
          if (!cancelled) {
            setSelectedOrder(orderResponse);
          }
        } else if (!cancelled) {
          setSelectedOrder(null);
        }
      } catch (loadError) {
        if (!cancelled) {
          setError(loadError instanceof Error ? loadError.message : "Не вдалося завантажити кабінет.");
        }
      } finally {
        if (!cancelled) {
          setLoading(false);
        }
      }
    }

    void loadDashboard(selectedOrderId);

    return () => {
      cancelled = true;
    };
  }, [token, user?.id, user?.role]);

  useEffect(() => {
    if (!selectedOrderId || !token || !user || user.role !== "user") {
      return;
    }

    const authToken = token;
    const currentOrderId = selectedOrderId;
    let cancelled = false;

    async function loadOrderDetails() {
      try {
        const orderResponse = await getOrder(currentOrderId, authToken);
        if (!cancelled) {
          setSelectedOrder(orderResponse);
        }
      } catch (loadError) {
        if (!cancelled) {
          setError(loadError instanceof Error ? loadError.message : "Не вдалося оновити деталі замовлення.");
        }
      }
    }

    void loadOrderDetails();

    return () => {
      cancelled = true;
    };
  }, [selectedOrderId, token, user?.id, user?.role]);

  async function refreshOrders(preferredOrderId?: string | null) {
    if (!token) {
      return;
    }

    const authToken = token;
    const [cartResponse, orderSummaries] = await Promise.all([
      getCart(authToken),
      listOrders(authToken),
    ]);
    setCart(cartResponse);
    setOrders(orderSummaries);

    const nextOrderId = preferredOrderId || orderSummaries[0]?.id || null;
    setSelectedOrderId(nextOrderId);

    if (nextOrderId) {
      setSelectedOrder(await getOrder(nextOrderId, authToken));
    } else {
      setSelectedOrder(null);
    }
  }

  async function handleCartItemUpdate(item: OrderItem, quantity: number) {
    if (!token) {
      return;
    }

    setBusy(true);
    setNotice(null);
    setError(null);

    try {
      const updatedCart =
        item.item_type === "service"
          ? await addServiceToCart(item.reference_id, quantity, token)
          : await addProductToCart(item.reference_id, quantity, token);
      setCart(updatedCart);
      setNotice("Кошик оновлено. Плани на сумний день стають дедалі конкретнішими.");
    } catch (actionError) {
      setError(actionError instanceof Error ? actionError.message : "Не вдалося оновити кошик.");
    } finally {
      setBusy(false);
    }
  }

  async function handleCartItemRemove(item: OrderItem) {
    if (!token) {
      return;
    }

    setBusy(true);
    setNotice(null);
    setError(null);

    try {
      const updatedCart =
        item.item_type === "service"
          ? await removeServiceFromCart(item.reference_id, token)
          : await removeProductFromCart(item.reference_id, token);
      setCart(updatedCart);
      setNotice("Позицію прибрано з кошика. Не кожна похмура ідея має дожити до фінального списку.");
    } catch (actionError) {
      setError(actionError instanceof Error ? actionError.message : "Не вдалося видалити позицію.");
    } finally {
      setBusy(false);
    }
  }

  async function handleCheckout(event: React.FormEvent<HTMLFormElement>) {
    event.preventDefault();

    if (!token) {
      return;
    }

    setBusy(true);
    setNotice(null);
    setError(null);

    try {
      const order = await checkout(checkoutForm, token);
      await refreshOrders(order.id);
      setSelectedSlot(null);
      setNotice(`Замовлення ${order.order_number} оформлено. Тепер можна перейти від планів до зовсім серйозних кроків.`);
    } catch (actionError) {
      setError(actionError instanceof Error ? actionError.message : "Оформлення не вдалося. Схоже, навіть похоронна бюрократія іноді бере реванш.");
    } finally {
      setBusy(false);
    }
  }

  async function handlePayment() {
    if (!token || !selectedOrder) {
      return;
    }

    setBusy(true);
    setNotice(null);
    setError(null);

    try {
      const response = await createPayment(selectedOrder.id, token, paymentComment || undefined);
      await refreshUser();
      await refreshOrders(selectedOrder.id);
      setPaymentComment("");
      setNotice(`Оплату проведено. Баланс схуд до ${formatCredits(response.wallet_balance_after)}. Життя б'є, сервіс списує.`);
    } catch (actionError) {
      setError(actionError instanceof Error ? actionError.message : "Оплату не вдалося виконати.");
    } finally {
      setBusy(false);
    }
  }

  async function handleAppointment() {
    if (!token || !selectedOrder || !selectedSlot) {
      return;
    }

    setBusy(true);
    setNotice(null);
    setError(null);

    try {
      const appointment = await createAppointment(
        selectedOrder.id,
        {
          scheduled_at: selectedSlot,
          location: appointmentLocation,
        },
        token,
      );
      setNotice(`Час заброньовано на ${formatDateTime(appointment.scheduled_at)}. Календар неминучого тепер упорядкований.`);
    } catch (actionError) {
      setError(actionError instanceof Error ? actionError.message : "Не вдалося створити бронювання.");
    } finally {
      setBusy(false);
    }
  }

  if (!isHydrated) {
    return <section className="panel">Завантажуємо сесію... навіть траурним сервісам треба кілька секунд на зібратися.</section>;
  }

  if (!isAuthenticated || !user) {
    return (
      <section className="panel hero-compact">
        <p className="eyebrow">Особистий кабінет</p>
        <h1 className="section-title">Замовлення, баланс і вибраний час відкриваються після входу в цей клуб сумних привілеїв.</h1>
        <p className="lead">
          Після реєстрації можна зберігати покупки, оплачувати замовлення, переглядати історію та
          бронювати час без повторного проходження всіх кіл організаційного суму.
        </p>
        <div className="inline-actions">
          <Link href="/login" className="button">
            Увійти без церемоній
          </Link>
          <Link href="/register" className="button ghost">
            Завести профіль
          </Link>
        </div>
      </section>
    );
  }

  if (user.role === "admin" && token) {
    return <AdminDashboard user={user} token={token} />;
  }

  if (user.role !== "user") {
    return (
      <section className="panel hero-compact">
        <p className="eyebrow">Профіль</p>
        <h1 className="section-title">{user.full_name}</h1>
        <p className="lead">
          Ви увійшли як <strong>{roleLabel(user.role)}</strong>. Цей кабінет насамперед призначений для
          покупців, але каталог і основна інформація на сайті все одно залишаються доступними. Не всім же тут замовляти домовину особисто.
        </p>
        <div className="metric-grid compact">
          <div className="metric-card">
            <span>Роль</span>
            <strong>{roleLabel(user.role)}</strong>
          </div>
          <div className="metric-card">
            <span>Email</span>
            <strong>{user.email}</strong>
          </div>
          <div className="metric-card">
            <span>Статус</span>
            <strong>Профіль живіший за деякі наші кейси</strong>
          </div>
        </div>
      </section>
    );
  }

  return (
    <div className="dashboard-stack">
      <section className="dashboard-grid">
        <div className="panel">
          <p className="eyebrow">Головний організатор цієї сумної логістики</p>
          <h1 className="section-title">{user.full_name}</h1>
          <div className="stack-list">
            <div className="detail-row">
              <span>Email</span>
              <strong>{user.email}</strong>
            </div>
            <div className="detail-row">
              <span>Телефон</span>
              <strong>{user.phone || "Не вказано"}</strong>
            </div>
            <div className="detail-row">
              <span>Баланс</span>
              <strong>{formatCredits(user.wallet_balance)}</strong>
            </div>
            <div className="detail-row">
              <span>Роль</span>
              <strong>{roleLabel(user.role)}</strong>
            </div>
          </div>
        </div>

        <div className="panel">
          <p className="eyebrow">Що маємо на цьому етапі</p>
          <div className="metric-grid compact">
            <div className="metric-card">
              <span>У кошику</span>
              <strong>{cart?.items.length || 0} позицій</strong>
            </div>
            <div className="metric-card">
              <span>Оформлені рішення</span>
              <strong>{orders.length}</strong>
            </div>
            <div className="metric-card">
              <span>Останній статус</span>
              <strong>{selectedOrder ? orderStatusLabel(selectedOrder.current_status_code) : "Немає"}</strong>
            </div>
          </div>
        </div>
      </section>

      {notice ? <div className="panel subtle">{notice}</div> : null}
      {error ? <div className="panel subtle muted">{error}</div> : null}

      <section className="section-heading compact">
        <div>
          <p className="eyebrow">Покупки</p>
          <h2 className="section-title">Усе в одному місці: від вибору домовини до остаточного “домовились”.</h2>
        </div>
        <button
          type="button"
          className="button ghost"
          onClick={() => {
            void refreshOrders(selectedOrderId);
          }}
        >
          Оновити дані
        </button>
      </section>

      {loading ? (
        <div className="panel">Завантажуємо ваші замовлення... сумний архів теж потребує хвилинку.</div>
      ) : (
        <div className="dashboard-grid wide">
          <div className="stack-24">
            <section className="panel">
              <div className="panel-header">
                <h3>Кошик останніх рішень</h3>
                <span className="muted">{formatCredits(cart?.total_amount || 0)} на весь цей фінальний набір</span>
              </div>

              {cart?.items.length ? (
                <div className="stack-list">
                  {cart.items.map((item) => (
                    <div key={item.item_id} className="cart-item">
                      <div className="grow">
                        <strong>{item.title}</strong>
                        <p className="muted">
                          {item.item_type === "service" ? "Послуга" : "Товар"} · {formatCredits(item.unit_price)}
                        </p>
                      </div>
                      <label className="quantity-box inline">
                        <span>К-сть</span>
                        <input
                          className="input"
                          type="number"
                          min="1"
                          defaultValue={item.quantity}
                          onBlur={(event) => {
                            const nextQuantity = Math.max(1, Number(event.target.value) || item.quantity);
                            if (nextQuantity !== item.quantity) {
                              void handleCartItemUpdate(item, nextQuantity);
                            }
                          }}
                        />
                      </label>
                      <button
                        type="button"
                        className="button ghost"
                        disabled={busy}
                        onClick={() => {
                          void handleCartItemRemove(item);
                        }}
                      >
                        Прибрати
                      </button>
                    </div>
                  ))}
                </div>
              ) : (
                <div className="empty-state">
                  <h4>Кошик поки порожній</h4>
                  <p className="muted">Додайте послуги або товари з каталогу, коли будете готові перетворити сум на конкретний список.</p>
                </div>
              )}
            </section>

            <section className="panel">
              <div className="panel-header">
                <h3>Фінальне оформлення</h3>
                <span className="muted">Момент, коли плани стають офіційними</span>
              </div>

              <form className="form-grid" onSubmit={handleCheckout}>
                <label className="field">
                  <span>Контактна особа</span>
                  <input
                    className="input"
                    required
                    value={checkoutForm.contact_name}
                    onChange={(event) =>
                      setCheckoutForm((current) => ({ ...current, contact_name: event.target.value }))
                    }
                  />
                </label>

                <label className="field">
                  <span>Телефон</span>
                  <input
                    className="input"
                    required
                    value={checkoutForm.contact_phone}
                    onChange={(event) =>
                      setCheckoutForm((current) => ({ ...current, contact_phone: event.target.value }))
                    }
                  />
                </label>

                <label className="field">
                  <span>Email</span>
                  <input
                    className="input"
                    type="email"
                    required
                    value={checkoutForm.contact_email}
                    onChange={(event) =>
                      setCheckoutForm((current) => ({ ...current, contact_email: event.target.value }))
                    }
                  />
                </label>

                <label className="field full-span">
                  <span>Адреса</span>
                  <textarea
                    className="textarea"
                    required
                    value={checkoutForm.delivery_address}
                    onChange={(event) =>
                      setCheckoutForm((current) => ({ ...current, delivery_address: event.target.value }))
                    }
                  />
                </label>

                <button type="submit" className="button" disabled={busy || !cart?.items.length}>
                  {busy ? "Зачекайте, ми вже ставимо печатку..." : "Підтвердити замовлення"}
                </button>
              </form>
            </section>
          </div>

          <div className="stack-24">
            <section className="panel">
              <div className="panel-header">
                <h3>Мої сумні покупки</h3>
                <span className="muted">{orders.length} записів у вашій особистій хроніці неминучого</span>
              </div>

              {orders.length ? (
                <div className="order-list">
                  {orders.map((order) => (
                    <button
                      key={order.id}
                      type="button"
                      className={selectedOrderId === order.id ? "order-summary active" : "order-summary"}
                      onClick={() => setSelectedOrderId(order.id)}
                    >
                      <div>
                        <strong>{order.order_number}</strong>
                        <p className="muted">{orderStatusLabel(order.current_status_code)}</p>
                      </div>
                      <span>{formatCredits(order.total_amount)}</span>
                    </button>
                  ))}
                </div>
              ) : (
                <div className="empty-state">
                  <h4>Оформлених покупок ще немає</h4>
                  <p className="muted">Після першого оформлення тут з’явиться вся історія ваших сумних, але добре структурованих рішень.</p>
                </div>
              )}
            </section>

            {selectedOrder ? (
              <section className="panel">
                <div className="panel-header">
                  <div>
                    <h3>{selectedOrder.order_number}</h3>
                    <p className="muted">{orderStatusLabel(selectedOrder.current_status_code)}</p>
                  </div>
                  <span>{formatCredits(selectedOrder.total_amount)}</span>
                </div>

                <div className="stack-list">
                  {selectedOrder.items.map((item) => (
                    <div key={item.item_id} className="detail-row">
                      <span>
                        {item.title} × {item.quantity}
                      </span>
                      <strong>{formatCredits(item.unit_price * item.quantity)}</strong>
                    </div>
                  ))}
                </div>

                <div className="soft-line" />

                <label className="field">
                  <span>Коментар до оплати</span>
                  <textarea
                    className="textarea"
                    value={paymentComment}
                    onChange={(event) => setPaymentComment(event.target.value)}
                    placeholder="Напишіть щось коротке, якщо хочете залишити слід ще й тут"
                  />
                </label>

                <button
                  type="button"
                  className="button wide"
                  disabled={
                    busy ||
                    !["awaiting_payment", "new"].includes(selectedOrder.current_status_code)
                  }
                  onClick={() => {
                    void handlePayment();
                  }}
                >
                  {busy ? "Проводимо списання..." : "Оплатити й не озиратися"}
                </button>
              </section>
            ) : null}
          </div>
        </div>
      )}

      {selectedOrder ? (
        <section className="dashboard-grid wide">
          <ScheduleExplorer
            selectable
            selectedSlot={selectedSlot}
            onSelectSlot={(slot) => setSelectedSlot(slot)}
          />

          <div className="panel">
            <div className="panel-header">
              <h3>Закріпити час за замовленням</h3>
              <span className="muted">{selectedOrder.order_number}</span>
            </div>

            <label className="field">
              <span>Обраний слот</span>
              <input
                className="input"
                value={selectedSlot ? formatDateTime(selectedSlot) : ""}
                readOnly
                placeholder="Оберіть вільний час ліворуч, поки його не зайняв хтось інший"
              />
            </label>

            <label className="field">
              <span>Локація</span>
              <input
                className="input"
                value={appointmentLocation}
                onChange={(event) => setAppointmentLocation(event.target.value)}
              />
            </label>

            <button
              type="button"
              className="button wide"
              disabled={busy || !selectedSlot}
              onClick={() => {
                void handleAppointment();
              }}
            >
              Закріпити цей фінальний таймінг
            </button>
          </div>
        </section>
      ) : null}
    </div>
  );
}
