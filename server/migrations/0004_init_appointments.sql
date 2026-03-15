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
