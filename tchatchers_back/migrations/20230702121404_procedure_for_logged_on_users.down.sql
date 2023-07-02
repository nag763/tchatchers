-- Add down migration script here
-- Revert script to drop the stored procedure
DROP FUNCTION IF EXISTS update_last_logon(chatter_list chatter_logged_on[]);

-- Revert script to drop the composite type
DROP TYPE IF EXISTS chatter_logged_on;
