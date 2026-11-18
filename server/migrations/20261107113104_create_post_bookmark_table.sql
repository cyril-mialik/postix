CREATE TABLE post_bookmark (
    profile_id UUID NOT NULL,
    post_id UUID NOT NULL,

    created_at TIMESTAMP DEFAULT now(),

    PRIMARY KEY (profile_id, post_id),

    CONSTRAINT fk_bookmark_profile FOREIGN KEY (profile_id) REFERENCES profile(id) ON DELETE CASCADE,
    CONSTRAINT fk_bookmark_post FOREIGN KEY (post_id) REFERENCES post(id) ON DELETE CASCADE
);

CREATE INDEX idx_post_bookmark_post ON post_bookmark (post_id);
