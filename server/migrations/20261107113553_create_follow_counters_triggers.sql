CREATE OR REPLACE FUNCTION update_follow_counters()
RETURNS TRIGGER AS $$
BEGIN
    IF (TG_OP = 'INSERT') THEN
        UPDATE profile 
        SET following_count = COALESCE(following_count, 0) + 1 
        WHERE id = NEW.follower_id;
        
        UPDATE profile 
        SET followers_count = COALESCE(followers_count, 0) + 1 
        WHERE id = NEW.following_id;
        
    ELSIF (TG_OP = 'DELETE') THEN
        UPDATE profile 
        SET following_count = COALESCE(following_count, 0) - 1 
        WHERE id = OLD.follower_id;
        
        UPDATE profile 
        SET followers_count = COALESCE(followers_count, 0) - 1 
        WHERE id = OLD.following_id;
    END IF;
    RETURN NULL;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER trg_follow_counters
AFTER INSERT OR DELETE ON follow
FOR EACH ROW EXECUTE FUNCTION update_follow_counters();
