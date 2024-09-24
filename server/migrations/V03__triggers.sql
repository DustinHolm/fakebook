-- for posts
CREATE FUNCTION post_notification() RETURNS trigger AS $post_notification$
    DECLARE
        message TEXT;
    BEGIN
        message := format('%s:%s', NEW.post_id, NEW.author);
        PERFORM pg_notify('post_notification', message);
        RETURN NEW;
    END;
$post_notification$ LANGUAGE plpgsql;

CREATE TRIGGER post_notification_trigger
AFTER INSERT OR UPDATE ON post
FOR EACH ROW EXECUTE FUNCTION post_notification();

-- for comments
CREATE FUNCTION comment_notification() RETURNS trigger AS $comment_notification$
    DECLARE
        message TEXT;
    BEGIN
        message := format('%s:%s:%s', NEW.comment_id, NEW.referenced_post, NEW.author);
        PERFORM pg_notify('comment_notification', message);
        RETURN NEW;
    END;
$comment_notification$ LANGUAGE plpgsql;

CREATE TRIGGER comment_notification_trigger
AFTER INSERT OR UPDATE OR DELETE ON comment
FOR EACH ROW EXECUTE FUNCTION comment_notification();