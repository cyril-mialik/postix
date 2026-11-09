CREATE TABLE follow (
    follower_id UUID NOT NULL,
    following_id UUID NOT NULL,

    created_at TIMESTAMP DEFAULT now(),

    PRIMARY KEY (follower_id, following_id),

    CONSTRAINT fk_follow_follower FOREIGN KEY (follower_id) REFERENCES profile(id) ON DELETE CASCADE ON UPDATE CASCADE,
    CONSTRAINT fk_follow_following FOREIGN KEY (following_id) REFERENCES profile(id) ON DELETE CASCADE ON UPDATE CASCADE,
    CONSTRAINT chk_no_self_follow CHECK (follower_id != following_id)
);

CREATE INDEX idx_follow_following ON follow (following_id);
