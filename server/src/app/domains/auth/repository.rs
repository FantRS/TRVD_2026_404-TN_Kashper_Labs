use sqlx::PgExecutor;
use uuid::Uuid;

use crate::app::RequestResult;
use crate::app::domains::auth::models::{AuthUserResponse, RegisterRequestValid, UserRow};

pub async fn create_user<'c, E>(
    request: &RegisterRequestValid,
    password_hash: &str,
    executor: E,
) -> RequestResult<AuthUserResponse>
where
    E: PgExecutor<'c>,
{
    let row = sqlx::query_as::<_, UserRow>(
        r#"
        WITH default_role AS (
            SELECT id, code
            FROM roles
            WHERE code = 'user'
        )
        INSERT INTO users (
            role_id,
            email,
            password_hash,
            full_name,
            phone
        )
        SELECT
            default_role.id,
            $1,
            $2,
            $3,
            $4
        FROM default_role
        RETURNING
            users.id,
            users.email,
            users.password_hash,
            users.full_name,
            users.phone,
            users.wallet_balance::DOUBLE PRECISION AS wallet_balance,
            users.role_id,
            (SELECT code FROM default_role) AS role_code,
            users.is_active
        "#,
    )
    .bind(&request.email)
    .bind(password_hash)
    .bind(&request.full_name)
    .bind(&request.phone)
    .fetch_one(executor)
    .await?;

    row.try_into()
}

pub async fn find_user_by_email<'c, E>(email: &str, executor: E) -> RequestResult<UserRow>
where
    E: PgExecutor<'c>,
{
    sqlx::query_as::<_, UserRow>(
        r#"
        SELECT
            users.id,
            users.email,
            users.password_hash,
            users.full_name,
            users.phone,
            users.wallet_balance::DOUBLE PRECISION AS wallet_balance,
            users.role_id,
            roles.code AS role_code,
            users.is_active
        FROM users
        INNER JOIN roles ON roles.id = users.role_id
        WHERE users.email = $1
        "#,
    )
    .bind(email)
    .fetch_one(executor)
    .await
    .map_err(Into::into)
}

pub async fn find_user_by_id<'c, E>(id: Uuid, executor: E) -> RequestResult<UserRow>
where
    E: PgExecutor<'c>,
{
    sqlx::query_as::<_, UserRow>(
        r#"
        SELECT
            users.id,
            users.email,
            users.password_hash,
            users.full_name,
            users.phone,
            users.wallet_balance::DOUBLE PRECISION AS wallet_balance,
            users.role_id,
            roles.code AS role_code,
            users.is_active
        FROM users
        INNER JOIN roles ON roles.id = users.role_id
        WHERE users.id = $1
        "#,
    )
    .bind(id)
    .fetch_one(executor)
    .await
    .map_err(Into::into)
}
