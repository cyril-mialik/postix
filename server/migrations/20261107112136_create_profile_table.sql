CREATE TABLE profile (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    username VARCHAR(128) NOT NULL,
    avatar VARCHAR(1000),

    first_name VARCHAR(128),
    last_name VARCHAR(128),

    followers_count INT DEFAULT 0,
    following_count INT DEFAULT 0,
    posts_count INT DEFAULT 0,

    created_at TIMESTAMP DEFAULT now(),
    updated_at TIMESTAMP DEFAULT now(),
    deleted_at TIMESTAMP,

    theme_id INT NOT NULL DEFAULT 1,
    auth_id UUID UNIQUE NOT NULL,
    
    CONSTRAINT fk_theme_profile FOREIGN KEY (theme_id) REFERENCES theme(id) ON UPDATE CASCADE,
    CONSTRAINT fk_auth_profile FOREIGN KEY (auth_id) REFERENCES auth(id) ON DELETE CASCADE ON UPDATE CASCADE
);

CREATE UNIQUE INDEX uq_profile_username_lower ON profile (lower(username));
CREATE UNIQUE INDEX uq_profile_auth ON profile (auth_id);
