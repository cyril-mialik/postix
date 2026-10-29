ALTER TABLE settings ADD COLUMN user_id UUID;

UPDATE settings
SET user_id = (SELECT id FROM users LIMIT 1)
WHERE user_id IS NULL;

ALTER TABLE settings ALTER COLUMN user_id SET NOT NULL;
ALTER TABLE settings
ADD CONSTRAINT fk_settings_user
FOREIGN KEY (user_id) REFERENCES users(id)
ON DELETE CASCADE;
