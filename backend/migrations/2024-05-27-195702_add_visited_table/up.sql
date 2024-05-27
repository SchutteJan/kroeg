CREATE TABLE visits (
    id SERIAL PRIMARY KEY,
    user_id integer NOT NULL REFERENCES users,
    location_id integer NOT NULL REFERENCES locations,
    visited_at TIMESTAMP NOT NULL DEFAULT timezone('UTC', now())
);
