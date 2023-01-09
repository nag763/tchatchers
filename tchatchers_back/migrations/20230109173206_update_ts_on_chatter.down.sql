-- Add down migration script here

DROP TRIGGER update_lastupdated_timestamp ON CHATTER;
DROP FUNCTION update_lastupdated_timestamp();

ALTER TABLE CHATTER
DROP COLUMN last_update;