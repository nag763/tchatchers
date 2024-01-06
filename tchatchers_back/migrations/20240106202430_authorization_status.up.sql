-- Add up migration script here
CREATE TABLE AUTHORIZATION_STATUS (
	id SERIAL PRIMARY KEY,
	name VARCHAR NOT NULL UNIQUE
);

INSERT INTO AUTHORIZATION_STATUS(id, name) 
VALUES (1, 'deactivated'), (10, 'pending_verification'), (11, 'unverified_authorized'), (20, 'verified'), (21, 'verified_by_admin');

ALTER TABLE CHATTER
ADD COLUMN authorization_status_id INTEGER NOT NULL DEFAULT 10,
ADD CONSTRAINT fk_authorization_status FOREIGN KEY(authorization_status_id) REFERENCES AUTHORIZATION_STATUS(id) ON DELETE SET DEFAULT;

UPDATE CHATTER SET authorization_status_id = 1 WHERE is_authorized IS FALSE;
UPDATE CHATTER SET authorization_status_id = 11 WHERE is_authorized IS TRUE;