-- Add up migration script here
ALTER TABLE REPORT
ALTER COLUMN created_at SET DATA TYPE timestamp USING created_at::timestamp,
DROP COLUMN reported_name,
DROP COLUMN reported_pfp,
DROP COLUMN message_content ;