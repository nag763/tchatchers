-- Add down migration script here
-- Add up migration script here
ALTER TABLE PROCESS_REPORT 
ADD COLUMN PROCESS_KIND TEXT,
DROP COLUMN PROCESS_ID,
DROP CONSTRAINT IF EXISTS fk_process_id_process_kind;

DROP TABLE IF EXISTS PROCESS_KIND;

