-- Add up migration script here
CREATE TABLE DELETED_RECORD(
    ID SERIAL PRIMARY KEY,
    ORIGIN TEXT NOT NULL,
    RECORD_ID INTEGER,
    RECORD_LOGIN TEXT,
    RECORD_CREATED_AT TIMESTAMPTZ,
    RECORD_UUID UUID,
    RECORD_ROOM TEXT,
    DELETED_AT TIMESTAMPTZ DEFAULT NOW()
);

-- Create the trigger function
CREATE OR REPLACE FUNCTION chatters_delete_trigger_function()
  RETURNS TRIGGER AS
$$
BEGIN
  -- Insert the deleted record into the DELETED_RECORD table
  INSERT INTO DELETED_RECORD (ORIGIN, RECORD_ID, RECORD_LOGIN, RECORD_CREATED_AT)
  VALUES ('CHATTER', OLD.id, OLD.login, OLD.created_at);

  RETURN OLD;
END;
$$
LANGUAGE plpgsql;

-- Create the trigger
CREATE TRIGGER chatters_delete_trigger
AFTER DELETE ON CHATTER
FOR EACH ROW
EXECUTE FUNCTION chatters_delete_trigger_function();

-- Create the trigger function
CREATE OR REPLACE FUNCTION messages_delete_trigger_function()
  RETURNS TRIGGER AS
$$
BEGIN
  -- Insert the deleted record into the DELETED_RECORD table
  INSERT INTO DELETED_RECORD (ORIGIN, RECORD_UUID, RECORD_ROOM, RECORD_CREATED_AT)
  VALUES ('MESSAGE', OLD.uuid, OLD.ROOM, OLD.timestamp);

  RETURN OLD;
END;
$$
LANGUAGE plpgsql;

-- Create the trigger
CREATE TRIGGER messages_delete_trigger
AFTER DELETE ON MESSAGE
FOR EACH ROW
EXECUTE FUNCTION messages_delete_trigger_function();
