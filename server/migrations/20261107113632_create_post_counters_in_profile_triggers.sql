CREATE OR REPLACE FUNCTION update_post_count_in_profile()
RETURNS TRIGGER AS $$
BEGIN
    IF (TG_OP = 'INSERT') THEN
        UPDATE profile 
        SET posts_count = COALESCE(posts_count, 0) + 1 
        WHERE id = NEW.profile_id;
        
    ELSIF (TG_OP = 'DELETE') THEN
        UPDATE profile 
        SET posts_count = COALESCE(posts_count, 0) - 1 
        WHERE id = OLD.profile_id;
    END IF;
    RETURN NULL;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER trg_post_profile_counter
AFTER INSERT OR DELETE ON post
FOR EACH ROW EXECUTE FUNCTION update_post_count_in_profile();
