-- Add up migration script here
ALTER TABLE REPORT
ALTER COLUMN created_at SET DATA TYPE timestamptz USING created_at::timestamptz,
ADD COLUMN reported_name TEXT NULL,
ADD COLUMN reported_pfp TEXT NULL,
ADD COLUMN message_content TEXT NULL;