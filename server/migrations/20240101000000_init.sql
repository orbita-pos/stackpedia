CREATE TABLE users (
    id UUID PRIMARY KEY,
    nickname TEXT NOT NULL,
    recovery_code_hash TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE TABLE stacks (
    id UUID PRIMARY KEY,
    creator_id UUID NOT NULL REFERENCES users(id),
    project_name TEXT NOT NULL,
    description TEXT NOT NULL,
    category TEXT NOT NULL,
    url TEXT,
    scale TEXT NOT NULL,
    lessons TEXT,
    upvotes INT NOT NULL DEFAULT 0,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE TABLE stack_tools (
    id UUID PRIMARY KEY,
    stack_id UUID NOT NULL REFERENCES stacks(id) ON DELETE CASCADE,
    name TEXT NOT NULL,
    category TEXT NOT NULL,
    why TEXT NOT NULL,
    cost TEXT,
    verdict TEXT NOT NULL
);

CREATE INDEX idx_stack_tools_name ON stack_tools(name);
CREATE INDEX idx_stack_tools_stack_id ON stack_tools(stack_id);

CREATE TABLE votes (
    id UUID PRIMARY KEY,
    user_id UUID NOT NULL REFERENCES users(id),
    stack_id UUID NOT NULL REFERENCES stacks(id) ON DELETE CASCADE,
    direction SMALLINT NOT NULL,
    UNIQUE(user_id, stack_id)
);

CREATE TABLE comments (
    id UUID PRIMARY KEY,
    stack_id UUID NOT NULL REFERENCES stacks(id) ON DELETE CASCADE,
    creator_id UUID NOT NULL REFERENCES users(id),
    content TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX idx_comments_stack_id ON comments(stack_id);
CREATE INDEX idx_stacks_category ON stacks(category);
CREATE INDEX idx_stacks_scale ON stacks(scale);
CREATE INDEX idx_stacks_created_at ON stacks(created_at DESC);
