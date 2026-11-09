CREATE TABLE post (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    title VARCHAR(256),
    content JSONB,

    rating_sum INT DEFAULT 0,
    rating_count INT DEFAULT 0,
    reviews_count INT DEFAULT 0,
    views_count INT DEFAULT 0,

    created_at TIMESTAMP DEFAULT now(),
    updated_at TIMESTAMP DEFAULT now(),
    deleted_at TIMESTAMP,
    deleted_by UUID,

    profile_id UUID NOT NULL,
    status_id INT NOT NULL DEFAULT 1,
    
    CONSTRAINT fk_profile_post FOREIGN KEY (profile_id) REFERENCES profile(id) ON DELETE CASCADE ON UPDATE CASCADE,
    CONSTRAINT fk_status_post FOREIGN KEY (status_id) REFERENCES post_status(id) ON UPDATE CASCADE,
    CONSTRAINT fk_deleted_by_post FOREIGN KEY (deleted_by) REFERENCES profile(id) ON DELETE SET NULL ON UPDATE CASCADE
);

CREATE INDEX idx_post_profile_id ON post (profile_id);
CREATE INDEX idx_post_feed ON post (status_id, created_at);
