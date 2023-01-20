-- Add up migration script here
INSERT INTO LABEL(name, default_translation) VALUES
('your_login_field', 'Your login'),
('your_name_field', 'Your name'),
('your_locale_field', 'Your locale'),
('your_tz_field', 'Your timezone'),
('your_pfp_field', 'Your pfp'),
('delete_profile', 'Delete'),
('modal_delete_profile_title', 'You are about to delete your account'),
('modal_delete_profile_content', 'This action is not reversible, once your account is deleted, there is no way for you to get it back.'),
('modal_delete_profile_no', 'I changed, my mind, don''t delete my account'),
('modal_delete_profile_yes', 'Understood, farewell'),
('update_profile', 'Update'),
('join_a_room_title', 'Join a room'),
('join_room', 'Join'),
('room_name', 'Room name'),
('room_name_incorrect', 'The room name you tried to join is not valid, please select one within this screen.'),
('type_msg_here', 'Type a message here'),
('you_are_disconnected', 'You are disconnecetd'),
('try_reconnect', 'Reconnect'),
('logged_out', 'You logged out with success, see you !'),
('profile_updated', 'Your profile has been updated with success')
ON CONFLICT DO NOTHING;

INSERT INTO TRANSLATION(label_id, locale_id, translation) VALUES
((SELECT id FROM label WHERE name='your_login_field'), 2, 'Votre identifiant'),
((SELECT id FROM label WHERE name='your_name_field'), 2, 'Votre nom'),
((SELECT id FROM label WHERE name='your_locale_field'), 2, 'Votre locale'),
((SELECT id FROM label WHERE name='your_tz_field'), 2, 'Votre fuseau horaire'),
((SELECT id FROM label WHERE name='your_pfp_field'), 2, 'Votre photo de profil'),
((SELECT id FROM label WHERE name='delete_profile'), 2, 'Supprimer'),
((SELECT id FROM label WHERE name='modal_delete_profile_title'), 2, 'Vous êtes sur le point de supprimer votre compte'),
((SELECT id FROM label WHERE name='modal_delete_profile_content'), 2, 'Cette action n’est pas réversible, une fois que votre compte est supprimé, il n’y aura pas de possibilité de récuperer votre compte.'),
((SELECT id FROM label WHERE name='modal_delete_profile_no'), 2, 'J’ai changé d’avis, ne supprimez pas mon compte'),
((SELECT id FROM label WHERE name='modal_delete_profile_yes'), 2, 'Je souhaite supprimer mon compte'),
((SELECT id FROM label WHERE name='update_profile'), 2, 'Mettre à jour'),
((SELECT id FROM label WHERE name='join_a_room_title'), 2, 'Rejoindre un salon'),
((SELECT id FROM label WHERE name='join_room'), 2, 'Rejoindre'),
((SELECT id FROM label WHERE name='room_name'), 2, 'Nom du salon'),
((SELECT id FROM label WHERE name='room_name_incorrect'), 2, 'Le nom de salon que vous avez entré n’est pas correct, essayez de le taper de nouveau via cet écran'),
((SELECT id FROM label WHERE name='type_msg_here'), 2, 'Commencez à écrire ici'),
((SELECT id FROM label WHERE name='you_are_disconnected'), 2, 'Vous êtes déconnecté'),
((SELECT id FROM label WHERE name='try_reconnect'), 2, 'Se reconnecter'),
((SELECT id FROM label WHERE name='logged_out'), 2, 'Vous êtes déconnecté, au revoir !'),
((SELECT id FROM label WHERE name='profile_updated'), 2, 'Votre profil a été mis à jour avec succès')
ON CONFLICT DO NOTHING;


