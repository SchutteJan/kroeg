-- Provide a default purely to make the migration not fail
ALTER TABLE locations ADD COLUMN address_line varchar NOT NULL DEFAULT 'No address provided';
ALTER TABLE locations ALTER COLUMN address_line DROP DEFAULT;
