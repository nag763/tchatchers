ALTER TABLE CHATTER
ALTER COLUMN created_at SET DATA TYPE timestamp USING created_at::timestamp;

ALTER TABLE CHATTER
ALTER COLUMN last_update SET DATA TYPE timestamp USING last_update::timestamp;