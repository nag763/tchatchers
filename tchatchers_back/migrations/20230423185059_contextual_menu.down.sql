-- Add down migration script here
DELETE FROM LABEL WHERE name='revoke_user';
DELETE FROM LABEL WHERE name='delete_message';
