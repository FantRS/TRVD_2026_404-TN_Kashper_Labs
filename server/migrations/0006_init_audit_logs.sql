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
