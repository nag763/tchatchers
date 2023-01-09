-- Add down migration script here
ALTER TABLE CHATTER
DROP COLUMN tz_name;
ALTER TABLE CHATTER
DROP COLUMN tz_offset;
