-- Add up migration script here
ALTER TABLE CHATTER
ADD COLUMN tz_name VARCHAR NOT NULL DEFAULT 'Europe/London',
ADD COLUMN tz_offset BIGINT NOT NULL DEFAULT 0;
