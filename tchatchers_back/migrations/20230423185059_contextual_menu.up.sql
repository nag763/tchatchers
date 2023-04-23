-- Add up migration script here
INSERT INTO LABEL(name, default_translation) VALUES
('revoke_user', 'Revoke user')
ON CONFLICT DO NOTHING; ;

INSERT INTO TRANSLATION(label_id, locale_id, translation) VALUES
((SELECT id FROM label WHERE name='revoke_user'), 2, 'Désactiver l''utilisateur')
ON CONFLICT DO NOTHING; ;

INSERT INTO LABEL(name, default_translation) VALUES
('delete_message', 'Delete message')
ON CONFLICT DO NOTHING; ;

INSERT INTO TRANSLATION(label_id, locale_id, translation) VALUES
((SELECT id FROM label WHERE name='delete_message'), 2, 'Supprimer le message')
ON CONFLICT DO NOTHING; ;