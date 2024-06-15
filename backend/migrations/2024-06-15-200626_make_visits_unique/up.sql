-- Drop any rows that are not unique, leaving the row with the earliest visited_at
WITH earliest_visits AS (
    SELECT MIN(id) AS id
    FROM visits
    GROUP BY user_id, location_id
)
DELETE FROM visits
WHERE id NOT IN (SELECT id FROM earliest_visits);

-- Add unique constraint on the visits table user and location id columns
ALTER TABLE visits ADD CONSTRAINT unique_user_location_id UNIQUE (user_id, location_id);
