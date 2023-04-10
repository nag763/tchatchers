-- Add up migration script here
ALTER TABLE CHATTER
DROP COLUMN tz_name;
ALTER TABLE CHATTER
DROP COLUMN tz_offset;

DELETE FROM LABEL WHERE name = 'your_tz_field';