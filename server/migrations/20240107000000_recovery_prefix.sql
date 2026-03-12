ALTER TABLE users ADD COLUMN recovery_prefix VARCHAR(9);
CREATE INDEX idx_users_recovery_prefix ON users(recovery_prefix);
