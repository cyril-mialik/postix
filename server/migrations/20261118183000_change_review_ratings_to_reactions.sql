DROP TABLE post_review_rating;

CREATE TABLE post_review_reaction (
    id SERIAL PRIMARY KEY,
    review_id INT NOT NULL,
    profile_id UUID NOT NULL,
    reaction VARCHAR(8) NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    CONSTRAINT fk_post_review_reaction_review
        FOREIGN KEY (review_id) REFERENCES post_review(id) ON DELETE CASCADE ON UPDATE CASCADE,
    CONSTRAINT fk_post_review_reaction_profile
        FOREIGN KEY (profile_id) REFERENCES profile(id) ON DELETE CASCADE ON UPDATE CASCADE,
    CONSTRAINT chk_post_review_reaction_value CHECK (reaction IN ('agree', 'disagree')),
    CONSTRAINT uq_post_review_reaction UNIQUE (review_id, profile_id)
);

CREATE INDEX idx_post_review_reaction_profile ON post_review_reaction (profile_id);
