-- Add down migration script here
DELETE FROM PROCESS_KIND WHERE id =5, 'RemoveUserData';