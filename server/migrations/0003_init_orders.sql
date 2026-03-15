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
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE (order_id, service_id)
);

CREATE TABLE order_product_items (
    id UUID PRIMARY KEY DEFAULT uuidv7(),
    order_id UUID NOT NULL REFERENCES orders(id) ON DELETE CASCADE,
    product_id UUID NOT NULL REFERENCES products(id),
    quantity INTEGER NOT NULL CHECK (quantity > 0),
    unit_price NUMERIC(12, 2) NOT NULL CHECK (unit_price >= 0),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE (order_id, product_id)
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
