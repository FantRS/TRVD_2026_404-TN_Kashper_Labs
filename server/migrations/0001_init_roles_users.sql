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
    wallet_balance NUMERIC(12, 2) NOT NULL DEFAULT 10000 CHECK (wallet_balance >= 0),
    is_active BOOLEAN NOT NULL DEFAULT TRUE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_users_role_id ON users(role_id);
CREATE INDEX idx_users_email ON users(email);
CREATE INDEX idx_users_is_active ON users(is_active);
CREATE INDEX idx_users_wallet_balance ON users(wallet_balance);
