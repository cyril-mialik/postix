CREATE TABLE report (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    reason VARCHAR(20) NOT NULL,
    message VARCHAR(1000),

    created_at TIMESTAMP DEFAULT now(),
    resolved_at TIMESTAMP,
    resolved_by UUID,

    reporter_id UUID NOT NULL,
    post_id UUID NOT NULL,
    status_id INT NOT NULL DEFAULT 1,
    
    CONSTRAINT fk_report_reporter FOREIGN KEY (reporter_id) REFERENCES profile(id) ON DELETE CASCADE ON UPDATE CASCADE,
    CONSTRAINT fk_report_post FOREIGN KEY (post_id) REFERENCES post(id) ON DELETE CASCADE ON UPDATE CASCADE,
    CONSTRAINT fk_report_status FOREIGN KEY (status_id) REFERENCES report_status(id) ON UPDATE CASCADE,
    CONSTRAINT fk_report_resolved_by FOREIGN KEY (resolved_by) REFERENCES profile(id) ON DELETE SET NULL ON UPDATE CASCADE,
    CONSTRAINT chk_valid_report_reason CHECK (reason IN ('spam', 'abuse', 'nsfw', 'violence', 'copyright', 'other'))
);

CREATE UNIQUE INDEX uq_report_one_per_post ON report (reporter_id, post_id);
CREATE INDEX idx_report_queue ON report (status_id, created_at);
