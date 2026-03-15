# Етап 1: Підготовка рецепта
FROM lukemathwalker/cargo-chef:latest-rust-1.92-bookworm AS chef
WORKDIR /app
FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

# Етап 2: Збірка залежностей
FROM chef AS cacher
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json

# Етап 3: Збірка проекту
FROM chef AS builder
COPY . .
ENV SQLX_OFFLINE=true
COPY --from=cacher /app/target target
COPY --from=cacher $CARGO_HOME $CARGO_HOME
RUN cargo build --release --bin server --bin create_admin

# Етап 4: Фінальний образ
FROM debian:bookworm-slim AS runtime
WORKDIR /app
RUN apt-get update && apt-get install -y --no-install-recommends \
    openssl \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/server /app/server
COPY --from=builder /app/target/release/create_admin /app/create_admin
COPY --from=builder /app/migrations /app/migrations
CMD ["./server"]
