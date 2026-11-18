CREATE TABLE notification (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    is_read BOOLEAN DEFAULT false,

    recipient_id UUID NOT NULL,
    actor_id UUID,
    post_id UUID,
    notification_type_id INT NOT NULL DEFAULT 1,

    created_at TIMESTAMP DEFAULT now(),
    
    CONSTRAINT fk_notif_recipient FOREIGN KEY (recipient_id) REFERENCES profile(id) ON DELETE CASCADE,
    CONSTRAINT fk_notif_actor FOREIGN KEY (actor_id) REFERENCES profile(id) ON DELETE CASCADE,
    CONSTRAINT fk_notif_post FOREIGN KEY (post_id) REFERENCES post(id) ON DELETE CASCADE,
    CONSTRAINT fk_notif_type FOREIGN KEY (notification_type_id) REFERENCES notification_type(id) ON UPDATE CASCADE
);

CREATE INDEX idx_notification_recipient ON notification (recipient_id);
CREATE INDEX idx_notification_unread ON notification (recipient_id, is_read);
CREATE UNIQUE INDEX uq_notification_dedupe ON notification (recipient_id, actor_id, post_id, notification_type_id);
