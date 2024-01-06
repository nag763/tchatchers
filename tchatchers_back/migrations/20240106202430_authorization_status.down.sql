-- Add down migration script here
UPDATE CHATTER SET is_authorized = FALSE WHERE authorization_status_id = 1;
UPDATE CHATTER SET is_authorized = TRUE WHERE authorization_status_id != 1;

ALTER TABLE CHATTER DROP COLUMN authorization_status_id;
DROP TABLE AUTHORIZATION_STATUS;