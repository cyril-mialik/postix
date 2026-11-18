CREATE TABLE auth (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    email VARCHAR NOT NULL,
    password VARCHAR NOT NULL,

    is_email_verified BOOLEAN DEFAULT false,

    is_banned BOOLEAN DEFAULT false,
    ban_reason VARCHAR(512),
    banned_at TIMESTAMP,

    role_id INT NOT NULL DEFAULT 1,

    created_at TIMESTAMP DEFAULT now(),
    updated_at TIMESTAMP DEFAULT now(),

    CONSTRAINT fk_role_auth FOREIGN KEY (role_id) REFERENCES role(id) ON UPDATE CASCADE
);

CREATE UNIQUE INDEX uq_auth_email_lower ON auth (lower(email));
