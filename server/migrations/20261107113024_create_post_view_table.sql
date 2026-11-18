CREATE TABLE post_view (
    post_id UUID NOT NULL,
    viewer_id UUID NOT NULL,

    viewed_at DATE NOT NULL,

    PRIMARY KEY (post_id, viewer_id, viewed_at),

    CONSTRAINT fk_post_view_post FOREIGN KEY (post_id) REFERENCES post(id) ON DELETE CASCADE,
    CONSTRAINT fk_post_view_viewer FOREIGN KEY (viewer_id) REFERENCES profile(id) ON DELETE CASCADE
);
