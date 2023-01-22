-- Add up migration script here
INSERT INTO LABEL(name, default_translation) VALUES
('settings', 'Settings');

INSERT INTO TRANSLATION(label_id, locale_id, translation) VALUES
((SELECT id FROM label WHERE name='settings'), 2, 'Paramètres'),