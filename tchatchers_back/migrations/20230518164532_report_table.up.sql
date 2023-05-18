-- Add up migration script here
CREATE TABLE REPORTED_MESSAGE (
	id SERIAL PRIMARY KEY,
	reporter_id INTEGER NOT NULL,
	message_uuid UUID NOT NULL,
	created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT fk_reporter
        FOREIGN KEY(reporter_id)
        REFERENCES CHATTER(id)
        ON DELETE CASCADE,
    CONSTRAINT fk_message
        FOREIGN KEY(message_uuid)
        REFERENCES MESSAGE(uuid)
        ON DELETE CASCADE,
    CONSTRAINT unicity_reporter_message
        UNIQUE (message_uuid, reporter_id)
);

CREATE TABLE REPORTED_USER (
	id SERIAL PRIMARY KEY,
	reporter_id INTEGER NOT NULL,
	reported_id INTEGER NOT NULL,
	created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT fk_reporter
        FOREIGN KEY(reporter_id)
        REFERENCES CHATTER(id)
        ON DELETE CASCADE,
    CONSTRAINT fk_reported
        FOREIGN KEY(reported_id)
        REFERENCES CHATTER(id)
        ON DELETE CASCADE,
    CONSTRAINT unicity_reported_user
        UNIQUE (reported_id, reporter_id)
);