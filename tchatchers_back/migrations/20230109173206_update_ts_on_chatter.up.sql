-- Add up migration script here
ALTER TABLE CHATTER
ADD COLUMN last_update TIMESTAMP DEFAULT NULL;

CREATE OR REPLACE FUNCTION update_lastupdated_timestamp()
RETURNS TRIGGER AS $$
BEGIN
  NEW.last_update = current_timestamp;
  RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER update_lastupdated_timestamp
BEFORE UPDATE ON CHATTER
FOR EACH ROW
EXECUTE PROCEDURE update_lastupdated_timestamp();