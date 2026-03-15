# Lab 3. Звіт по реалізованому бекенду

## 1. Посилання на Git-репозиторій

- Репозиторій: `https://github.com/FantRS/TRVD_2026_404-TN_Kashper_Labs.git`

## 2. Короткий опис реалізованого бекенду

У межах реалізації бекенду було побудовано модульний REST API на `Rust + Actix Web + SQLx + PostgreSQL 18 + Redis`.

Реалізовані основні бізнес-домени:

- `auth`:
  - реєстрація;
  - логін;
  - logout;
  - logout-all;
  - endpoint `me`;
  - JWT + Redis whitelist.
- `catalog`:
  - публічний перегляд послуг і товарів;
  - фільтрація, пошук, пагінація;
  - адміністраторський CRUD для каталогу і категорій.
- `orders`:
  - кошик як `draft`-замовлення;
  - додавання товарів і послуг;
  - checkout;
  - перегляд замовлення;
  - зміна статусів.
- `schedule`:
  - перегляд слотів;
  - створення запису на замовлення;
  - призначення працівника;
  - денний план працівника.
- `payments`:
  - оплата через внутрішній гаманець;
  - автоматичне стартове нарахування `10000` валюти новому користувачу;
  - журнал транзакцій гаманця.
- `users`:
  - перегляд користувачів;
  - зміна ролі;
  - блокування / активація.
- `reports`:
  - агрегований звіт по замовленнях;
  - агрегований звіт по оплатах.

Також були реалізовані:

- SQL-міграції;
- Swagger/OpenAPI документація;
- role-based access control для ролей `user`, `employee`, `admin`;
- health / ready endpoints;
- подієва інтеграція через Redis Stream.

## 3. Технічний стек

- Мова: `Rust`
- Web framework: `Actix Web`
- База даних: `PostgreSQL 18`
- Кеш / технічні дані: `Redis`
- ORM / DB access: `SQLx`
- Документація API: `Swagger UI` через `utoipa`
- Контейнеризація: `Docker Compose`

## 4. Що саме було перевірено

- API успішно відповідає на `GET /health`
- Swagger UI доступний локально
- У згенерованому OpenAPI описано `29` REST endpoint-ів

`gRPC` у цьому бекенді не реалізовувався, тому скріншот виклику методу через gRPC клієнт відсутній.

## 5. Приклади коду

### 5.1. Визначення моделі API (DTO)

Приклад DTO для реєстрації користувача:

```rust
#[derive(Debug, Deserialize, ToSchema)]
pub struct RegisterRequest {
    pub email: String,
    pub password: String,
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

impl TryFrom<RegisterRequest> for RegisterRequestValid {
    type Error = RequestError;

    fn try_from(value: RegisterRequest) -> Result<Self, Self::Error> {
        let password = value.password.trim().to_owned();
        if password.len() < 8 || password.len() > 128 {
            return Err(RequestError::unprocessable_entity(
                "password must contain from 8 to 128 characters",
            ));
        }

        Ok(Self {
            email: normalized_email(&value.email, "email")?,
            password,
            full_name: trimmed_required(&value.full_name, "full_name", 3, 255)?,
            phone: match value.phone {
                Some(phone) => Some(phone_number(&phone, "phone")?),
                None => None,
            },
        })
    }
}
```

### 5.2. Реалізація методу (Controller Action)

Приклад controller action для оплати замовлення через внутрішній гаманець:

```rust
/// Оплачує замовлення з внутрішнього гаманця користувача (`user`).
#[utoipa::path(
    post,
    path = "/api/orders/{id}/payments",
    params(("id" = Uuid, Path, description = "Order id")),
    request_body = CreatePaymentRequest,
    responses((status = 200, body = crate::app::domains::payments::models::PaymentCheckoutResponse)),
    security(("bearer_auth" = [])),
    tag = "Payments"
)]
#[tracing::instrument(name = "create_payment", skip_all, fields(request_id = %Uuid::new_v4(), order_id = %id))]
pub async fn create_payment(
    id: web::Path<Uuid>,
    request: web::Json<CreatePaymentRequest>,
    claims: web::ReqData<Claims>,
    app_data: web::Data<AppData>,
) -> RequestResult<impl Responder> {
    let id = id.into_inner();
    let ctx = ServiceContext::from(app_data.get_ref());
    let response = service::create_payment(id, claims.sub, request.into_inner(), &ctx).await;

    match &response {
        Ok(_) => tracing::info!("Payment created successfully"),
        Err(error) => tracing::error!("Payment creation failed: {error}"),
    }

    Ok(HttpResponse::Ok().json(response?))
}
```

## 6. Приклади реалізованих REST endpoint-ів

- `POST /api/auth/register`
- `POST /api/auth/login`
- `GET /api/auth/me`
- `GET /api/catalog/services`
- `GET /api/catalog/products`
- `GET /api/orders/cart`
- `POST /api/orders/checkout`
- `POST /api/orders/{id}/payments`
- `GET /api/schedule/slots`
- `GET /api/users`
- `GET /api/reports/orders`

## 7. Висновок по реалізації

У проєкті реалізовано повноцінний REST-бекенд із розділенням на домени, сервісний шар, репозиторії, DTO-моделі, Swagger документацію та SQL-міграції. Основні користувацькі сценарії покриті: аутентифікація, робота з каталогом, кошиком, замовленнями, бронюванням, оплатою через внутрішню валюту, адміністрування користувачів і формування звітів.

## 8. Контрольні запитання

### 8.1. Порівняйте REST, GraphQL та gRPC. У яких випадках доцільно використовувати кожен із них?

`REST` добре підходить для класичних вебсервісів, CRUD API, інтеграцій між системами та публічних HTTP API. Він простий, зрозумілий, добре документується через OpenAPI і зручний для фронтенду та сторонніх клієнтів.

`GraphQL` доцільно використовувати тоді, коли клієнтам потрібна гнучкість у виборі полів і складні агрегації даних з кількох джерел. Він особливо корисний для багатих frontend-додатків, де різні екрани потребують різні набори полів.

`gRPC` доцільний для високопродуктивної взаємодії між сервісами, low-latency communication, streaming-сценаріїв та внутрішніх distributed systems. Найчастіше його застосовують у microservices або для взаємодії backend-backend.

### 8.2. Що таке проблема Over-fetching та Under-fetching даних? Як GraphQL вирішує ці проблеми порівняно з REST?

`Over-fetching` означає, що клієнт отримує більше даних, ніж йому потрібно. Наприклад, REST endpoint повертає повний об’єкт користувача, хоча клієнту потрібні лише ім’я та email.

`Under-fetching` означає, що одного запиту недостатньо і клієнт змушений робити кілька запитів, щоб зібрати всі потрібні дані.

`GraphQL` вирішує обидві проблеми тим, що клієнт сам описує точну структуру потрібних даних у запиті. Тобто він отримує тільки необхідні поля й може в одному запиті запитати пов’язані сутності.

### 8.3. Що таке Protocol Buffers (Protobuf)? Які його переваги перед JSON?

`Protocol Buffers` — це компактний бінарний формат серіалізації даних, який описується через `.proto` файли.

Переваги `Protobuf` перед `JSON`:

- менший розмір повідомлень;
- швидша серіалізація та десеріалізація;
- строго типізована схема;
- краща продуктивність для міжсервісної взаємодії;
- зручна генерація коду для різних мов програмування.

`JSON` простіший для людини, але зазвичай програє `Protobuf` у швидкості та компактності.

### 8.4. Як забезпечується версіонування API у REST та GraphQL?

У `REST` версіонування часто робиться через URI, наприклад `/api/v1/...`, або через заголовки. Це дозволяє одночасно підтримувати кілька версій контракту.

У `GraphQL` частіше використовують еволюцію схеми без явного versioned endpoint: нові поля додають, старі позначають як `deprecated`, а клієнти поступово мігрують. Тобто замість жорсткого `v1 / v2` частіше застосовується керована зміна схеми.

### 8.5. Поясніть концепцію Idempotency (ідемпотентності). Які методи/операції мають бути ідемпотентними?

Ідемпотентність означає, що повторне виконання однієї й тієї ж операції з тим самим вхідним станом не змінює результат після першого застосування.

У REST ідемпотентними мають бути:

- `GET`
- `PUT`
- `DELETE`

Часто і `PATCH` намагаються робити ідемпотентним, якщо це дозволяє логіка.

`POST` зазвичай не є ідемпотентним, бо створює новий ресурс. Але в платіжних або критичних сценаріях `POST` часто додатково захищають `Idempotency-Key`, щоб уникнути дублювання операцій.

### 8.6. Яка роль Schema Definition Language (SDL) у GraphQL або `.proto` файлів у gRPC?

`SDL` у `GraphQL` описує типи, поля, запити, мутації та зв’язки між ними. По суті це контракт між клієнтом і сервером.

`.proto` файли у `gRPC` виконують ту саму роль контракту: вони описують повідомлення, сервіси та методи, після чого на їх основі генерується клієнтський і серверний код.

В обох випадках схема:

- формалізує API;
- забезпечує типобезпечність;
- спрощує документування;
- допомагає автоматизувати генерацію коду.
