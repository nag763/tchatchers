-- Remove the "is_deleted" column from the "report" table.
ALTER TABLE report
DROP COLUMN is_deleted;

-- Restore the FOREIGN KEY constraint for cascade deletion.
ALTER TABLE report
DROP CONSTRAINT fk_message, -- Drop the existing constraint if it exists.
ADD CONSTRAINT fk_message
FOREIGN KEY (message_uuid)
REFERENCES public.message (uuid) MATCH SIMPLE
ON UPDATE NO ACTION
ON DELETE CASCADE; -- Revert to cascade deletion.

-- Restore the "is_something_deleted" constraint to not consider "is_deleted".
ALTER TABLE report
DROP CONSTRAINT IF EXISTS something_reported, -- Drop the existing constraint if it exists.
ADD CONSTRAINT something_reported
CHECK (reported_id IS NOT NULL OR message_uuid IS NOT NULL);

-- Remove the BEFORE DELETE trigger on the "message" table.

-- Remove the trigger.
DROP TRIGGER IF EXISTS trigger_before_message_delete ON message;

-- Remove the associated function.
DROP FUNCTION IF EXISTS before_message_delete();
