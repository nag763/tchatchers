ALTER TABLE CHATTER
ALTER COLUMN created_at SET DATA TYPE timestamptz USING created_at::timestamptz;

ALTER TABLE CHATTER
ALTER COLUMN last_update SET DATA TYPE timestamptz USING last_update::timestamptz;