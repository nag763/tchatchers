-- Add up migration script here
INSERT INTO LABEL(name, default_translation) VALUES
('report_message', 'Report message')
ON CONFLICT DO NOTHING; ;
INSERT INTO LABEL(name, default_translation) VALUES
('report_user', 'Report user')
ON CONFLICT DO NOTHING; ;


INSERT INTO TRANSLATION(label_id, locale_id, translation) VALUES
((SELECT id FROM label WHERE name='report_message'), 2, 'Signaler le message')
ON CONFLICT DO NOTHING; 
INSERT INTO TRANSLATION(label_id, locale_id, translation) VALUES
((SELECT id FROM label WHERE name='report_user'), 2, 'Signaler l''utilisateur')
ON CONFLICT DO NOTHING; ;