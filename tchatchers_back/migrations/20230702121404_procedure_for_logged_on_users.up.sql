-- Add up migration script here
CREATE TYPE chatter_logged_on AS (
    id INTEGER,
    queue_id TEXT
);

CREATE OR REPLACE FUNCTION update_last_logon(chatter_list chatter_logged_on[])
RETURNS TABLE (id INTEGER, queue_id TEXT)
LANGUAGE plpgsql
AS $$
BEGIN
    
    -- Create a temporary table to store the chatter_list
    CREATE TEMPORARY TABLE TEMP_ROWS_TO_UPDATE (
        id INTEGER,
        queue_id TEXT,
        is_updated BOOLEAN DEFAULT FALSE
    );

    -- Insert the chatter_list values into the temporary table
    INSERT INTO TEMP_ROWS_TO_UPDATE (id, queue_id)
    SELECT (chatter_list[i]).id, (chatter_list[i]).queue_id
    FROM generate_subscripts(chatter_list, 1) AS i;

    UPDATE CHATTER c SET LAST_LOGON = NOW() FROM TEMP_ROWS_TO_UPDATE tr WHERE tr.id = c.id;
    UPDATE TEMP_ROWS_TO_UPDATE tr SET is_updated=true FROM CHATTER c WHERE c.id = tr.id;
    
    IF EXISTS(SELECT 1 FROM TEMP_ROWS_TO_UPDATE WHERE is_updated=false)
    THEN
        UPDATE TEMP_ROWS_TO_UPDATE tr SET is_updated=true FROM DELETED_RECORD dr WHERE tr.id = dr.RECORD_ID AND dr.ORIGIN = 'CHATTER' AND tr.is_updated=false;
    END IF;

    RETURN QUERY SELECT tr.id, tr.queue_id FROM TEMP_ROWS_TO_UPDATE tr WHERE tr.is_updated=false;

    DROP TABLE TEMP_ROWS_TO_UPDATE;
END;
$$;
