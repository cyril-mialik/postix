CREATE TABLE post_history (
    id BIGSERIAL PRIMARY KEY,
    post_id UUID NOT NULL REFERENCES post(id) ON DELETE CASCADE,
    snapshot JSONB NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_post_history_post_created
    ON post_history (post_id, created_at DESC);

CREATE TABLE post_review_history (
    id BIGSERIAL PRIMARY KEY,
    review_id INTEGER NOT NULL,
    post_id UUID NOT NULL REFERENCES post(id) ON DELETE CASCADE,
    snapshot JSONB NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_post_review_history_review_created
    ON post_review_history (review_id, created_at DESC);
