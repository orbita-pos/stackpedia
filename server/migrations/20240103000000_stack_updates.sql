ALTER TABLE stacks ADD COLUMN updated_at TIMESTAMPTZ;

CREATE TABLE stack_history (
    id UUID PRIMARY KEY,
    stack_id UUID NOT NULL REFERENCES stacks(id) ON DELETE CASCADE,
    changed_by UUID NOT NULL REFERENCES users(id),
    change_type TEXT NOT NULL,
    detail TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);
CREATE INDEX idx_stack_history_stack_id ON stack_history(stack_id);
