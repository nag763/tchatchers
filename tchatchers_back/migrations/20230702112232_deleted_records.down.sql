-- Add down migration script here
DROP TABLE DELETED_RECORD;

-- Drop the trigger
DROP TRIGGER IF EXISTS chatters_delete_trigger ON CHATTER;

-- Drop the trigger function
DROP FUNCTION IF EXISTS chatters_delete_trigger_function();

-- Revert script to delete the trigger
DROP TRIGGER IF EXISTS messages_delete_trigger ON MESSAGE;

DROP FUNCTION IF EXISTS messages_delete_trigger_function();
