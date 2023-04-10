-- Add down migration script here
ALTER TABLE CHATTER
ADD COLUMN tz_name VARCHAR NOT NULL DEFAULT 'Europe/London',
ADD COLUMN tz_offset BIGINT NOT NULL DEFAULT 0;

INSERT INTO LABEL(name, default_translation) VALUES ('your_tz_field', 'Your timezone');
INSERT INTO TRANSLATION(label_id, locale_id, translation) VALUES ((SELECT id FROM label WHERE name='your_tz_field'), 2, 'Votre fuseau horaire');