-- Add up migration script here
CREATE TABLE REPORT (
	id SERIAL PRIMARY KEY,
	reporter_id INTEGER NOT NULL,
	reported_id INTEGER,
	message_uuid UUID,
    report_kind_id INTEGER NOT NULL,
	created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT fk_reporter
        FOREIGN KEY(reporter_id)
        REFERENCES CHATTER(id)
        ON DELETE CASCADE,
    CONSTRAINT fk_reported
        FOREIGN KEY(reported_id)
        REFERENCES CHATTER(id)
        ON DELETE CASCADE,
    CONSTRAINT fk_message
        FOREIGN KEY(message_uuid)
        REFERENCES MESSAGE(uuid)
        ON DELETE CASCADE,
    CONSTRAINT unicity_reported_user
        UNIQUE (reporter_id, reported_id),
    CONSTRAINT unicity_reported_message
        UNIQUE (reporter_id, message_uuid),
    CONSTRAINT something_reported
        CHECK (reported_id is not null or message_uuid is not null)
);