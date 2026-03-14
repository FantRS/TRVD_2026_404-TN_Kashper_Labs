# Nativia Backend — Повний аналіз архітектури та інструкція з розробки

## Зміст

1. [Загальний огляд](#1-загальний-огляд)
2. [Технологічний стек](#2-технологічний-стек)
3. [Файлова структура проєкту](#3-файлова-структура-проєкту)
4. [Архітектурні шари та їх відповідальність](#4-архітектурні-шари-та-їх-відповідальність)
5. [Модельний шар (models/)](#5-модельний-шар-models)
6. [Newtype-патерн та валідація на рівні типів](#6-newtype-патерн-та-валідація-на-рівні-типів)
7. [Request → Valid конверсія (TryFrom)](#7-request--valid-конверсія-tryfrom)
8. [Контролери (controller.rs)](#8-контролери-controllerrs)
9. [Сервіси (service.rs)](#9-сервіси-servicers)
10. [Репозиторії (repository.rs)](#10-репозиторії-repositoryrs)
11. [Обробка помилок (RequestError)](#11-обробка-помилок-requesterror)
12. [ServiceContext та AppData](#12-servicecontext-та-appdata)
13. [Маршрутизація та Middleware](#13-маршрутизація-та-middleware)
14. [Swagger / OpenAPI (utoipa)](#14-swagger--openapi-utoipa)
15. [Redis: клієнт, події, whitelist](#15-redis-клієнт-події-whitelist)
16. [Доменні події (Event-Driven)](#16-доменні-події-event-driven)
17. [Пагінація та фільтрація](#17-пагінація-та-фільтрація)
18. [Ідіоматичний Rust: трейти та конверсії](#18-ідіоматичний-rust-трейти-та-конверсії)
19. [Правила написання коду](#19-правила-написання-коду)
20. [Чеклист додавання нового домену](#20-чеклист-додавання-нового-домену)
21. [Антипатерни та що уникати](#21-антипатерни-та-що-уникати)

---

## 1. Загальний огляд

Бекенд Nativia — це production-grade REST API, побудований на **Actix-Web 4** з **PostgreSQL** (через sqlx) та **Redis** (через deadpool-redis). Архітектура слідує **domain-grouped** підходу, де кожен бізнес-домен інкапсульований у власній директорії з чітким розділенням на шари: controller → service → repository.

### Ключові архітектурні рішення

- **Domain-grouped file structure** — кожен домен (students, teachers, groups...) є самодостатнім модулем
- **Type-level validation** — валідація через newtype-структури з `TryFrom`, а не рантайм-перевірки в сервісах
- **Request → Valid pattern** — кожен вхідний запит конвертується у валідну структуру до потрапляння в сервіс
- **Centralized error handling** — єдиний `RequestError` enum з `From` імплементаціями для всіх джерел помилок
- **ServiceContext** — lightweight reference-based контекст замість передачі `AppData` напряму
- **Event-driven notifications** — доменні події публікуються в Redis Streams
- **JWT whitelist** — токени зберігаються в Redis для можливості інвалідації

---

## 2. Технологічний стек

| Категорія | Бібліотека | Призначення |
|---|---|---|
| **Web framework** | `actix-web 4` | HTTP сервер, маршрутизація, middleware |
| **Async runtime** | `tokio` | Асинхронний рантайм |
| **Database** | `sqlx 0.8` (Postgres) | Типобезпечні SQL-запити, міграції |
| **Cache/Pub-Sub** | `deadpool-redis` | Connection pooling для Redis |
| **Auth** | `jsonwebtoken`, `argon2`, `hmac`, `sha2` | JWT, хешування паролів, HMAC верифікація |
| **Validation** | `validator` + custom newtypes | Валідація вхідних даних |
| **Serialization** | `serde`, `serde_json`, `serde_qs` | JSON, query string парсинг |
| **API Docs** | `utoipa` + `utoipa-swagger-ui` | OpenAPI/Swagger автогенерація |
| **Logging** | `tracing` + `tracing-subscriber` | Структуроване логування |
| **Error handling** | `thiserror`, `anyhow` | Типізовані помилки (app) та anyhow (infra) |
| **Config** | `config` + `dotenvy` | Конфігурація з env-змінних |
| **IDs** | `uuid` (v4, v7) | Унікальні ідентифікатори |
| **Date/Time** | `chrono` | Робота з датами та часом |
| **Phone** | `phonenumber` | Валідація телефонних номерів |

---

## 3. Приблизна файлова структура проєкту

```
backend/
├── migrations/                    # SQL міграції (sqlx)
│   ├── 0001_init_someone.sql
│   ├── 0002_init_*.sql
│   └── ...
├── src/
│   ├── main.rs                    # Точка входу (мінімальна)
│   ├── lib.rs                     # Ініціалізація: config, DB, Redis, server
│   ├── core/                      # Інфраструктурний шар
│   │   ├── mod.rs
│   │   ├── app_config.rs          # Конфігурація з env (serde + config crate)
│   │   ├── app_data.rs            # Builder pattern для AppData
│   │   ├── server.rs              # Actix HttpServer setup
│   │   ├── logger.rs              # Tracing subscriber (stdout/stderr split)
│   │   └── pg_connector.rs        # PostgreSQL connection pool
│   └── app/                       # Аплікаційний шар
│       ├── mod.rs
│       ├── domains/               # === БІЗНЕС-ДОМЕНИ (приклади) ===
│       │   ├── mod.rs
│       │   ├── students/          # Приклад повного домену
│       │   │   ├── mod.rs
│       │   │   ├── controller.rs  # HTTP handlers
│       │   │   ├── service.rs     # Бізнес-логіка
│       │   │   ├── repository.rs  # SQL-запити
│       │   │   └── models/        # Типи даних
│       │   │       ├── mod.rs     # Re-exports
│       │   │       ├── request.rs # Вхідні DTO + Valid-структури
│       │   │       ├── response.rs# Вихідні DTO + Row-маппінги
│       │   │       ├── newtype.rs # Доменні newtype (BirthDate, PhoneNumber)
│       │   │       └── summary.rs # Агреговані відповіді
│       │   ├── teachers/
│       │   ├── groups/
│       │   ├── auth/
│       │   ├── lessons/
│       │   ├── bookings/
│       │   ├── schedules/
│       │   ├── parents/
│       │   ├── questionnaires/
│       │   ├── test_results/
│       │   ├── lesson_notes/
│       │   ├── identities/
│       │   └── internals/
│       ├── routes/                # Конфігурація маршрутів
│       │   ├── mod.rs             # configure_all_routes()
│       │   ├── students_routes.rs
│       │   ├── auth_routes.rs
│       │   ├── swagger_ui.rs      # OpenAPI schema + SwaggerUI
│       │   └── ...
│       ├── middlewares/           # Actix middleware
│       │   ├── auth_middleware.rs  # JWT перевірка
│       │   └── role_middleware.rs  # RBAC guard
│       └── utils/                 # Спільні утиліти
│           ├── request_error.rs   # RequestError enum + From impls
│           ├── service_context.rs # ServiceContext<'a>
│           ├── paginaion.rs       # PaginatedResponse, PaginationParams
│           ├── qs_query.rs        # Custom query string extractor
│           ├── jwt.rs             # JWT create/decode
│           └── lesson_time_validation.rs
├── Cargo.toml
├── Dockerfile
└── .env.example
```

### Принцип організації

Кожен домен — це **самодостатній модуль** зі структурою:

```
domain_name/
├── mod.rs           # pub mod controller; pub mod models; pub mod repository; pub mod service;
├── controller.rs    # HTTP handlers (thin layer)
├── service.rs       # Business logic
├── repository.rs    # Database queries
└── models/
    ├── mod.rs       # Re-exports всіх публічних типів
    ├── request.rs   # Request DTO + Valid structs + TryFrom impls
    ├── response.rs  # Response DTO + Row structs + From impls
    ├── newtype.rs   # Domain newtypes (опціонально)
    └── summary.rs   # Aggregated responses (опціонально)
```

---

## 4. Архітектурні шари та їх відповідальність

### Потік даних

```
HTTP Request
    ↓
[Route] → вибір handler
    ↓
[Middleware] → auth, role guard
    ↓
[Controller] → десеріалізація, .try_into() валідація, виклик service, логування
    ↓
[Service] → бізнес-логіка, транзакції, виклик repository, публікація подій
    ↓
[Repository] → SQL-запити, маппінг Row → Response через From
    ↓
HTTP Response (JSON)
```

### Чітке розділення відповідальності

| Шар | Відповідальність | НЕ робить |
|---|---|---|
| **Controller** | Десеріалізація, валідація (try_into), логування результату, HTTP-статуси | Бізнес-логіку, SQL-запити |
| **Service** | Бізнес-правила, транзакції, оркестрація репозиторіїв, публікація подій | HTTP-специфіку, прямі SQL-запити |
| **Repository** | SQL-запити, маппінг рядків БД у Rust-структури | Бізнес-логіку, HTTP |
| **Models** | Визначення типів, валідація через TryFrom, конверсії через From | Логіку, запити |

---

## 5. Модельний шар (models/)

### 5.1. Request-моделі (request.rs)

Моделі, що приходять з фронтенду. Завжди мають `Deserialize` + `ToSchema`.

```rust
// DTO з фронтенду — "сирі" дані
#[derive(Debug, ToSchema, Deserialize)]
pub struct StudentCreateRequest {
    pub first_name: String,
    pub last_name: String,
    pub phone_number: Option<String>,
    pub birth_date: String,           // Рядок! Не NaiveDate
    pub is_tada_employee: bool,
}
```

**Правила:**
- Поля мають примітивні типи (`String`, `Option<String>`, `bool`, `Uuid`)
- Дати приходять як `String` (парсяться в newtype)
- Derive: `Debug, Deserialize, ToSchema` (для Swagger)
- Для query params: `Deserialize, IntoParams` з `#[into_params(parameter_in = Query)]`
- Для валідації через `validator` crate: додати `Validate` derive + атрибути `#[validate(...)]`

### 5.2. Valid-моделі (request.rs)

Валідовані версії request-моделей з доменними newtype-типами.

```rust
// Валідована структура — доменні типи замість примітивів
#[derive(Debug)]
pub struct StudentCreateRequestValid {
    pub first_name: String,
    pub last_name: String,
    pub phone_number: Option<PhoneNumber>,  // Newtype!
    pub birth_date: BirthDate,              // Newtype!
    pub is_tada_employee: bool,
}
```

**Правила:**
- НЕ мають `Deserialize` / `ToSchema` — це внутрішні типи
- Мають лише `Debug`
- Містять доменні newtype замість примітивів
- Конвертуються з Request через `TryFrom`

### 5.3. Response-моделі (response.rs)

Моделі для відповіді сервера та маппінгу з БД.

```rust
// Відповідь клієнту
#[derive(Debug, ToSchema, Serialize)]
pub struct StudentResponse {
    pub id: Uuid,
    pub first_name: String,
    // ...
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent: Option<ParentResponse>,
}

// Маппінг рядка з БД (sqlx::FromRow)
#[derive(Debug, FromRow)]
pub struct StudentRow {
    pub id: Uuid,
    pub first_name: String,
    // ...
}

// Конверсія Row → Response через From
impl From<StudentRow> for StudentResponse {
    fn from(value: StudentRow) -> Self { /* ... */ }
}
```

**Правила:**
- Response: `Debug, Serialize, ToSchema`
- Row: `Debug, FromRow` — внутрішній тип для sqlx
- Конверсія Row → Response через `impl From<Row> for Response`
- Використовувати `#[serde(skip_serializing_if = "Option::is_none")]` для опціональних полів
- Використовувати `#[sqlx(skip)]` для полів, яких немає в SQL-запиті
- Використовувати `#[sqlx(default)]` для полів з дефолтним значенням

### 5.4. Summary-моделі (summary.rs)

Агреговані відповіді з вкладеними даними з кількох таблиць. Використовуються для уникнення додаткових REST-запитів.

```rust
// Агрегована відповідь з вкладеними об'єктами
#[derive(Debug, Serialize, FromRow, ToSchema)]
pub struct StudentSummary {
    pub id: Uuid,
    pub first_name: String,
    // ... базові поля студента ...

    #[sqlx(skip)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent: Option<ParentResponse>,

    #[sqlx(skip)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub questionnaire: Option<QuestionnaireSummary>,

    #[sqlx(skip)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_group: Option<CurrentGroupSummary>,

    /// Total count for pagination (window function)
    #[sqlx(default)]
    #[serde(skip)]
    pub total_count: Option<i64>,
}

// Flat Row для маппінгу з SQL JOIN-запиту
#[derive(Debug, FromRow)]
pub struct StudentSummaryRow {
    // Student fields
    pub id: Uuid,
    // ... всі поля з префіксами для кожної таблиці ...
    pub parent_id: Option<Uuid>,
    pub parent_first_name: Option<String>,
    pub questionnaire_id: Option<Uuid>,
    // ...
    pub total_count: Option<i64>,
}

// Конверсія SummaryRow → Summary через From
impl From<StudentSummaryRow> for StudentSummary {
    fn from(row: StudentSummaryRow) -> Self {
        // Збирання вкладених об'єктів з Option-полів через if let
        let parent = if let (Some(id), Some(first_name), ...) = (...) {
            Some(ParentResponse { ... })
        } else {
            None
        };
        // ...
    }
}
```

**Правила:**
- Використовується для list + detail endpoints (один тип для обох)
- `SummaryRow` — flat-структура з префіксованими полями для SQL JOIN
- Конверсія `SummaryRow → Summary` через `From` з pattern matching на `Option` кортежах
- `total_count` — для пагінації через `COUNT(*) OVER()` window function
- Вкладені об'єкти (`parent`, `questionnaire`, etc.) мають `#[sqlx(skip)]` + `#[serde(skip_serializing_if = "Option::is_none")]`

### 5.5. Newtype-моделі (newtype.rs)

Доменні обгортки для валідації на рівні типів.

```rust
#[derive(Debug, Clone)]
pub struct BirthDate(NaiveDate);

impl BirthDate {
    pub fn into_inner(self) -> NaiveDate { self.0 }
}

impl TryFrom<String> for BirthDate {
    type Error = RequestError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let date = NaiveDate::parse_from_str(&value, "%d-%m-%Y")
            .map_err(|e| RequestError::BadRequest(e.to_string()))?;

        // Бізнес-валідація
        if date > today { return Err(RequestError::BadRequest(...)); }
        if date < min_date { return Err(RequestError::BadRequest(...)); }

        Ok(BirthDate(date))
    }
}

impl AsRef<NaiveDate> for BirthDate {
    fn as_ref(&self) -> &NaiveDate { &self.0 }
}
```

**Правила:**
- Tuple struct з одним полем: `pub struct TypeName(InnerType);`
- `into_inner(self)` — consuming accessor
- `AsRef<InnerType>` — borrowing accessor
- `TryFrom<String>` з `type Error = RequestError` — валідація при конверсії
- `Display` — якщо потрібно форматування
- Вся бізнес-валідація — в `TryFrom`, НЕ в сервісі

---

## 6. Newtype-патерн та валідація на рівні типів

### Філософія

> **Якщо значення пройшло через newtype — воно гарантовано валідне.**
> Сервіс ніколи не отримує невалідні дані.

### Приклади newtype у проєкті

| Newtype | Домен | Валідація |
|---|---|---|
| `BirthDate` | students | Формат дати, не в майбутньому, мін. вік 5 років, макс. 120 |
| `PhoneNumber` | students, teachers | Парсинг через `phonenumber` crate (UA) |
| `EnglishLevel` | groups | Enum A1-C2, case-insensitive парсинг |
| `DayOfWeek` | schedules | Парсинг EN/UA назв днів тижня |
| `Role` | teachers | Whitelist: "teacher", "manager", "dev" |
| `LessonStatus` | lessons | Enum + sqlx Type/Encode/Decode |

### Патерн реалізації newtype

```rust
#[derive(Debug, Clone)]
pub struct MyNewtype(InnerType);

impl MyNewtype {
    /// Consuming accessor — забирає ownership
    pub fn into_inner(self) -> InnerType { self.0 }
}

/// Валідація при створенні
impl TryFrom<String> for MyNewtype {
    type Error = RequestError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        // 1. Парсинг
        // 2. Бізнес-правила
        // 3. Ok(Self(parsed_value))
    }
}

/// Borrowing доступ до внутрішнього значення
impl AsRef<InnerType> for MyNewtype {
    fn as_ref(&self) -> &InnerType { &self.0 }
}

/// Форматування (за потреби)
impl std::fmt::Display for MyNewtype {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
```

### Enum як newtype (EnglishLevel, DayOfWeek, LessonStatus)

```rust
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub enum EnglishLevel { A1, A2, B1, B2, C1, C2 }

impl EnglishLevel {
    pub fn as_str(&self) -> &str { /* match */ }
}

impl TryFrom<String> for EnglishLevel {
    type Error = RequestError;
    fn try_from(value: String) -> Result<Self, Self::Error> { /* match */ }
}

impl std::fmt::Display for EnglishLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
```

Для enum-ів, що зберігаються в PostgreSQL як `TEXT`, додатково реалізувати:
- `sqlx::Type<sqlx::Postgres>`
- `sqlx::Decode<'r, sqlx::Postgres>`
- `sqlx::Encode<'q, sqlx::Postgres>`

(Приклад: `LessonStatus` в `lessons/models/request.rs`)

---

## 7. Request → Valid конверсія (TryFrom)

### Основний патерн

```rust
impl TryFrom<StudentCreateRequest> for StudentCreateRequestValid {
    type Error = RequestError;

    fn try_from(value: StudentCreateRequest) -> Result<Self, Self::Error> {
        // 1. Якщо є validator — спочатку value.validate()?;
        // 2. Конвертація полів через newtype TryFrom
        let result = Self {
            first_name: value.first_name,
            last_name: value.last_name,
            phone_number: value.phone_number.map(PhoneNumber::try_from).transpose()?,
            birth_date: value.birth_date.try_into()?,
            is_tada_employee: value.is_tada_employee,
        };

        Ok(result)
    }
}
```

### Ключові прийоми

**Обов'язкове поле з newtype:**
```rust
birth_date: value.birth_date.try_into()?,
```

**Опціональне поле з newtype:**
```rust
phone_number: value.phone_number.map(PhoneNumber::try_from).transpose()?,
```
> `Option<String>.map(TryFrom).transpose()` → `Result<Option<PhoneNumber>, Error>`

**Validator + newtypes разом:**
```rust
impl TryFrom<StudentRegisterRequest> for StudentRegisterRequestValid {
    type Error = RequestError;

    fn try_from(value: StudentRegisterRequest) -> Result<Self, Self::Error> {
        // 1. Спочатку validator
        value.validate()?;

        // 2. Додаткова бізнес-валідація
        if value.student_info.registrant_type.is_child() && value.parent_info.is_none() {
            return Err(RequestError::BadRequest(
                "The child must have a parent/guardian".to_string(),
            ));
        }

        // 3. Конвертація вкладених структур
        let result = Self {
            init_data: value.init_data,
            student_info: value.student_info.try_into()?,
            parent_info: value.parent_info.map(TryFrom::try_from).transpose()?,
        };

        Ok(result)
    }
}
```

### Де викликається try_into()

**Завжди в контролері**, до виклику сервісу:

```rust
pub async fn update_student(
    id: web::Path<Uuid>,
    request: web::Json<StudentUpdateRequest>,
    app_data: web::Data<AppData>,
) -> RequestResult<impl Responder> {
    let id = id.into_inner();
    let request = request.into_inner().try_into()?;  // ← Валідація тут!

    let ctx = ServiceContext::from(app_data.get_ref());
    let response = service::update_student(id, request, &ctx).await;
    // ...
}
```

---

## 8. Контролери (controller.rs)

### Шаблон контролера

```rust
/// Опис ендпоінту (коментар для Swagger)
#[utoipa::path(
    get,                                    // HTTP метод
    path = "/api/students/{id}",            // Шлях
    params(
        ("id" = Uuid, Path, description = "Student ID")
    ),
    responses(
        (status = 200, description = "Student found", body = StudentSummary),
        (status = 404, description = "Student not found"),
    ),
    tag = "Students",
    security(("bearer_auth" = []))          // Якщо потрібна авторизація
)]
#[tracing::instrument(
    name = "get_student",                   // Назва span
    skip_all,                               // Не логувати аргументи
    fields(
        request_id = %Uuid::new_v4(),       // Унікальний ID запиту
        student_id = %id                    // Контекстні поля
    )
)]
pub async fn get_student(
    id: web::Path<Uuid>,
    app_data: web::Data<AppData>,
) -> RequestResult<impl Responder> {
    // 1. Витягнути дані з extractors
    let id = id.into_inner();

    // 2. Створити ServiceContext
    let ctx = ServiceContext::from(app_data.get_ref());

    // 3. Викликати сервіс
    let response = service::get_student(id, &ctx).await;

    // 4. Залогувати результат
    match &response {
        Ok(_) => tracing::info!("Student successfully received!"),
        Err(e) => tracing::error!("Error during student receiving: {}", e),
    };

    // 5. Повернути відповідь
    Ok(HttpResponse::Ok().json(response?))
}
```

### Правила контролерів

1. **Тонкий шар** — лише десеріалізація, валідація, виклик сервісу, логування
2. **Завжди `#[utoipa::path(...)]`** — для Swagger документації
3. **Завжди `#[tracing::instrument(...)]`** — з `skip_all` та `request_id`
4. **Завжди логувати результат** — `match &response { Ok => info, Err => error }`
5. **Валідація через `.try_into()?`** — для request body з newtype-полями
6. **`ServiceContext::from(app_data.get_ref())`** — створення контексту
7. **Повертати `RequestResult<impl Responder>`** — уніфікований тип
8. **HTTP статуси:**
   - `HttpResponse::Ok()` — GET, PATCH, DELETE
   - `HttpResponse::Created()` — POST (створення)

### Extractors

```rust
// Path параметр
id: web::Path<Uuid>

// JSON body
request: web::Json<StudentUpdateRequest>

// Query params (стандартний)
query: web::Query<PaginationParams>

// Query params (кастомний, non-strict)
query: QsQuery<StudentFilterParams>

// AppData
app_data: web::Data<AppData>

// Claims з middleware
claims: web::ReqData<Claims>
```

---

## 9. Сервіси (service.rs)

### Шаблон сервісу

```rust
pub async fn create_entity(
    request: EntityCreateRequestValid,      // Вже валідований!
    ctx: &ServiceContext<'_>,
) -> RequestResult<EntityResponse> {
    // 1. Почати транзакцію (якщо кілька операцій)
    let mut tx = ctx.db_pool.begin().await?;

    // 2. Бізнес-логіка та виклики репозиторіїв
    let entity = entity_repository::create(request, &mut *tx).await?;

    // 3. Додаткові операції в тій же транзакції
    related_repository::create(entity.id, data, &mut *tx).await?;

    // 4. Комміт транзакції
    tx.commit().await?;

    // 5. Публікація доменної події (після коміту!)
    ctx.event_publisher()
        .publish(DomainEvent::EntityCreated(payload))
        .await;

    Ok(entity)
}
```

### Правила сервісів

1. **Приймає лише Valid-структури** — ніколи сирі Request
2. **Бізнес-логіка тут** — перевірки, умови, оркестрація
3. **Транзакції** — `ctx.db_pool.begin().await?` + `tx.commit().await?`
4. **Передача executor** — `&mut *tx` для операцій в транзакції
5. **Публікація подій** — після коміту транзакції
6. **Повертає `RequestResult<T>`** — уніфікована обробка помилок
7. **Може викликати кілька репозиторіїв** — оркестрація
8. **Не знає про HTTP** — жодних HttpResponse, StatusCode

### Приклад з транзакцією та подією

```rust
pub async fn student_register(
    request: StudentRegisterRequestValid,
    ctx: &ServiceContext<'_>,
) -> RequestResult<LoginResponse> {
    let mut tx = ctx.db_pool.begin().await?;

    // Перевірка існування
    let existing = students_repository::get_by_provider_id("telegram", &uid, &mut *tx).await;
    if existing.is_ok() {
        return Err(RequestError::BadRequest("Already registered".to_string()));
    }

    // Створення в транзакції
    let student = students_repository::create_student(data, &mut *tx).await?;
    students_repository::create_identity(student.id, tg_id, &mut *tx).await?;

    if let Some(parent_data) = parent_info {
        parents_repository::create_parent(student.id, parent_data.into(), &mut *tx).await?;
    }

    tx.commit().await?;

    // Подія після коміту
    ctx.event_publisher()
        .publish(DomainEvent::StudentRegistered(payload))
        .await;

    // JWT
    let (token, claims) = jwt::create_jwt(student.id, UserRole::Student, ctx.jwt_secret)?;
    token_whitelist_service::add_to_whitelist(&claims, ctx).await?;

    Ok(LoginResponse { token })
}
```

---

## 10. Репозиторії (repository.rs)

### Шаблон репозиторію

```rust
pub async fn find_by_id<'c, E>(id: Uuid, executor: E) -> RequestResult<EntityResponse>
where
    E: PgExecutor<'c>,
{
    sqlx::query_as::<_, EntityRow>(
        r#"
        SELECT id, name, created_at, updated_at
        FROM entities
        WHERE id = $1
        "#,
    )
    .bind(id)
    .fetch_one(executor)
    .await
    .map(|row| row.into())          // Row → Response через From
    .map_err(RequestError::from)     // sqlx::Error → RequestError через From
}
```

### Правила репозиторіїв

1. **Тільки SQL** — жодної бізнес-логіки
2. **Generic executor** — `E: PgExecutor<'c>` для підтримки і pool, і transaction
3. **Повертає `RequestResult<T>`** — через `From<sqlx::Error>`
4. **Row → Response через `.map(|row| row.into())`** — From trait
5. **Error через `.map_err(RequestError::from)`** або `.map_err(From::from)`
6. **Bind newtype** — `student.birth_date.as_ref()` або `.into_inner()`

### Dynamic updates з QueryBuilder

```rust
pub async fn update_entity(
    id: Uuid,
    request: EntityUpdateRequestValid,
    pool: &sqlx::PgPool,
) -> RequestResult<EntityResponse> {
    let mut query_builder: QueryBuilder<Postgres> = QueryBuilder::new("UPDATE entities SET ");
    let mut separated = query_builder.separated(", ");

    if let Some(name) = request.name {
        separated.push("name = ");
        separated.push_bind_unseparated(name);
    }

    if let Some(phone) = request.phone_number {
        separated.push("phone_number = ");
        separated.push_bind_unseparated(phone.into_inner());
    }

    // Завжди оновлювати timestamp
    separated.push("updated_at = CURRENT_TIMESTAMP");

    query_builder.push(" WHERE id = ");
    query_builder.push_bind(id);
    query_builder.push(" RETURNING id");

    let updated_id: Uuid = query_builder.build_query_scalar().fetch_one(pool).await?;

    // Повернути повну сутність
    find_by_id(updated_id, pool).await
}
```

### Пагінований запит з фільтрами (QueryBuilder)

```rust
pub async fn find_all_paginated<'c, E>(
    params: &PaginationParams,
    filters: &FilterParams,
    sort: &SortParams,
    executor: E,
) -> RequestResult<Vec<EntitySummary>>
where
    E: PgExecutor<'c>,
{
    let search_pattern = params.search_pattern();
    let limit = params.per_page_capped() as i64;
    let offset = params.offset() as i64;

    let mut qb: QueryBuilder<Postgres> = QueryBuilder::new(
        r#"SELECT ..., COUNT(*) OVER() as total_count FROM entities e"#
    );

    qb.push(" WHERE 1=1 ");

    // Динамічні фільтри
    if let Some(pattern) = search_pattern {
        qb.push(" AND e.name ILIKE ");
        qb.push_bind(pattern);
    }

    if let Some(is_active) = filters.is_active {
        qb.push(" AND e.is_active = ");
        qb.push_bind(is_active);
    }

    // Сортування
    qb.push(" ORDER BY e.name ASC");

    // Пагінація
    qb.push(" LIMIT ");
    qb.push_bind(limit);
    qb.push(" OFFSET ");
    qb.push_bind(offset);

    let rows: Vec<EntitySummaryRow> = qb.build_query_as().fetch_all(executor).await?;

    Ok(rows.into_iter().map(EntitySummary::from).collect())
}
```

### Reusable SQL fragments

```rust
// Виносити повторювані SELECT-частини в функції
pub fn entity_summary_select() -> &'static str {
    r#"
        e.id,
        e.name,
        e.created_at,
        e.updated_at
    "#
}
```

---

## 11. Обробка помилок (RequestError)

### Структура

```rust
pub type RequestResult<T> = Result<T, RequestError>;

#[derive(Debug, thiserror::Error)]
pub enum RequestError {
    #[error("400 Bad Request. Context: {0}")]
    BadRequest(String),

    #[error("401 Unauthorized. Context: {0}")]
    Unauthorized(String),

    #[error("403 Forbidden. Context: {0}")]
    Forbidden(String),

    #[error("404 Not Found. Context: {0}")]
    NotFound(String),

    #[error("409 Conflict. Context: {0}")]
    Conflict(String),

    #[error("500 Internal Server Error. Context: {0}")]
    InternalServerError(String),
    // ... інші HTTP статуси
}
```

### From імплементації

```rust
impl From<sqlx::Error> for RequestError { /* маппінг DB помилок */ }
impl From<argon2::password_hash::Error> for RequestError { /* → InternalServerError */ }
impl From<jsonwebtoken::errors::Error> for RequestError { /* → Unauthorized */ }
impl From<validator::ValidationErrors> for RequestError { /* → BadRequest */ }
impl From<serde_json::Error> for RequestError { /* → InternalServerError */ }
```

### Маппінг PostgreSQL помилок

`From<sqlx::Error>` розпізнає PostgreSQL error codes:
- `23502` (NOT NULL) → `BadRequest`
- `23503` (FK violation) → `BadRequest`
- `23505` (Unique violation) → `Conflict`
- `23514` (Check constraint) → `BadRequest`
- `42601`, `42P01`, `42703` (SQL errors) → `InternalServerError`
- `08xxx` (Connection errors) → `ServiceUnavailable`
- `40001`, `40P01` (Transaction errors) → `Conflict`
- `RowNotFound` → `NotFound`

### ResponseError trait

```rust
impl ResponseError for RequestError {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::plaintext())
            .body(self.to_string())
    }

    fn status_code(&self) -> StatusCode {
        match *self { /* маппінг variant → StatusCode */ }
    }
}
```

### Правила

1. **Додавати `From` для кожного нового джерела помилок** — щоб працював `?`
2. **Ніколи не `unwrap()` в продакшен-коді** — завжди `?` або `map_err`
3. **Контекстні повідомлення** — `RequestError::BadRequest("Reason".to_string())`
4. **`RequestResult<T>`** — використовувати скрізь замість `Result<T, RequestError>`

---

## 12. ServiceContext та AppData

### AppData (Builder pattern)

```rust
#[derive(Clone)]
pub struct AppData {
    pub db_pool: PgPool,
    pub redis: RedisClient,
    pub jwt_secret: String,
    pub internal_service_token: String,
    pub integrations: IntegrationSettings,
}

// Створення через Builder
let app_data = AppData::builder()
    .with_db_pool(db_pool)
    .with_redis_client(redis_client)
    .with_jwt(config.jwt_secret)
    .with_internal_service_token(config.internal_service_token)
    .with_integrations(config.integrations)
    .build()
    .unwrap();
```

### ServiceContext (Lightweight reference)

```rust
pub struct ServiceContext<'a> {
    pub db_pool: &'a PgPool,
    pub redis: &'a RedisClient,
    pub jwt_secret: &'a str,
    pub integrations: &'a IntegrationSettings,
}

impl<'a> From<&'a AppData> for ServiceContext<'a> {
    fn from(value: &'a AppData) -> Self {
        Self {
            db_pool: &value.db_pool,
            redis: &value.redis,
            jwt_secret: &value.jwt_secret,
            integrations: &value.integrations,
        }
    }
}

impl<'a> ServiceContext<'a> {
    pub fn event_publisher(&self) -> EventPublisher<'_> {
        EventPublisher::new(self.redis)
    }
}
```

### Використання в контролері

```rust
let ctx = ServiceContext::from(app_data.get_ref());
let response = service::do_something(&ctx).await;
```

**Чому ServiceContext, а не AppData напряму:**
- Передає лише references (`&'a`) — zero-cost
- Не клонує PgPool/RedisClient
- Інкапсулює доступ до інфраструктури
- Легко розширювати (додати нове поле = додати reference)

---

## 13. Маршрутизація та Middleware

### Конфігурація маршрутів

```rust
// routes/mod.rs
pub fn configure_all_routes(cfg: &mut web::ServiceConfig) {
    swagger_ui::configure(cfg);

    cfg.service(
        web::scope("/api")
            .configure(auth_routes::configure)
            .configure(students_routes::configure)
            // ...
    );
}

// routes/students_routes.rs
pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/students")
            .wrap(AuthMiddlewareFactory)                    // Auth для всього scope
            .route("", web::get().to(controller::get_students))
            .route("/{id}", web::get().to(controller::get_student))
            .route("/{id}", web::patch().to(controller::update_student))
            .route("/{id}", web::delete().to(controller::delete_student))
    );
}
```

### Middleware стек

**AuthMiddlewareFactory** — JWT верифікація + whitelist:
1. Витягує `Authorization: Bearer <token>` header
2. Декодує JWT через `decode_jwt()`
3. Перевіряє токен в Redis whitelist
4. Додає `Claims` в `req.extensions_mut()`

**RoleGuardFactory** — RBAC:
```rust
// Фабричні методи для зручності
RoleGuardFactory::dev_only()       // [Dev]
RoleGuardFactory::admin_only()     // [Dev, Admin]
RoleGuardFactory::all_teachers()   // [Dev, Admin, Teacher]
RoleGuardFactory::students_only()  // [Student]
```

**ProviderMiddlewareFactory** — для internal service-to-service:
- Перевіряє `internal_service_token` header

### Порядок middleware (важливо!)

```rust
// Actix виконує middleware ЗНИЗУ ВГОРУ при запиті
web::resource("/teacher/register")
    .wrap(RoleGuardFactory::admin_only())  // 2. Перевірка ролі
    .wrap(AuthMiddlewareFactory)           // 1. Спочатку auth
    .route(web::post().to(controller::teacher_register))
```

---

## 14. Swagger / OpenAPI (utoipa)

### Реєстрація в swagger_ui.rs

```rust
#[derive(OpenApi)]
#[openapi(
    info(title = "Nativia API", version = "1.0.0"),
    paths(
        // Додати кожен handler
        students_controller::get_students,
        students_controller::get_student,
        // ...
    ),
    components(
        schemas(
            // Додати кожну модель
            StudentUpdateRequest,
            StudentResponse,
            StudentSummary,
            // ...
        )
    ),
    tags(
        (name = "Students", description = "Student management endpoints"),
        // ...
    ),
    modifiers(&SecurityAddon)
)]
struct ApiDoc;
```

### Правила Swagger

1. **Кожен новий endpoint** → додати в `paths(...)` в `swagger_ui.rs`
2. **Кожна нова Request/Response модель** → додати в `components(schemas(...))`
3. **Кожен новий тег** → додати в `tags(...)`
4. **Controller handler** → `#[utoipa::path(...)]` з повним описом
5. **Request models** → `#[derive(ToSchema)]`
6. **Response models** → `#[derive(ToSchema)]`
7. **Query params** → `#[derive(IntoParams)]` з `#[into_params(parameter_in = Query)]`
8. **Для NaiveTime** → `#[schema(value_type = String, example = "10:00:00")]`

---

## 15. Redis: клієнт, події, whitelist

### RedisClient

Обгортка над `deadpool-redis` з типізованими методами:

```rust
pub struct RedisClient { pool: RedisPool }

impl RedisClient {
    pub async fn get(&self, key: &str) -> RequestResult<Option<String>> { ... }
    pub async fn set_ex(&self, key: &str, value: &str, ttl: u64) -> RequestResult<()> { ... }
    pub async fn del(&self, key: &str) -> RequestResult<()> { ... }
    pub async fn smembers(&self, key: &str) -> RequestResult<Vec<String>> { ... }
    pub async fn xadd(&self, stream_key: &str, fields: &[(&str, &str)]) -> RequestResult<String> { ... }
    pub fn get_pipe(&self) -> Pipeline { ... }
    pub async fn exec_pipe<T: FromRedisValue>(&self, pipeline: &Pipeline) -> RequestResult<T> { ... }
}
```

### Redis Keys (enum)

```rust
pub enum RedisKey {
    WhiteList(String),      // wl:{jti}
    UserTokens(Uuid),       // ut:{user_id}
    NotificationStream,     // notifications:events
}

impl Display for RedisKey { /* format */ }
```

### JWT Whitelist

- `add_to_whitelist()` — SET_EX + SADD (pipeline)
- `verify_in_whitelist()` — GET + role check
- `invalidate_user_tokens()` — SMEMBERS + DEL all (pipeline)
- `invalidate_token()` — DEL + SREM (pipeline)

---

## 16. Доменні події (Event-Driven)

### DomainEvent enum

```rust
#[derive(Debug, Serialize)]
#[serde(tag = "event_type", content = "payload")]
pub enum DomainEvent {
    StudentRegistered(StudentRegisteredPayload),
    LessonRescheduled(LessonEventPayload),
    BookingCreated(BookingEventPayload),
    // ...
}
```

### Кожна подія має:
- **Payload struct** — дані події
- **`scope()`** — кому відправити (Group, User, Role, All)
- **`event_name()`** — рядок для Redis Stream (`"student.registered"`)

### Публікація

```rust
// В сервісі, ПІСЛЯ коміту транзакції
ctx.event_publisher()
    .publish(DomainEvent::StudentRegistered(StudentRegisteredPayload {
        student_id: student.id,
        first_name: student.first_name.clone(),
        last_name: student.last_name.clone(),
    }))
    .await;
```

### EventPublisher

Серіалізує подію в JSON та публікує в Redis Stream через `XADD`.

### Правила подій

1. **Публікувати після коміту** — щоб подія відповідала реальному стану
2. **Не панікувати при помилці публікації** — логувати error, але не зупиняти
3. **Payload містить мінімум даних** — ID + ключові поля для нотифікації
4. **Scope визначає отримувачів** — Group, User, Users, Role, All

---

## 17. Пагінація та фільтрація

### PaginationParams

```rust
#[derive(Debug, Deserialize, IntoParams)]
#[into_params(parameter_in = Query)]
pub struct PaginationParams {
    #[serde(default = "default_page")]     // default: 1
    pub page: u32,
    #[serde(default = "default_per_page")] // default: 20
    pub per_page: u32,
    pub search: Option<String>,
}

impl PaginationParams {
    pub fn per_page_capped(&self) -> u32 { self.per_page.min(100) }
    pub fn offset(&self) -> u32 { (self.page.saturating_sub(1)) * self.per_page_capped() }
    pub fn search_pattern(&self) -> Option<String> { /* %search% */ }
}
```

### PaginatedResponse

```rust
#[derive(Debug, Serialize, ToSchema)]
pub struct PaginatedResponse<T: Serialize> {
    pub data: Vec<T>,
    pub meta: PaginationMeta,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct PaginationMeta {
    pub current_page: u32,
    pub per_page: u32,
    pub total_items: i64,
    pub total_pages: u32,
}
```

### Патерн використання

```rust
// В сервісі
pub async fn get_entities(
    params: &PaginationParams,
    ctx: &ServiceContext<'_>,
) -> RequestResult<PaginatedResponse<EntitySummary>> {
    let rows = repository::find_all_paginated(params, ctx.db_pool).await?;
    let total = rows.first().and_then(|r| r.total_count).unwrap_or(0);

    Ok(PaginatedResponse::new(rows, params, total))
}
```

### QsQuery — кастомний query extractor

```rust
pub struct QsQuery<T>(pub T);

impl<T: DeserializeOwned> FromRequest for QsQuery<T> {
    // Використовує serde_qs з non-strict mode
    // Підтримує масиви в query: ?levels[]=A1&levels[]=B2
}
```

Використовувати `QsQuery` замість `web::Query` коли потрібні:
- Масиви в query params (`?english_levels[]=A1&english_levels[]=B2`)
- Non-strict парсинг (ігнорує невідомі поля)

---

## 18. Ідіоматичний Rust: трейти та конверсії

### Обов'язкові трейти

| Трейт | Де використовувати | Приклад |
|---|---|---|
| `From<A> for B` | Row → Response, infallible конверсії | `impl From<StudentRow> for StudentResponse` |
| `TryFrom<A> for B` | Request → Valid, парсинг з валідацією | `impl TryFrom<StudentCreateRequest> for StudentCreateRequestValid` |
| `Display` | Enum → String для логів/БД | `impl Display for EnglishLevel` |
| `AsRef<T>` | Borrowing доступ до newtype inner | `impl AsRef<NaiveDate> for BirthDate` |
| `ResponseError` | Помилки → HTTP відповіді | `impl ResponseError for RequestError` |
| `FromRequest` | Custom extractors | `impl FromRequest for QsQuery<T>` |

### Ідіоматичні прийоми в проєкті

**1. `.map_err(From::from)` замість `.map_err(|e| RequestError::from(e))`:**
```rust
sqlx::query_scalar("DELETE FROM students WHERE id = $1 RETURNING id")
    .bind(id)
    .fetch_one(executor)
    .await
    .map_err(From::from)
```

**2. `.map(|row| row.into())` замість `.map(|row| Response::from(row))`:**
```rust
.fetch_one(executor)
.await
.map(|inner| inner.into())
.map_err(RequestError::from)
```

**3. `Option::map(TryFrom::try_from).transpose()` для Optional newtype:**
```rust
phone_number: value.phone_number.map(PhoneNumber::try_from).transpose()?,
```

**4. `value.try_into()?` замість `Type::try_from(value)?`:**
```rust
let request = request.into_inner().try_into()?;
```

**5. Pattern matching на кортежах Option для збирання вкладених об'єктів:**
```rust
let parent = if let (Some(id), Some(name), Some(phone)) = (
    row.parent_id,
    row.parent_name,
    row.parent_phone,
) {
    Some(ParentResponse { id, name, phone })
} else {
    None
};
```

**6. `config.try_deserialize().map_err(From::from)` — ланцюжок конверсій:**
```rust
impl AppConfig {
    pub fn configure() -> Result<Self> {
        let config = config::Config::builder()
            .add_source(config::Environment::default())
            .build()?;
        config.try_deserialize().map_err(From::from)
    }
}
```

**7. `From<LogLevel> for EnvFilter` — конверсія конфігурації:**
```rust
impl From<LogLevel> for EnvFilter {
    fn from(value: LogLevel) -> Self {
        match value {
            LogLevel::Info => "info".into(),
            // ...
        }
    }
}
```

**8. Builder pattern для AppData:**
```rust
AppData::builder()
    .with_db_pool(pool)
    .with_redis_client(redis)
    .build()
    .unwrap();
```

---

## 19. Правила написання коду

### 19.1. Загальні правила

1. **Ідіоматичний Rust** — використовувати трейти (`From`, `TryFrom`, `Display`, `AsRef`), а не ad-hoc функції
2. **Жодних `unwrap()` в продакшен-коді** — тільки `?`, `map_err`, `ok_or_else`
3. **`RequestResult<T>`** — єдиний тип результату для всіх шарів
4. **Уникати дублювання** — перевикористовувати код, виносити спільне в utils
5. **Чистити невикористаний код** — видаляти мертвий код, unused imports
6. **Коментарі українською** — для бізнес-логіки
7. **Документація англійською** — для публічних API (utoipa descriptions)

### 19.2. Файлова структура

1. **Один домен = одна директорія** з `mod.rs`, `controller.rs`, `service.rs`, `repository.rs`, `models/`
2. **models/mod.rs** — re-export всіх публічних типів через `pub use`
3. **Нові типи моделей** — додавати у відповідний файл (`request.rs`, `response.rs`, `newtype.rs`, `summary.rs`)
4. **Не створювати файли без потреби** — якщо немає newtype, не створювати `newtype.rs`

### 19.3. Контролери

1. **`#[utoipa::path(...)]`** — обов'язково для кожного handler
2. **`#[tracing::instrument(name = "...", skip_all, fields(request_id = %Uuid::new_v4()))]`** — обов'язково
3. **Логування результату** — `match &response { Ok => info, Err => error }`
4. **Валідація** — `.try_into()?` для request body
5. **ServiceContext** — `ServiceContext::from(app_data.get_ref())`

### 19.4. Сервіси

1. **Приймати Valid-структури** — ніколи сирі Request
2. **Транзакції** — для кількох операцій запису
3. **Публікація подій** — після коміту
4. **Не знати про HTTP** — жодних HttpResponse

### 19.5. Репозиторії

1. **Generic executor** — `E: PgExecutor<'c>`
2. **Row → Response через From** — `.map(|row| row.into())`
3. **QueryBuilder** — для динамічних UPDATE/SELECT з фільтрами
4. **`COUNT(*) OVER()`** — для пагінації (window function)

### 19.6. Моделі

1. **Request** — `Deserialize, ToSchema` (+ `Validate` за потреби)
2. **Valid** — тільки `Debug`, з доменними newtype
3. **Response** — `Serialize, ToSchema`
4. **Row** — `FromRow` (внутрішній тип)
5. **Summary** — `Serialize, FromRow, ToSchema` з `#[sqlx(skip)]` для вкладених
6. **Newtype** — `TryFrom<String>`, `into_inner()`, `AsRef`, `Display`

### 19.7. Swagger

1. **Кожен новий endpoint** → `paths(...)` в `swagger_ui.rs`
2. **Кожна нова модель** → `components(schemas(...))` в `swagger_ui.rs`
3. **Кожен новий тег** → `tags(...)` в `swagger_ui.rs`

### 19.8. Помилки

1. **Новий crate з помилками** → додати `From<CrateError> for RequestError`
2. **Бізнес-помилки** → `RequestError::BadRequest("context".to_string())`
3. **Не знайдено** → `RequestError::NotFound("context".to_string())`
4. **Конфлікт** → `RequestError::Conflict("context".to_string())`

### 19.9. Уникнення бойлерплейту

1. **`pub use` в mod.rs** — для коротких імпортів
2. **Спільні SQL-фрагменти** — виносити в функції (`student_summary_select()`)
3. **`HasTotalCount` trait** — для уніфікованої пагінації
4. **`create_paginated_response()`** — generic helper
5. **`ServiceContext`** — замість передачі кожного поля окремо

---

## 20. Чеклист додавання нового домену

### Крок 1: Створити структуру файлів

```
src/app/domains/new_domain/
├── mod.rs
├── controller.rs
├── service.rs
├── repository.rs
└── models/
    ├── mod.rs
    ├── request.rs
    └── response.rs
```

### Крок 2: Визначити моделі

- [ ] **request.rs** — Request DTO з `Deserialize, ToSchema`
- [ ] **request.rs** — Valid struct з доменними newtype
- [ ] **request.rs** — `impl TryFrom<Request> for Valid`
- [ ] **response.rs** — Response DTO з `Serialize, ToSchema`
- [ ] **response.rs** — Row struct з `FromRow`
- [ ] **response.rs** — `impl From<Row> for Response`
- [ ] **newtype.rs** — (якщо потрібно) Newtype з `TryFrom<String>`
- [ ] **summary.rs** — (якщо потрібно) Summary + SummaryRow + `From`
- [ ] **models/mod.rs** — `pub use` re-exports

### Крок 3: Написати repository

- [ ] CRUD функції з `E: PgExecutor<'c>`
- [ ] QueryBuilder для dynamic updates
- [ ] Пагінований запит з `COUNT(*) OVER()`

### Крок 4: Написати service

- [ ] Бізнес-логіка
- [ ] Транзакції де потрібно
- [ ] Публікація подій

### Крок 5: Написати controller

- [ ] `#[utoipa::path(...)]` для кожного handler
- [ ] `#[tracing::instrument(...)]` для кожного handler
- [ ] `.try_into()?` для валідації
- [ ] Логування результату

### Крок 6: Зареєструвати

- [ ] **domains/mod.rs** — `pub mod new_domain;`
- [ ] **routes/new_domain_routes.rs** — маршрути + middleware
- [ ] **routes/mod.rs** — `mod new_domain_routes;` + `.configure(new_domain_routes::configure)`
- [ ] **swagger_ui.rs** — paths, schemas, tags

### Крок 7: Міграція

- [ ] SQL міграція в `migrations/`

### Крок 8: Доменні події (якщо потрібно)

- [ ] Payload struct в `notifications/events.rs`
- [ ] Variant в `DomainEvent` enum
- [ ] `scope()` та `event_name()` для нового варіанту

---

## 21. Антипатерни та що уникати

### ❌ НЕ робити

1. **Бізнес-логіка в контролері** — контролер тільки десеріалізує, валідує, логує
2. **SQL в сервісі** — тільки в repository
3. **HTTP-специфіка в сервісі** — жодних `HttpResponse`, `StatusCode`
4. **`unwrap()` в продакшен-коді** — завжди `?` або `map_err`
5. **Передача `AppData` в сервіс** — використовувати `ServiceContext`
6. **Дублювання newtype** — якщо `PhoneNumber` вже є в `students`, перевикористати або винести в спільний модуль
7. **Публікація подій до коміту транзакції** — подія має відповідати реальному стану
8. **Забувати додати в Swagger** — кожен endpoint та модель
9. **Забувати логувати** — кожен контролер логує результат
10. **Валідація в сервісі** — валідація через newtype `TryFrom` в контролері
11. **Хардкод конфігурації** — все через env-змінні та `AppConfig`
12. **Створення файлів без потреби** — не створювати `newtype.rs` якщо немає newtype
13. **Ігнорування помилок Redis** — логувати, але не панікувати

### ✅ Робити

1. **Перевикористовувати існуючий код** — newtypes, utils, helpers
2. **Оновлювати код** — якщо знайшов кращий спосіб, оновити
3. **Чистити за собою** — видаляти невикористаний код
4. **Документувати бізнес-логіку** — коментарі українською
5. **Тестувати через Swagger** — `/swagger-ui/`
6. **Використовувати трейти** — `From`, `TryFrom`, `Display`, `AsRef`
7. **Generic executor в repository** — для підтримки транзакцій
8. **Window functions для пагінації** — `COUNT(*) OVER()`
9. **Pipeline для Redis** — batch операції

---

## Додаток: Сильні сторони архітектури

1. **Type-safe validation** — неможливо передати невалідні дані в сервіс
2. **Centralized error handling** — один enum для всіх помилок з автоматичним маппінгом
3. **Domain isolation** — кожен домен самодостатній
4. **Event-driven** — слабка зв'язаність через Redis Streams
5. **Zero-cost abstractions** — `ServiceContext` з references, `From`/`Into` конверсії
6. **Production-ready** — connection pooling, migrations, structured logging, CORS, JWT whitelist
7. **Self-documenting** — Swagger автогенерація з коду
8. **Flexible queries** — `QueryBuilder` для динамічних фільтрів/сортування
9. **Transaction safety** — явні транзакції з `begin()/commit()`
10. **RBAC** — гнучка система ролей через middleware
