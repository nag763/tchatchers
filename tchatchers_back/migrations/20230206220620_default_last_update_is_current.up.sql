ALTER TABLE CHATTER
ALTER COLUMN last_update SET DEFAULT now();

UPDATE CHATTER SET last_update = now() WHERE last_update IS NULL;