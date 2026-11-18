CREATE OR REPLACE FUNCTION update_post_counters()
RETURNS TRIGGER AS $$
BEGIN
    IF (TG_OP = 'INSERT') THEN
        IF (TG_TABLE_NAME = 'post_rating') THEN
            UPDATE post 
            SET rating_sum = COALESCE(rating_sum, 0) + NEW.rating, 
                rating_count = COALESCE(rating_count, 0) + 1 
            WHERE id = NEW.post_id;
        ELSIF (TG_TABLE_NAME = 'post_review') THEN
            UPDATE post 
            SET reviews_count = COALESCE(reviews_count, 0) + 1 
            WHERE id = NEW.post_id;
        END IF;
        
    ELSIF (TG_OP = 'DELETE') THEN
        IF (TG_TABLE_NAME = 'post_rating') THEN
            UPDATE post 
            SET rating_sum = COALESCE(rating_sum, 0) - OLD.rating, 
                rating_count = COALESCE(rating_count, 0) - 1 
            WHERE id = OLD.post_id;
        ELSIF (TG_TABLE_NAME = 'post_review') THEN
            UPDATE post 
            SET reviews_count = COALESCE(reviews_count, 0) - 1 
            WHERE id = OLD.post_id;
        END IF;
        
    ELSIF (TG_OP = 'UPDATE') THEN
        IF (TG_TABLE_NAME = 'post_rating') THEN
            IF (OLD.post_id = NEW.post_id) THEN
                UPDATE post 
                SET rating_sum = COALESCE(rating_sum, 0) - OLD.rating + NEW.rating 
                WHERE id = NEW.post_id;
            ELSE
                UPDATE post 
                SET rating_sum = COALESCE(rating_sum, 0) - OLD.rating, 
                    rating_count = COALESCE(rating_count, 0) - 1 
                WHERE id = OLD.post_id;
                
                UPDATE post 
                SET rating_sum = COALESCE(rating_sum, 0) + NEW.rating, 
                    rating_count = COALESCE(rating_count, 0) + 1 
                WHERE id = NEW.post_id;
            END IF;
        END IF;
    END IF;
    RETURN NULL;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER trg_post_rating_counter
AFTER INSERT OR UPDATE OR DELETE ON post_rating
FOR EACH ROW EXECUTE FUNCTION update_post_counters();

CREATE TRIGGER trg_post_review_counter
AFTER INSERT OR DELETE ON post_review
FOR EACH ROW EXECUTE FUNCTION update_post_counters();
