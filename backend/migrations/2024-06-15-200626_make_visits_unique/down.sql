-- Remove unique constraint on the visits table
ALTER TABLE visits DROP CONSTRAINT unique_user_location_id;
