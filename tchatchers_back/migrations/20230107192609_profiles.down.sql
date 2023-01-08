-- Add down migration script here
ALTER TABLE CHATTER
DROP COLUMN profile_id;

DROP TABLE PROFILE;