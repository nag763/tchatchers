-- Add the "is_deleted" column to the "report" table with a default value of "false".
ALTER TABLE report
ADD COLUMN is_deleted BOOLEAN DEFAULT FALSE;

-- Modify the FOREIGN KEY constraint to update "is_deleted" instead of cascading deletion.
ALTER TABLE report
DROP CONSTRAINT fk_message, -- Drop the existing constraint if it exists.
ADD CONSTRAINT fk_message
FOREIGN KEY (message_uuid)
REFERENCES public.message (uuid) MATCH SIMPLE
ON UPDATE NO ACTION
ON DELETE SET DEFAULT; -- Set the "is_deleted" column to true.

-- Modify the "is_something_deleted" constraint to consider deleted records.
ALTER TABLE report
DROP CONSTRAINT IF EXISTS something_reported, -- Drop the existing constraint if it exists.
ADD CONSTRAINT something_reported
CHECK (reported_id IS NOT NULL OR message_uuid IS NOT NULL OR is_deleted IS TRUE);

-- Create a BEFORE DELETE trigger on the "message" table to update records in "reported" before deletion.

-- Ensure you have the "is_deleted" column in the "reported" table to mark records as deleted.

CREATE OR REPLACE FUNCTION before_message_delete()
RETURNS TRIGGER AS $$
BEGIN
  -- Update records in "reported" where "message_uuid" matches the UUID of the message being deleted.
  UPDATE report
  SET is_deleted = TRUE
  WHERE message_uuid = OLD.uuid; -- OLD.uuid represents the UUID of the record being deleted in "message".
  RETURN OLD;
END;
$$ LANGUAGE plpgsql;

-- Create the BEFORE DELETE trigger on the "message" table to invoke the before_message_delete function.
CREATE TRIGGER trigger_before_message_delete
BEFORE DELETE
ON message
FOR EACH ROW
EXECUTE FUNCTION before_message_delete();
