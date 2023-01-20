-- Add down migration script here
DELETE FROM LABEL WHERE name IN(
'your_login_field',
'your_name_field',
'your_locale_field',
'your_tz_field',
'your_pfp_field',
'delete_profile',
'modal_delete_profile_title',
'modal_delete_profile_content',
'modal_delete_profile_no',
'modal_delete_profile_yes',
'update_profile',
'join_a_room_title',
'join_room',
'room_name',
'room_name_incorrect',
'type_msg_here',
'you_are_disconnected',
'try_reconnect',
'logged_out',
'profile_updated');

DELETE FROM TRANSLATION WHERE label_id=(SELECT id FROM label WHERE name='your_login_field') AND locale_id=2;
DELETE FROM TRANSLATION WHERE label_id=(SELECT id FROM label WHERE name='your_name_field') AND locale_id=2;
DELETE FROM TRANSLATION WHERE label_id=(SELECT id FROM label WHERE name='your_locale_field') AND locale_id=2;
DELETE FROM TRANSLATION WHERE label_id=(SELECT id FROM label WHERE name='your_tz_field') AND locale_id=2;
DELETE FROM TRANSLATION WHERE label_id=(SELECT id FROM label WHERE name='your_pfp_field') AND locale_id=2;
DELETE FROM TRANSLATION WHERE label_id=(SELECT id FROM label WHERE name='delete_profile') AND locale_id=2;
DELETE FROM TRANSLATION WHERE label_id=(SELECT id FROM label WHERE name='modal_delete_profile_title') AND locale_id=2;
DELETE FROM TRANSLATION WHERE label_id=(SELECT id FROM label WHERE name='modal_delete_profile_content') AND locale_id=2;
DELETE FROM TRANSLATION WHERE label_id=(SELECT id FROM label WHERE name='modal_delete_profile_no') AND locale_id=2;
DELETE FROM TRANSLATION WHERE label_id=(SELECT id FROM label WHERE name='modal_delete_profile_yes') AND locale_id=2;
DELETE FROM TRANSLATION WHERE label_id=(SELECT id FROM label WHERE name='update_profile') AND locale_id=2;
DELETE FROM TRANSLATION WHERE label_id=(SELECT id FROM label WHERE name='join_a_room_title') AND locale_id=2;
DELETE FROM TRANSLATION WHERE label_id=(SELECT id FROM label WHERE name='join_room') AND locale_id=2;
DELETE FROM TRANSLATION WHERE label_id=(SELECT id FROM label WHERE name='room_name') AND locale_id=2;
DELETE FROM TRANSLATION WHERE label_id=(SELECT id FROM label WHERE name='room_name_incorrect') AND locale_id=2;
DELETE FROM TRANSLATION WHERE label_id=(SELECT id FROM label WHERE name='type_msg_here') AND locale_id=2;
DELETE FROM TRANSLATION WHERE label_id=(SELECT id FROM label WHERE name='you_are_disconnected') AND locale_id=2;
DELETE FROM TRANSLATION WHERE label_id=(SELECT id FROM label WHERE name='try_reconnect') AND locale_id=2;
DELETE FROM TRANSLATION WHERE label_id=(SELECT id FROM label WHERE name='logged_out') AND locale_id=2;
DELETE FROM TRANSLATION WHERE label_id=(SELECT id FROM label WHERE name='profile_updated') AND locale_id=2;