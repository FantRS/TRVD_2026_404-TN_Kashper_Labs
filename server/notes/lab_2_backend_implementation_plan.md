# План реалізації бекенду CrematoryRs для Lab 2

## 1. Призначення документа

Цей документ фіксує детальний поетапний план реалізації бекенду `CrematoryRs` на базі вже наявного каркасу REST API. План побудований на основі:

- `server/notes/lab_2.md` як джерела цільової архітектури, C4-моделі та базової ER-схеми;
- `server/notes/lab_1.md` як джерела функціональних і нефункціональних вимог;
- `server/notes/backend_development_guide.md` як обов'язкового стандарту структури коду, моделювання, валідації, repository/service/controller flow, Swagger, middleware та error handling.

Документ не описує фронтенд. Він визначає саме бекенд-реалізацію: модулі, міграції, ендпоінти, інтеграції, черговість робіт і критерії готовності.

## 2. Аналіз поточного стану бекенду

### 2.1. Що вже готово

- Серверний каркас на `Rust + Tokio + Actix Web` уже є і проходить `cargo check`.
- Наявні інфраструктурні модулі:
  - `core/config.rs` для env-конфігурації;
  - `core/pg_connector.rs` для `PgPool`;
  - `core/redis_connector.rs` і `app/redis/client.rs` для Redis;
  - `core/app_data.rs` і `app/utils/service_context.rs`;
  - `app/utils/request_error.rs` з хорошим центральним `RequestError`;
  - `app/utils/jwt.rs` для JWT;
  - `app/middlewares/auth_middleware.rs` і `app/middlewares/role_middleware.rs`;
  - `app/utils/pagination.rs` і `app/utils/qs_query.rs`;
  - `app/routes/swagger.rs` як початковий bootstrap для OpenAPI.
- Є базова auth-модель:
  - `Claims`;
  - `UserRole`;
  - Redis whitelist для токенів.

### 2.2. Що фактично ще відсутнє

- Немає жодного повного домену у форматі `controller.rs -> service.rs -> repository.rs -> models/`.
- В `app/domains/` реально існує тільки заготовка `auth`, і вона не містить controller/service/repository.
- `routes/mod.rs` містить лише swagger; бізнес-маршрути закоментовані або не створені.
- `auth/models/response.rs` порожній.
- Каталог, замовлення, розклад, оплати, користувачі, звітність і подієво-аудитна інфраструктура ще не реалізовані.
- Папка `server/migrations/` існує, але фактичних SQL-міграцій немає.
- У конфігурації немає параметрів для:
  - платіжного провайдера;
  - внутрішніх інтеграційних токенів;
  - налаштувань бізнес-розкладу.

### 2.3. Архітектурні розриви, які треба закрити на старті

- Документацію треба повністю узгодити з реальною рольовою моделлю проєкту: `user`, `employee`, `admin`.
- Функціонал кошика з `lab_1` не має окремої сутності в ER-моделі `lab_2`, тому на рівні реалізації треба зафіксувати підхід: або окремий cart, або `draft order`.
- Показ вільних слотів не покривається самою таблицею `APPOINTMENTS`, тому потрібно або:
  - виводити слоти з бізнес-розкладу плюс зайняті `appointments`;
  - або додати таблицю доступності персоналу.
- Для статусного workflow, звітності та аудиту потрібні додаткові cross-cutting механіки:
  - аудит адміндій;
  - експорт звітів;
  - історія статусів із коментарями.

## 3. Обов'язкові правила реалізації з backend guide

Уся реалізація нижче повинна робитися строго за шаблоном з `backend_development_guide.md`.

### 3.1. Структура домену

Кожен новий бізнес-домен має бути самодостатнім модулем:

```text
src/app/domains/<domain_name>/
├── mod.rs
├── controller.rs
├── service.rs
├── repository.rs
└── models/
    ├── mod.rs
    ├── request.rs
    ├── response.rs
    ├── newtype.rs      # якщо потрібен
    └── summary.rs      # якщо потрібен
```

### 3.2. Потік даних

Кожен endpoint реалізується тільки так:

`route -> middleware -> controller -> service -> repository -> response`

### 3.3. Валідація

- `Request` моделі містять сирі DTO.
- `Valid` моделі живуть у `request.rs` і є внутрішніми.
- Уся бізнес-валідація переноситься в `TryFrom<Request> for Valid` або в `TryFrom<String> for Newtype`.
- `controller.rs` завжди робить `request.into_inner().try_into()?` до виклику сервісу.

### 3.4. Шари відповідальності

- `controller.rs`: десеріалізація, `.try_into()?`, `ServiceContext`, логування, `HttpResponse`.
- `service.rs`: бізнес-логіка, транзакції, оркестрація репозиторіїв, публікація подій.
- `repository.rs`: лише SQL, generic executor `E: PgExecutor<'c>`, `Row -> Response` через `From`.
- `models/*`: DTO, row-моделі, summary-моделі, newtype-типи.

### 3.5. Обов'язкові технічні правила

- Кожен handler має `#[utoipa::path(...)]`.
- Кожен handler має `#[tracing::instrument(name = \"...\", skip_all, fields(request_id = %Uuid::new_v4()))]`.
- Кожен сервіс і репозиторій повертає `RequestResult<T>`.
- Жодного `unwrap()` у production-коді.
- Для paginated list ендпоінтів використовувати `PaginationParams`, `PaginatedResponse`, `COUNT(*) OVER()`.
- Для ролей і auth використовувати наявні middleware та узгодити їх із фінальною RBAC-моделлю `user / employee / admin`.
- Після додавання кожного endpoint і кожної DTO оновлювати `swagger.rs`.

## 4. Цільова доменна декомпозиція

Для цього проєкту доцільно реалізувати такі домени:

| Домен | Відповідальність | Основні таблиці | Основні ролі |
| --- | --- | --- | --- |
| `auth` | реєстрація, логін, logout, `me`, JWT whitelist | `users`, `roles` | всі |
| `users` | адміністрування користувачів, зміна ролей, блокування | `users`, `roles`, `audit_logs` | `admin` |
| `catalog` | послуги, товари, категорії, публічний каталог, адмін CRUD | `service_categories`, `services`, `product_categories`, `products` | public, `admin` |
| `orders` | кошик, checkout, замовлення, позиції, статуси, історія | `orders`, `order_service_items`, `order_product_items`, `order_statuses`, `order_status_history` | `user`, `employee`, `admin` |
| `schedule` | доступні слоти, бронювання, підтверджені записи, робочий день співробітника | `appointments` і/або `employee_availability` | public, `employee`, `admin` |
| `payments` | ініціація оплати, webhook, статуси платежів, повторна оплата | `payments`, за потреби `payment_events` | `user`, `employee`, `admin` |
| `reports` | фінансова та операційна звітність, CSV/PDF export | read-model запити до `orders`, `payments`, `order_status_history` | `admin` |
| `events` | доменні події й аудит критичних дій | `audit_logs` і Redis Stream | system, `admin` |

## 5. Базові архітектурні рішення для реалізації

### 5.1. Ролі

Цільовий набір ролей у проєкті:

- `user`
- `employee`
- `admin`

Поточний `UserRole` уже відповідає цій моделі, тому треба не розширювати enum, а вирівняти всю документацію, ендпоінти й RBAC-сценарії під нього. `RoleGuardFactory` варто залишити навколо трьох фабричних сценаріїв:

- `admin_only()`
- `all_employees()`
- `user_only()`

### 5.2. Кошик

Щоб не ламати ER-модель і не створювати окрему зайву сутність, кошик доцільно реалізувати як `orders` у статусі `draft`.

Переваги:

- прямо вкладається в поточну ER-модель;
- дає один шлях `cart -> checkout -> order`;
- дозволяє зберігати товари й послуги через вже заплановані item-таблиці;
- спрощує оплату та історію статусів.

Отже, набір статусів має бути ширшим за перелік із user story про відстеження стану замовлення:

- `draft`
- `new`
- `awaiting_payment`
- `confirmed`
- `in_progress`
- `completed`
- `needs_revision`
- `cancelled`

### 5.3. Розклад і слоти

Для MVP достатньо такого підходу:

- публічні вільні слоти обчислюються з бізнес-годин плюс уже заброньовані `appointments`;
- `appointments` містить фактичне бронювання під замовлення;
- якщо під час реалізації стане зрозуміло, що потрібен контроль індивідуальної доступності працівників, додається таблиця `employee_availability`.

### 5.4. Події та аудит

Після ключових змін стану потрібно публікувати доменні події в Redis Stream:

- `order.created`
- `order.status_changed`
- `payment.created`
- `payment.updated`
- `appointment.confirmed`
- `user.role_changed`

### 5.5. Конвенції схеми БД

Для всієї майбутньої SQL-схеми в цьому проєкті потрібно зафіксувати такі правила:

- Цільова СКБД: `PostgreSQL 18`.
- Для всіх primary key використовувати лише формат:
  - `id UUID PRIMARY KEY DEFAULT uuidv7()`
- Назва primary key у кожній таблиці завжди однакова:
  - `id`
- Усі текстові колонки в SQL описуються як `TEXT`, а не `VARCHAR`, `CHAR` чи `CITEXT`.
- Довжина текстових значень, формат email, формат phone, формат SKU, довжина order number та інші обмеження перевіряються на рівні серверу через `TryFrom`, `newtype` і request validation.
- У кожній таблиці обов'язково має бути поле:
  - `created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()`
- Для довідників на кшталт `roles`, `order_statuses`, `service_categories`, `product_categories` бізнес-логіка повинна орієнтуватися на `code` або `name`, а не на хардкод UUID.

## 6. План реалізації по етапах

### Етап 0. Вирівнювання каркасу під цільову архітектуру

### Ціль

Підготувати поточний skeleton до додавання доменів без хаотичних змін.

### Роботи

- Оновити `src/app/domains/mod.rs` і зареєструвати майбутні домени.
- Створити route-модулі:
  - `auth_routes.rs`
  - `catalog_routes.rs`
  - `orders_routes.rs`
  - `schedule_routes.rs`
  - `payments_routes.rs`
  - `users_routes.rs`
  - `reports_routes.rs`
- Розширити `core/config.rs` новими секціями:
  - `payment`
  - `business_hours`
  - за потреби `internal_service_token`
- Розширити `core/app_data.rs` і `app/utils/service_context.rs` новими reference-полями.
- Створити окремий модуль для подій, наприклад:
  - `src/app/events/mod.rs`
  - `src/app/events/domain_event.rs`
  - `src/app/events/event_publisher.rs`
- Додати health endpoint:
  - `GET /health`
  - `GET /ready`

### Результат етапу

- Каркас усе ще компілюється.
- Є повна структура route/domain/event modules.
- AppData готовий до зберігання інтеграцій і конфігів.

### Етап 1. Проєктування БД і міграції

### Ціль

Зафіксувати реальну SQL-схему під `PostgreSQL 18`, яка покриває `lab_2` і всі user stories.

### Роботи

- Створити базові міграції:
  - `0001_init_roles_users.sql`
  - `0002_init_catalog.sql`
  - `0003_init_orders.sql`
  - `0004_init_appointments.sql`
  - `0005_init_payments.sql`
  - `0006_init_audit_logs.sql`
- Ініціалізувати довідники:
  - ролі;
  - статуси замовлення;
  - початкові категорії за потреби.
- Зафіксувати єдині конвенції схеми:
  - всі PK мають формат `id UUID PRIMARY KEY DEFAULT uuidv7()`;
  - усі текстові поля мають тип `TEXT`;
  - кожна таблиця має `created_at`;
  - довжина та формат текстових значень валідуються на сервері, а не через `VARCHAR(n)`.
- Додати індекси:
  - `users.email`;
  - `services.name`, `products.name`;
  - `orders.order_number`, `orders.user_id`, `orders.current_status_id`, `orders.created_at`;
  - `appointments.scheduled_at`;
  - `payments.external_reference`.
- Зафіксувати `FOREIGN KEY`, `UNIQUE`, `CHECK` обмеження.
- Вирішити типи статусів:
  - lookup table + text code;
  - або PostgreSQL enum тільки там, де це справді стабільно.

### Обов'язкові уточнення до ER

- Додати `audit_logs`, бо це потрібно для NFR-11.
- Зафіксувати, чи `appointments.employee_user_id` nullable до моменту призначення.
- Зафіксувати, що `orders.order_number` генерується сервером і має `UNIQUE`.

### Результат етапу

- Порожній сервер може стартувати з міграціями.
- Схема БД більше не суперечить функціональним вимогам.

### Етап 2. Домен `auth`

### Ціль

Закрити повний auth flow на вже наявному JWT + Redis whitelist.

### Структура файлів

Створити:

- `src/app/domains/auth/controller.rs`
- `src/app/domains/auth/service.rs`
- `src/app/domains/auth/repository.rs`
- доповнити `src/app/domains/auth/models/request.rs`
- заповнити `src/app/domains/auth/models/response.rs`
- за потреби залишити `newtype.rs` для `UserRole`

### Моделі

У `request.rs`:

- `RegisterRequest`
- `RegisterRequestValid`
- `LoginRequest`
- `LoginRequestValid`

У `response.rs`:

- `AuthUserResponse`
- `LoginResponse`
- `CurrentUserResponse`
- `UserRow`

### Репозиторій

Реалізувати:

- `create_user`
- `find_user_by_email`
- `find_user_by_id`
- `update_last_login` за потреби

### Сервіс

Реалізувати:

- register з `Argon2id`;
- login з перевіркою `is_active`;
- logout з інвалідацією конкретного токена;
- logout-all з інвалідацією всіх токенів користувача;
- `me`.

### Контролери та ендпоінти

- `POST /api/auth/register`
- `POST /api/auth/login`
- `POST /api/auth/logout`
- `POST /api/auth/logout-all`
- `GET /api/auth/me`

### Додаткові задачі

- Оновити `Claims` під новий enum ролей.
- Оновити `RoleGuardFactory`.
- Додати Swagger paths/schemas/tags.
- Написати unit-тести для:
  - паролів;
  - `TryFrom`;
  - whitelist logic.

### Результат етапу

- Користувач може створити акаунт, увійти, вийти й отримати `me`.
- Усі захищені scope можуть спиратися на реальну auth-базу.

### Етап 3. Домен `catalog` для публічного читання

### Ціль

Закрити FR-01 і FR-02: перегляд, пошук, фільтрація каталогу.

### Структура домену

Створити:

- `src/app/domains/catalog/mod.rs`
- `src/app/domains/catalog/controller.rs`
- `src/app/domains/catalog/service.rs`
- `src/app/domains/catalog/repository.rs`
- `src/app/domains/catalog/models/request.rs`
- `src/app/domains/catalog/models/response.rs`
- `src/app/domains/catalog/models/summary.rs`
- `src/app/domains/catalog/models/mod.rs`

### Моделі

У `request.rs`:

- `CatalogFilterParams` через `Deserialize + IntoParams`
- `CatalogFilterParamsValid` якщо потрібні newtype для сортування чи price range

У `response.rs`:

- `ServiceResponse`
- `ProductResponse`
- `ServiceCategoryResponse`
- `ProductCategoryResponse`
- відповідні `Row`

У `summary.rs`:

- `CatalogItemSummary`
- `CatalogItemSummaryRow`

### Репозиторій

Реалізувати paginated list через `QueryBuilder`:

- список послуг;
- список товарів;
- список змішаного каталогу, якщо фронтенду це потрібно;
- пошук `ILIKE`;
- фільтр по категорії;
- фільтр по ціні;
- фільтр по `is_active`;
- фільтр по наявності для товарів;
- `COUNT(*) OVER()`.

### Контролери та ендпоінти

- `GET /api/catalog/services`
- `GET /api/catalog/services/{id}`
- `GET /api/catalog/products`
- `GET /api/catalog/products/{id}`
- `GET /api/catalog/categories/services`
- `GET /api/catalog/categories/products`

### Результат етапу

- Публічний каталог повністю читається.
- Є пошук і фільтри, сумісні з `PaginationParams` і guide.

### Етап 4. Адмінський CRUD каталогу

### Ціль

Закрити FR-10: створення, редагування, приховування товарів і послуг.

### Роботи

- Додати create/update/deactivate use cases у `catalog/service.rs`.
- У `request.rs` додати:
  - `ServiceCreateRequest`, `ServiceCreateRequestValid`
  - `ServiceUpdateRequest`, `ServiceUpdateRequestValid`
  - `ProductCreateRequest`, `ProductCreateRequestValid`
  - `ProductUpdateRequest`, `ProductUpdateRequestValid`
  - DTO для категорій
- Винести newtype для:
  - `Money`
  - `StockQuantity`
  - `DurationMinutes`
  - за потреби `Sku`
- У repository використовувати `QueryBuilder` для partial update.

### Ендпоінти

- `POST /api/catalog/services`
- `PATCH /api/catalog/services/{id}`
- `DELETE /api/catalog/services/{id}` або soft delete через `is_active = false`
- `POST /api/catalog/products`
- `PATCH /api/catalog/products/{id}`
- `DELETE /api/catalog/products/{id}`
- `POST /api/catalog/categories/services`
- `PATCH /api/catalog/categories/services/{id}`
- `POST /api/catalog/categories/products`
- `PATCH /api/catalog/categories/products/{id}`

### RBAC

- Весь admin CRUD scope обгорнути:
  - `AuthMiddlewareFactory`
  - `RoleGuardFactory::admin_only()`

### Результат етапу

- Адміністратор може підтримувати актуальний каталог без ручного редагування коду.

### Етап 5. Домен `orders`: кошик і checkout

### Ціль

Закрити FR-03 і FR-05: кошик, позиції замовлення, контактні дані, фінальна реєстрація замовлення.

### Структура домену

Створити:

- `src/app/domains/orders/mod.rs`
- `src/app/domains/orders/controller.rs`
- `src/app/domains/orders/service.rs`
- `src/app/domains/orders/repository.rs`
- `src/app/domains/orders/models/request.rs`
- `src/app/domains/orders/models/response.rs`
- `src/app/domains/orders/models/summary.rs`
- `src/app/domains/orders/models/newtype.rs`

### Моделі

У `newtype.rs`:

- `ContactFullName`
- `PhoneNumber`
- `EmailAddress`
- `PostalAddress`
- `OrderNumber`
- `OrderStatusCode`

У `request.rs`:

- `AddServiceToCartRequest`
- `AddProductToCartRequest`
- `UpdateCartItemRequest`
- `CheckoutOrderRequest`
- `CheckoutOrderRequestValid`
- `OrderStatusUpdateRequest`
- `OrderStatusUpdateRequestValid`

У `response.rs`:

- `OrderResponse`
- `OrderItemResponse`
- `OrderStatusHistoryResponse`
- `OrderRow`
- `OrderServiceItemRow`
- `OrderProductItemRow`

У `summary.rs`:

- `OrderSummary`
- `OrderSummaryRow`

### Бізнес-логіка

- Один користувач має максимум один активний `draft order`.
- Додавання товарів оновлює кількість або створює нову позицію.
- Додавання послуг створює окрему service item позицію.
- `checkout`:
  - перевіряє, що кошик не порожній;
  - перевіряє активність позицій каталогу;
  - рахує `total_amount`;
  - генерує `order_number`;
  - переводить статус у `new` або `awaiting_payment` залежно від сценарію оплати;
  - створює первинний запис у `order_status_history`.

### Ендпоінти

- `GET /api/cart`
- `POST /api/cart/services`
- `POST /api/cart/products`
- `PATCH /api/cart/items/{id}`
- `DELETE /api/cart/items/{id}`
- `POST /api/orders/checkout`
- `GET /api/orders/my`
- `GET /api/orders/{id}`
- `PATCH /api/orders/{id}/cancel`

### Транзакції

Усі write-операції, де змінюється:

- `orders`
- `order_service_items`
- `order_product_items`
- `order_status_history`

треба обгортати в транзакцію.

### Результат етапу

- Користувач може зібрати кошик і оформити замовлення.
- Замовлення вже має номер, статус, історію та повний payload для подальших етапів.

### Етап 6. Домен `schedule`

### Ціль

Закрити FR-04 і FR-09: доступні слоти, бронювання дати/часу, графік співробітника.

### Роботи

- Створити `schedule` домен у форматі guide.
- Додати DTO для:
  - пошуку слотів;
  - бронювання слоту;
  - призначення співробітника;
  - завершення виконання етапу.
- У repository реалізувати:
  - пошук зайнятих слотів по даті;
  - пошук підтверджених замовлень на день;
  - створення `appointment`;
  - оновлення `appointment_status`.
- У service реалізувати:
  - генерацію доступних слотів із бізнес-годин;
  - блокування зайнятого слоту під час checkout або ручного підтвердження працівником;
  - призначення `employee_user_id`;
  - переведення замовлення в `confirmed` після успішного резервування;
  - переведення в `in_progress` і `completed` з боку employee flow.

### Ендпоінти

- `GET /api/schedule/slots`
- `POST /api/orders/{id}/appointment`
- `PATCH /api/appointments/{id}/assign-employee`
- `GET /api/employee/appointments/today`
- `PATCH /api/appointments/{id}/complete-stage`

### Результат етапу

- Користувач бачить вільні слоти.
- Після бронювання слот більше недоступний.
- Співробітник бачить свій денний план.

### Етап 7. Домен `payments`

### Ціль

Закрити FR-06: онлайн-оплата, резервування, webhook та повторна синхронізація статусів.

### Роботи

- Розширити конфігурацію параметрами Monobank.
- Створити `payments` домен.
- У `request.rs` описати:
  - `CreatePaymentRequest`
  - `PaymentWebhookRequest`
  - `ReservePaymentRequest` якщо потрібен окремий сценарій
- У `response.rs` описати:
  - `PaymentResponse`
  - `PaymentCheckoutResponse`
  - `PaymentStatusResponse`
- Створити payment integration client:
  - `src/app/integrations/monobank/client.rs`
  - `src/app/integrations/monobank/models.rs`
- У service реалізувати:
  - ініціацію платежу;
  - запис `payments` у статусі `created` або `pending`;
  - оновлення `orders.current_status_id` до `awaiting_payment`;
  - webhook idempotency;
  - оновлення платежу до `paid` або `failed`;
  - перехід замовлення до `confirmed` після успішної оплати, якщо слот уже заброньований або інші бізнес-умови виконані.

### Ендпоінти

- `POST /api/orders/{id}/payments`
- `GET /api/orders/{id}/payments`
- `POST /api/payments/monobank/webhook`
- `POST /api/orders/{id}/reserve`

### Додаткові правила

- У repository зберігати `external_reference` і, за потреби, сирий payload в `JSONB`.
- Webhook не повинен дублювати історію статусів при повторному виклику.
- Помилки зовнішнього API мапити через вже наявний `From<reqwest::Error> for RequestError`.

### Результат етапу

- Замовлення можна оплатити або залишити в резерві.
- Платіжні стани синхронізуються з провайдером.

### Етап 8. Операційний workflow працівника

### Ціль

Закрити FR-07 і FR-08: перегляд нових замовлень, зміна статусів, коментарі, картка замовлення.

### Роботи

- Розширити `orders` summary-модель вкладеними даними:
  - items;
  - payments;
  - appointment;
  - history.
- Реалізувати employee list з фільтрами:
  - статус;
  - дата;
  - пошук по номеру замовлення;
  - пошук по контактних даних.
- Додати `EmployeeOrderStatusUpdateRequest` з коментарем.
- У service контролювати дозволені переходи між статусами.
- Кожну зміну статусу записувати в `order_status_history`.
- Після зміни статусу публікувати `order.status_changed`.

### Ендпоінти

- `GET /api/employee/orders`
- `GET /api/employee/orders/{id}`
- `PATCH /api/employee/orders/{id}/status`
- `POST /api/employee/orders/{id}/comments`

### Результат етапу

- Працівник має повний операційний кабінет для обробки нових замовлень.
- Користувач бачить актуальний статус свого замовлення.

### Етап 9. Домен `users` і RBAC-адміністрування

### Ціль

Закрити FR-11: керування користувачами, ролями і блокуванням.

### Роботи

- Створити `users` домен.
- У `request.rs` описати:
  - `UserRoleUpdateRequest`
  - `UserBlockRequest`
  - `UsersFilterParams`
- У `response.rs`:
  - `UserAdminResponse`
  - `UserRoleResponse`
  - `UserSummary`
- У service реалізувати:
  - зміну ролі;
  - блокування/розблокування;
  - список користувачів;
  - інвалідацію токенів після зміни ролі або блокування.
- Вести `audit_logs` для:
  - зміни ролі;
  - блокування;
  - розблокування;
  - критичних дій адміністратора.

### Ендпоінти

- `GET /api/admin/users`
- `GET /api/admin/users/{id}`
- `PATCH /api/admin/users/{id}/role`
- `PATCH /api/admin/users/{id}/block`
- `PATCH /api/admin/users/{id}/unblock`

### Результат етапу

- Адміністратор реально керує доступами.
- Система відповідає базовому RBAC use case.

### Етап 10. Домен `reports`

### Ціль

Закрити FR-12: звіти щодо оплат і замовлень.

### Роботи

- Створити `reports` домен.
- У `request.rs` додати:
  - `ReportPeriodParams`
  - `OrdersReportFilterParams`
  - `PaymentsReportFilterParams`
- У `response.rs`:
  - `OrdersReportResponse`
  - `PaymentsReportResponse`
  - `StatusAggregationResponse`
- У repository написати аналітичні SQL-запити:
  - кількість замовлень за період;
  - сума оплат за період;
  - зріз за статусами;
  - зріз за категоріями послуг/товарів, якщо потрібно.
- Додати export:
  - CSV як обов'язковий;
  - PDF як окремий use case, якщо вистачає часу в межах роботи.

### Ендпоінти

- `GET /api/reports/orders`
- `GET /api/reports/payments`
- `GET /api/reports/orders/export.csv`
- `GET /api/reports/payments/export.csv`
- `GET /api/reports/orders/export.pdf` за наявності PDF-генератора

### Результат етапу

- Адміністратор може отримувати зрозумілу операційну та фінансову звітність.

### Етап 11. Аудит, доменні події і hardening

### Ціль

Закрити cross-cutting вимоги, без яких система буде функціонально неповною.

### Роботи

- Публікувати доменні події через `EventPublisher`.
- Додати `audit_logs` writer service.
- Додати базові rate-limit або хоча б захист від brute force на auth рівні, якщо є час.
- Звузити CORS з `allow_any_origin()` до конфігурованого списку.
- Додати централізовані константи:
  - order status codes;
  - supported currencies.
- Зафіксувати стратегію резервного копіювання та env-настроювання під Docker.

### Результат етапу

- Критичні адмін- і працівницькі дії журналюються.
- Безпека і конфігурованість стають ближчими до NFR.

### Етап 12. Swagger, тестування і release readiness

### Ціль

Довести бекенд до стану, де ним реально можна користуватись і захищати лабораторну.

### Роботи

- Повністю оновити `app/routes/swagger.rs`:
  - усі handlers у `paths(...)`;
  - усі request/response/summaries у `components(schemas(...))`;
  - теги по доменах.
- Додати тестове покриття:
  - unit-тести для newtype і `TryFrom`;
  - repository integration tests;
  - auth tests;
  - RBAC tests;
  - checkout tests;
  - payment webhook idempotency tests;
  - employee status transition tests.
- Підготувати seed або fixture-дані.
- Перевірити `SQLX_OFFLINE` workflow і збірку Docker image.
- Перевірити повний smoke-сценарій:
  - register;
  - login;
  - browse catalog;
  - create cart;
  - checkout;
  - create payment;
  - webhook updates payment;
  - employee confirms order;
  - employee completes appointment;
  - admin changes role;
  - admin exports report.

### Результат етапу

- Swagger відображає реальний API.
- Система має наскрізний перевірений сценарій.
- Документація і код синхронізовані.

### Конкретизація етапів: SQL, моделі та сигнатури

#### SQL-каркас міграцій

Нижче наведено орієнтовний зміст майбутніх міграцій. Це не абстрактні назви файлів, а реальний стартовий SQL, який можна брати за основу.

##### `0001_init_roles_users.sql`

```sql
CREATE TABLE roles (
    id UUID PRIMARY KEY DEFAULT uuidv7(),
    code TEXT NOT NULL UNIQUE,
    name TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

INSERT INTO roles (code, name)
VALUES
    ('user', 'Користувач'),
    ('employee', 'Працівник'),
    ('admin', 'Адміністратор');

CREATE TABLE users (
    id UUID PRIMARY KEY DEFAULT uuidv7(),
    role_id UUID NOT NULL REFERENCES roles(id),
    email TEXT NOT NULL UNIQUE,
    password_hash TEXT NOT NULL,
    full_name TEXT NOT NULL,
    phone TEXT,
    is_active BOOLEAN NOT NULL DEFAULT TRUE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_users_role_id ON users(role_id);
CREATE INDEX idx_users_email ON users(email);
CREATE INDEX idx_users_is_active ON users(is_active);
```

##### `0002_init_catalog.sql`

```sql
CREATE TABLE service_categories (
    id UUID PRIMARY KEY DEFAULT uuidv7(),
    name TEXT NOT NULL UNIQUE,
    description TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE services (
    id UUID PRIMARY KEY DEFAULT uuidv7(),
    category_id UUID NOT NULL REFERENCES service_categories(id),
    name TEXT NOT NULL,
    description TEXT,
    base_price NUMERIC(12, 2) NOT NULL CHECK (base_price >= 0),
    duration_minutes INTEGER NOT NULL CHECK (duration_minutes > 0),
    is_active BOOLEAN NOT NULL DEFAULT TRUE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE product_categories (
    id UUID PRIMARY KEY DEFAULT uuidv7(),
    name TEXT NOT NULL UNIQUE,
    description TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE products (
    id UUID PRIMARY KEY DEFAULT uuidv7(),
    category_id UUID NOT NULL REFERENCES product_categories(id),
    sku TEXT NOT NULL UNIQUE,
    name TEXT NOT NULL,
    description TEXT,
    unit_price NUMERIC(12, 2) NOT NULL CHECK (unit_price >= 0),
    stock_qty INTEGER NOT NULL DEFAULT 0 CHECK (stock_qty >= 0),
    is_active BOOLEAN NOT NULL DEFAULT TRUE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_services_category_id ON services(category_id);
CREATE INDEX idx_services_is_active ON services(is_active);
CREATE INDEX idx_services_name ON services(name);

CREATE INDEX idx_products_category_id ON products(category_id);
CREATE INDEX idx_products_is_active ON products(is_active);
CREATE INDEX idx_products_name ON products(name);
```

##### `0003_init_orders.sql`

```sql
CREATE TABLE order_statuses (
    id UUID PRIMARY KEY DEFAULT uuidv7(),
    code TEXT NOT NULL UNIQUE,
    display_name TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

INSERT INTO order_statuses (code, display_name)
VALUES
    ('draft', 'Чернетка'),
    ('new', 'Нове'),
    ('awaiting_payment', 'Очікує оплату'),
    ('confirmed', 'Підтверджено'),
    ('in_progress', 'Виконується'),
    ('completed', 'Завершено'),
    ('needs_revision', 'Потребує уточнення'),
    ('cancelled', 'Скасовано');

CREATE TABLE orders (
    id UUID PRIMARY KEY DEFAULT uuidv7(),
    user_id UUID NOT NULL REFERENCES users(id),
    current_status_id UUID NOT NULL REFERENCES order_statuses(id),
    order_number TEXT NOT NULL UNIQUE,
    contact_name TEXT NOT NULL,
    contact_phone TEXT NOT NULL,
    contact_email TEXT NOT NULL,
    delivery_address TEXT NOT NULL,
    total_amount NUMERIC(12, 2) NOT NULL DEFAULT 0 CHECK (total_amount >= 0),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE order_service_items (
    id UUID PRIMARY KEY DEFAULT uuidv7(),
    order_id UUID NOT NULL REFERENCES orders(id) ON DELETE CASCADE,
    service_id UUID NOT NULL REFERENCES services(id),
    quantity INTEGER NOT NULL CHECK (quantity > 0),
    unit_price NUMERIC(12, 2) NOT NULL CHECK (unit_price >= 0),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE order_product_items (
    id UUID PRIMARY KEY DEFAULT uuidv7(),
    order_id UUID NOT NULL REFERENCES orders(id) ON DELETE CASCADE,
    product_id UUID NOT NULL REFERENCES products(id),
    quantity INTEGER NOT NULL CHECK (quantity > 0),
    unit_price NUMERIC(12, 2) NOT NULL CHECK (unit_price >= 0),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE order_status_history (
    id UUID PRIMARY KEY DEFAULT uuidv7(),
    order_id UUID NOT NULL REFERENCES orders(id) ON DELETE CASCADE,
    status_id UUID NOT NULL REFERENCES order_statuses(id),
    changed_by_user_id UUID REFERENCES users(id),
    comment TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    changed_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_orders_user_id ON orders(user_id);
CREATE INDEX idx_orders_current_status_id ON orders(current_status_id);
CREATE INDEX idx_orders_created_at ON orders(created_at DESC);
CREATE INDEX idx_order_service_items_order_id ON order_service_items(order_id);
CREATE INDEX idx_order_product_items_order_id ON order_product_items(order_id);
CREATE INDEX idx_order_status_history_order_id ON order_status_history(order_id);
```

##### `0004_init_appointments.sql`

```sql
CREATE TABLE employee_availability (
    id UUID PRIMARY KEY DEFAULT uuidv7(),
    employee_user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    starts_at TIMESTAMPTZ NOT NULL,
    ends_at TIMESTAMPTZ NOT NULL,
    is_active BOOLEAN NOT NULL DEFAULT TRUE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    CHECK (ends_at > starts_at)
);

CREATE TABLE appointments (
    id UUID PRIMARY KEY DEFAULT uuidv7(),
    order_id UUID NOT NULL UNIQUE REFERENCES orders(id) ON DELETE CASCADE,
    employee_user_id UUID REFERENCES users(id),
    scheduled_at TIMESTAMPTZ NOT NULL,
    location TEXT NOT NULL,
    appointment_status TEXT NOT NULL DEFAULT 'reserved',
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_employee_availability_employee_user_id
    ON employee_availability(employee_user_id);
CREATE INDEX idx_employee_availability_starts_at
    ON employee_availability(starts_at);
CREATE INDEX idx_appointments_scheduled_at
    ON appointments(scheduled_at);
CREATE INDEX idx_appointments_employee_user_id
    ON appointments(employee_user_id);
```

##### `0005_init_payments.sql`

```sql
CREATE TABLE payments (
    id UUID PRIMARY KEY DEFAULT uuidv7(),
    order_id UUID NOT NULL REFERENCES orders(id) ON DELETE CASCADE,
    provider TEXT NOT NULL,
    payment_status TEXT NOT NULL,
    amount NUMERIC(12, 2) NOT NULL CHECK (amount >= 0),
    currency TEXT NOT NULL DEFAULT 'UAH',
    external_reference TEXT UNIQUE,
    provider_payload JSONB NOT NULL DEFAULT '{}'::jsonb,
    paid_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_payments_order_id ON payments(order_id);
CREATE INDEX idx_payments_payment_status ON payments(payment_status);
CREATE INDEX idx_payments_external_reference ON payments(external_reference);
```

##### `0006_init_audit_logs.sql`

```sql
CREATE TABLE audit_logs (
    id UUID PRIMARY KEY DEFAULT uuidv7(),
    actor_user_id UUID REFERENCES users(id) ON DELETE SET NULL,
    entity_name TEXT NOT NULL,
    entity_id UUID,
    action TEXT NOT NULL,
    details JSONB NOT NULL DEFAULT '{}'::jsonb,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_audit_logs_actor_user_id ON audit_logs(actor_user_id);
CREATE INDEX idx_audit_logs_entity_name_entity_id ON audit_logs(entity_name, entity_id);
CREATE INDEX idx_audit_logs_created_at ON audit_logs(created_at DESC);
```

#### Орієнтовні сигнатури моделей та функцій

Нижче наведено не псевдоопис, а конкретний каркас типів і функцій, який варто прямо переносити в `controller.rs`, `service.rs`, `repository.rs` та `models/*`.

##### `auth`

```rust
// src/app/domains/auth/models/request.rs
#[derive(Debug, Deserialize, ToSchema, Validate)]
pub struct RegisterRequest {
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 8, max = 128))]
    pub password: String,
    #[validate(length(min = 3, max = 255))]
    pub full_name: String,
    pub phone: Option<String>,
}

#[derive(Debug)]
pub struct RegisterRequestValid {
    pub email: String,
    pub password: String,
    pub full_name: String,
    pub phone: Option<String>,
}

#[derive(Debug, Deserialize, ToSchema, Validate)]
pub struct LoginRequest {
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 8, max = 128))]
    pub password: String,
}

#[derive(Debug)]
pub struct LoginRequestValid {
    pub email: String,
    pub password: String,
}

// src/app/domains/auth/models/response.rs
#[derive(Debug, Serialize, ToSchema)]
pub struct AuthUserResponse {
    pub id: Uuid,
    pub email: String,
    pub full_name: String,
    pub role: UserRole,
    pub is_active: bool,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct LoginResponse {
    pub token: String,
    pub user: AuthUserResponse,
}

#[derive(Debug, FromRow)]
pub struct UserRow {
    pub id: Uuid,
    pub email: String,
    pub password_hash: String,
    pub full_name: String,
    pub phone: Option<String>,
    pub role_id: Uuid,
    pub role_code: String,
    pub is_active: bool,
}

// controller.rs
pub async fn register(
    request: web::Json<RegisterRequest>,
    app_data: web::Data<AppData>,
) -> RequestResult<impl Responder>;

pub async fn login(
    request: web::Json<LoginRequest>,
    app_data: web::Data<AppData>,
) -> RequestResult<impl Responder>;

pub async fn logout(
    claims: web::ReqData<Claims>,
    app_data: web::Data<AppData>,
) -> RequestResult<impl Responder>;

pub async fn me(
    claims: web::ReqData<Claims>,
    app_data: web::Data<AppData>,
) -> RequestResult<impl Responder>;

// service.rs
pub async fn register(
    request: RegisterRequestValid,
    ctx: &ServiceContext<'_>,
) -> RequestResult<LoginResponse>;

pub async fn login(
    request: LoginRequestValid,
    ctx: &ServiceContext<'_>,
) -> RequestResult<LoginResponse>;

pub async fn logout(claims: &Claims, ctx: &ServiceContext<'_>) -> RequestResult<()>;
pub async fn get_current_user(id: Uuid, ctx: &ServiceContext<'_>) -> RequestResult<AuthUserResponse>;

// repository.rs
pub async fn create_user<'c, E>(
    request: &RegisterRequestValid,
    password_hash: &str,
    executor: E,
) -> RequestResult<AuthUserResponse>
where
    E: PgExecutor<'c>;

pub async fn find_user_by_email<'c, E>(email: &str, executor: E) -> RequestResult<UserRow>
where
    E: PgExecutor<'c>;

pub async fn find_user_by_id<'c, E>(id: Uuid, executor: E) -> RequestResult<UserRow>
where
    E: PgExecutor<'c>;
```

##### `catalog`

```rust
// models/request.rs
#[derive(Debug, Deserialize, IntoParams)]
#[into_params(parameter_in = Query)]
pub struct CatalogFilterParams {
    pub page: Option<u32>,
    pub per_page: Option<u32>,
    pub search: Option<String>,
    pub category_id: Option<Uuid>,
    pub min_price: Option<f64>,
    pub max_price: Option<f64>,
    pub only_active: Option<bool>,
}

#[derive(Debug, Deserialize, ToSchema, Validate)]
pub struct ServiceCreateRequest {
    pub category_id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub base_price: f64,
    pub duration_minutes: i32,
}

#[derive(Debug)]
pub struct ServiceCreateRequestValid {
    pub category_id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub base_price: f64,
    pub duration_minutes: i32,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct ServiceUpdateRequest {
    pub category_id: Option<Uuid>,
    pub name: Option<String>,
    pub description: Option<String>,
    pub base_price: Option<f64>,
    pub duration_minutes: Option<i32>,
    pub is_active: Option<bool>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct ServiceResponse {
    pub id: Uuid,
    pub category_id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub base_price: f64,
    pub duration_minutes: i32,
    pub is_active: bool,
}

#[derive(Debug, Serialize, FromRow, ToSchema)]
pub struct ProductResponse {
    pub id: Uuid,
    pub category_id: Uuid,
    pub sku: String,
    pub name: String,
    pub description: Option<String>,
    pub unit_price: f64,
    pub stock_qty: i32,
    pub is_active: bool,
}

// controller.rs
pub async fn get_services(
    query: web::Query<CatalogFilterParams>,
    app_data: web::Data<AppData>,
) -> RequestResult<impl Responder>;

pub async fn create_service(
    request: web::Json<ServiceCreateRequest>,
    app_data: web::Data<AppData>,
) -> RequestResult<impl Responder>;

pub async fn update_service(
    id: web::Path<Uuid>,
    request: web::Json<ServiceUpdateRequest>,
    app_data: web::Data<AppData>,
) -> RequestResult<impl Responder>;

// service.rs
pub async fn get_services(
    params: &PaginationParams,
    filters: &CatalogFilterParams,
    ctx: &ServiceContext<'_>,
) -> RequestResult<PaginatedResponse<ServiceResponse>>;

pub async fn create_service(
    request: ServiceCreateRequestValid,
    ctx: &ServiceContext<'_>,
) -> RequestResult<ServiceResponse>;

pub async fn update_service(
    id: Uuid,
    request: ServiceUpdateRequest,
    ctx: &ServiceContext<'_>,
) -> RequestResult<ServiceResponse>;

// repository.rs
pub async fn find_services_paginated<'c, E>(
    params: &PaginationParams,
    filters: &CatalogFilterParams,
    executor: E,
) -> RequestResult<Vec<ServiceResponse>>
where
    E: PgExecutor<'c>;

pub async fn create_service<'c, E>(
    request: &ServiceCreateRequestValid,
    executor: E,
) -> RequestResult<ServiceResponse>
where
    E: PgExecutor<'c>;
```

##### `orders`

```rust
// models/newtype.rs
#[derive(Debug, Clone)]
pub struct ContactFullName(String);

#[derive(Debug, Clone)]
pub struct PhoneNumber(String);

#[derive(Debug, Clone)]
pub struct EmailAddress(String);

#[derive(Debug, Clone)]
pub struct PostalAddress(String);

#[derive(Debug, Clone)]
pub struct OrderNumber(String);

// models/request.rs
#[derive(Debug, Deserialize, ToSchema)]
pub struct AddServiceToCartRequest {
    pub service_id: Uuid,
    pub quantity: i32,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct AddProductToCartRequest {
    pub product_id: Uuid,
    pub quantity: i32,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct CheckoutOrderRequest {
    pub contact_name: String,
    pub contact_phone: String,
    pub contact_email: String,
    pub delivery_address: String,
}

#[derive(Debug)]
pub struct CheckoutOrderRequestValid {
    pub contact_name: ContactFullName,
    pub contact_phone: PhoneNumber,
    pub contact_email: EmailAddress,
    pub delivery_address: PostalAddress,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct EmployeeOrderStatusUpdateRequest {
    pub status_code: String,
    pub comment: Option<String>,
}

// models/response.rs
#[derive(Debug, Serialize, ToSchema)]
pub struct OrderItemResponse {
    pub item_id: Uuid,
    pub item_type: String,
    pub reference_id: Uuid,
    pub title: String,
    pub quantity: i32,
    pub unit_price: f64,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct OrderResponse {
    pub id: Uuid,
    pub order_number: String,
    pub user_id: Uuid,
    pub current_status_code: String,
    pub contact_name: String,
    pub contact_phone: String,
    pub contact_email: String,
    pub delivery_address: String,
    pub total_amount: f64,
    pub items: Vec<OrderItemResponse>,
}

// controller.rs
pub async fn get_cart(
    claims: web::ReqData<Claims>,
    app_data: web::Data<AppData>,
) -> RequestResult<impl Responder>;

pub async fn add_service_to_cart(
    claims: web::ReqData<Claims>,
    request: web::Json<AddServiceToCartRequest>,
    app_data: web::Data<AppData>,
) -> RequestResult<impl Responder>;

pub async fn checkout(
    claims: web::ReqData<Claims>,
    request: web::Json<CheckoutOrderRequest>,
    app_data: web::Data<AppData>,
) -> RequestResult<impl Responder>;

// service.rs
pub async fn get_or_create_draft_order(
    user_id: Uuid,
    ctx: &ServiceContext<'_>,
) -> RequestResult<OrderResponse>;

pub async fn add_service_to_cart(
    user_id: Uuid,
    request: AddServiceToCartRequest,
    ctx: &ServiceContext<'_>,
) -> RequestResult<OrderResponse>;

pub async fn add_product_to_cart(
    user_id: Uuid,
    request: AddProductToCartRequest,
    ctx: &ServiceContext<'_>,
) -> RequestResult<OrderResponse>;

pub async fn checkout(
    user_id: Uuid,
    request: CheckoutOrderRequestValid,
    ctx: &ServiceContext<'_>,
) -> RequestResult<OrderResponse>;

pub async fn change_order_status(
    actor_user_id: Uuid,
    id: Uuid,
    request: EmployeeOrderStatusUpdateRequest,
    ctx: &ServiceContext<'_>,
) -> RequestResult<OrderResponse>;

// repository.rs
pub async fn find_draft_order_by_user_id<'c, E>(
    user_id: Uuid,
    executor: E,
) -> RequestResult<Option<OrderResponse>>
where
    E: PgExecutor<'c>;

pub async fn create_draft_order<'c, E>(user_id: Uuid, executor: E) -> RequestResult<OrderResponse>
where
    E: PgExecutor<'c>;

pub async fn upsert_service_item<'c, E>(
    order_id: Uuid,
    service_id: Uuid,
    quantity: i32,
    executor: E,
) -> RequestResult<()>
where
    E: PgExecutor<'c>;

pub async fn finalize_checkout<'c, E>(
    order_id: Uuid,
    request: &CheckoutOrderRequestValid,
    total_amount: f64,
    status_code: &str,
    order_number: &str,
    executor: E,
) -> RequestResult<OrderResponse>
where
    E: PgExecutor<'c>;
```

##### `schedule`

```rust
// models/request.rs
#[derive(Debug, Deserialize, IntoParams)]
#[into_params(parameter_in = Query)]
pub struct AvailableSlotsQuery {
    pub date_from: String,
    pub date_to: String,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateAppointmentRequest {
    pub scheduled_at: String,
    pub location: String,
}

#[derive(Debug)]
pub struct CreateAppointmentRequestValid {
    pub scheduled_at: DateTime<Utc>,
    pub location: String,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct AssignEmployeeRequest {
    pub employee_user_id: Uuid,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct CompleteAppointmentStageRequest {
    pub appointment_status: String,
    pub comment: Option<String>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct AvailableSlotResponse {
    pub scheduled_at: String,
    pub is_available: bool,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct AppointmentResponse {
    pub id: Uuid,
    pub order_id: Uuid,
    pub employee_user_id: Option<Uuid>,
    pub scheduled_at: String,
    pub location: String,
    pub appointment_status: String,
}

// service.rs
pub async fn get_available_slots(
    query: &AvailableSlotsQuery,
    ctx: &ServiceContext<'_>,
) -> RequestResult<Vec<AvailableSlotResponse>>;

pub async fn create_appointment(
    order_id: Uuid,
    request: CreateAppointmentRequestValid,
    ctx: &ServiceContext<'_>,
) -> RequestResult<AppointmentResponse>;

pub async fn assign_employee(
    id: Uuid,
    employee_user_id: Uuid,
    actor_user_id: Uuid,
    ctx: &ServiceContext<'_>,
) -> RequestResult<AppointmentResponse>;

pub async fn get_employee_day_plan(
    employee_user_id: Uuid,
    day: NaiveDate,
    ctx: &ServiceContext<'_>,
) -> RequestResult<Vec<AppointmentResponse>>;

// repository.rs
pub async fn find_appointments_in_range<'c, E>(
    date_from: DateTime<Utc>,
    date_to: DateTime<Utc>,
    executor: E,
) -> RequestResult<Vec<AppointmentResponse>>
where
    E: PgExecutor<'c>;

pub async fn create_appointment<'c, E>(
    order_id: Uuid,
    scheduled_at: DateTime<Utc>,
    location: &str,
    executor: E,
) -> RequestResult<AppointmentResponse>
where
    E: PgExecutor<'c>;
```

##### `payments`

```rust
// models/request.rs
#[derive(Debug, Deserialize, ToSchema)]
pub struct CreatePaymentRequest {
    pub return_url: Option<String>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct ReservePaymentRequest {
    pub comment: Option<String>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct MonobankWebhookRequest {
    pub invoice_id: String,
    pub reference: String,
    pub status: String,
    pub amount: i64,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct PaymentResponse {
    pub id: Uuid,
    pub order_id: Uuid,
    pub provider: String,
    pub payment_status: String,
    pub amount: f64,
    pub currency: String,
    pub external_reference: Option<String>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct PaymentCheckoutResponse {
    pub id: Uuid,
    pub checkout_url: String,
    pub status: String,
}

// controller.rs
pub async fn create_payment(
    id: web::Path<Uuid>,
    request: web::Json<CreatePaymentRequest>,
    claims: web::ReqData<Claims>,
    app_data: web::Data<AppData>,
) -> RequestResult<impl Responder>;

pub async fn monobank_webhook(
    request: web::Json<MonobankWebhookRequest>,
    app_data: web::Data<AppData>,
) -> RequestResult<impl Responder>;

// service.rs
pub async fn create_payment(
    order_id: Uuid,
    user_id: Uuid,
    request: CreatePaymentRequest,
    ctx: &ServiceContext<'_>,
) -> RequestResult<PaymentCheckoutResponse>;

pub async fn handle_monobank_webhook(
    request: MonobankWebhookRequest,
    ctx: &ServiceContext<'_>,
) -> RequestResult<PaymentResponse>;

// repository.rs
pub async fn create_payment<'c, E>(
    order_id: Uuid,
    provider: &str,
    amount: f64,
    currency: &str,
    external_reference: Option<&str>,
    provider_payload: &Value,
    executor: E,
) -> RequestResult<PaymentResponse>
where
    E: PgExecutor<'c>;

pub async fn find_payment_by_external_reference<'c, E>(
    external_reference: &str,
    executor: E,
) -> RequestResult<PaymentResponse>
where
    E: PgExecutor<'c>;
```

##### `users`

```rust
#[derive(Debug, Deserialize, ToSchema)]
pub struct UserRoleUpdateRequest {
    pub role: UserRole,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct UserBlockRequest {
    pub is_active: bool,
}

#[derive(Debug, Deserialize, IntoParams)]
#[into_params(parameter_in = Query)]
pub struct UsersFilterParams {
    pub page: Option<u32>,
    pub per_page: Option<u32>,
    pub search: Option<String>,
    pub role: Option<UserRole>,
    pub is_active: Option<bool>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct UserAdminResponse {
    pub id: Uuid,
    pub email: String,
    pub full_name: String,
    pub phone: Option<String>,
    pub role: UserRole,
    pub is_active: bool,
}

pub async fn get_users(
    query: web::Query<UsersFilterParams>,
    app_data: web::Data<AppData>,
) -> RequestResult<impl Responder>;

pub async fn update_user_role(
    id: web::Path<Uuid>,
    request: web::Json<UserRoleUpdateRequest>,
    app_data: web::Data<AppData>,
) -> RequestResult<impl Responder>;

pub async fn block_user(
    id: web::Path<Uuid>,
    request: web::Json<UserBlockRequest>,
    app_data: web::Data<AppData>,
) -> RequestResult<impl Responder>;

pub async fn update_user_role(
    actor_user_id: Uuid,
    id: Uuid,
    role: UserRole,
    ctx: &ServiceContext<'_>,
) -> RequestResult<UserAdminResponse>;

pub async fn set_user_active_state(
    actor_user_id: Uuid,
    id: Uuid,
    is_active: bool,
    ctx: &ServiceContext<'_>,
) -> RequestResult<UserAdminResponse>;
```

##### `reports` та `events`

```rust
#[derive(Debug, Deserialize, IntoParams)]
#[into_params(parameter_in = Query)]
pub struct ReportPeriodParams {
    pub date_from: String,
    pub date_to: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct OrdersReportResponse {
    pub total_orders: i64,
    pub total_amount: f64,
    pub draft_orders: i64,
    pub awaiting_payment_orders: i64,
    pub completed_orders: i64,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct PaymentsReportResponse {
    pub total_payments: i64,
    pub paid_amount: f64,
    pub failed_payments: i64,
}

#[derive(Debug, Serialize)]
#[serde(tag = "event_type", content = "payload")]
pub enum DomainEvent {
    OrderCreated { order_id: Uuid, order_number: String },
    OrderStatusChanged { order_id: Uuid, status_code: String },
    PaymentCreated { order_id: Uuid, payment_id: Uuid },
    PaymentUpdated { order_id: Uuid, payment_id: Uuid, status: String },
    AppointmentConfirmed { order_id: Uuid, appointment_id: Uuid },
    UserRoleChanged { user_id: Uuid, role: String },
}

pub async fn get_orders_report(
    query: web::Query<ReportPeriodParams>,
    app_data: web::Data<AppData>,
) -> RequestResult<impl Responder>;

pub async fn get_payments_report(
    query: web::Query<ReportPeriodParams>,
    app_data: web::Data<AppData>,
) -> RequestResult<impl Responder>;

pub async fn build_orders_report(
    params: &ReportPeriodParams,
    ctx: &ServiceContext<'_>,
) -> RequestResult<OrdersReportResponse>;

pub async fn build_payments_report(
    params: &ReportPeriodParams,
    ctx: &ServiceContext<'_>,
) -> RequestResult<PaymentsReportResponse>;

pub async fn publish(event: DomainEvent, redis: &RedisClient) -> RequestResult<String>;

pub async fn create_audit_log<'c, E>(
    actor_user_id: Option<Uuid>,
    entity_name: &str,
    entity_id: Option<Uuid>,
    action: &str,
    details: &Value,
    executor: E,
) -> RequestResult<()>
where
    E: PgExecutor<'c>;
```

## 7. Порядок реалізації по доменах

Щоб не порушувати залежності, домени треба впроваджувати саме в такій послідовності:

1. `auth`
2. `catalog` read
3. `catalog` admin write
4. `orders`
5. `schedule`
6. `payments`
7. employee flow у `orders`
8. `users`
9. `reports`
10. `events` / audit / hardening

Причина такого порядку:

- без `auth` немає RBAC;
- без `catalog` немає кошика;
- без `orders` немає сенсу в `payments` і `schedule`;
- `reports` мають сенс лише коли вже є реальні бізнес-потоки.

## 8. Шаблон робіт для кожного нового домену

Для кожного домену дотримуватись одного і того самого чеклиста з guide:

1. Створити `mod.rs`, `controller.rs`, `service.rs`, `repository.rs`, `models/`.
2. Описати сирі request DTO.
3. Описати `Valid` моделі і `TryFrom`.
4. Додати `newtype.rs`, якщо є доменні типи.
5. Описати response і row-моделі.
6. Додати summary-моделі для detail/list сценаріїв.
7. Написати repository з `E: PgExecutor<'c>`.
8. Написати service з транзакціями.
9. Написати thin controller з `ServiceContext`.
10. Зареєструвати routes.
11. Зареєструвати Swagger.
12. Додати unit/integration tests.

## 9. Definition of Done для всього бекенду

Бекенд можна вважати завершеним лише коли одночасно виконані всі умови:

- є повний набір міграцій і початкових seed-даних;
- усі домени реалізовані за шаблоном guide;
- Swagger повністю описує фактичний API;
- `cargo check`, `cargo test` і запуск із міграціями проходять стабільно;
- користувач може пройти весь сценарій від реєстрації до завершення замовлення;
- працівник може обробити замовлення й змінити статус;
- співробітник бачить свій план і може завершити етап;
- адміністратор керує ролями і каталогом;
- адміністратор може сформувати звіт;
- ключові дії мають аудит або доменні події;
- код не порушує правила `backend_development_guide.md`.

## 10. Найважливіші ризики, які треба контролювати

- Несинхронізованість ролей у вимогах, архітектурних нотатках і коді.
- Відсутність чіткої моделі для вільних слотів, якщо не визначити її до старту schedule-етапу.
- Складність webhook-логіки без idempotency.
- Нечіткі правила переходів статусів між `user`, `employee` і `admin`.
- Розростання контролерів, якщо не тримати бізнес-правила в сервісах.
- Розростання repository без reusable SQL fragments.
- Забута синхронізація Swagger із кодом.

## 11. Практична рекомендація щодо виконання

Реалізовувати не "по ендпоінтах у довільному порядку", а "по завершених бізнес-вертикалях":

1. auth vertical
2. public catalog vertical
3. admin catalog vertical
4. cart + checkout vertical
5. appointment vertical
6. payment vertical
7. employee processing vertical
8. user admin vertical
9. reporting vertical

Такий порядок найкраще узгоджується і з поточним каркасом, і з `backend_development_guide.md`, і з предметною областю CrematoryRs.
