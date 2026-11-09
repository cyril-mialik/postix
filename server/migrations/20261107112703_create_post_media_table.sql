CREATE TABLE post_media (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    url VARCHAR(1000) NOT NULL,
    media_type VARCHAR(32) NOT NULL,

    position INT NOT NULL DEFAULT 0,

    created_at TIMESTAMP DEFAULT now(),

    post_id UUID NOT NULL,
    
    CONSTRAINT fk_post_media FOREIGN KEY (post_id) REFERENCES post(id) ON DELETE CASCADE ON UPDATE CASCADE,
    CONSTRAINT chk_media_type CHECK (media_type IN ('image', 'video'))
);

CREATE INDEX idx_post_media_post_id ON post_media (post_id);
CREATE INDEX idx_post_media_order ON post_media (post_id, position);
