-- Add up migration script here
ALTER TABLE CHATTER
ADD COLUMN email TEXT UNIQUE;