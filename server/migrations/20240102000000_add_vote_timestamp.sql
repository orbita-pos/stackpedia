ALTER TABLE votes ADD COLUMN created_at TIMESTAMPTZ NOT NULL DEFAULT now();
CREATE INDEX idx_votes_created_at ON votes(created_at DESC);
