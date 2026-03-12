-- Bookmarks
CREATE TABLE bookmarks (
    id UUID PRIMARY KEY,
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    stack_id UUID NOT NULL REFERENCES stacks(id) ON DELETE CASCADE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    UNIQUE(user_id, stack_id)
);
CREATE INDEX idx_bookmarks_user_id ON bookmarks(user_id);

-- Forks: add forked_from column to stacks
ALTER TABLE stacks ADD COLUMN forked_from UUID NULL REFERENCES stacks(id) ON DELETE SET NULL;
