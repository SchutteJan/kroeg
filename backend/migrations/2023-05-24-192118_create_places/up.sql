CREATE EXTENSION postgis;

CREATE TABLE locations (
    id SERIAL PRIMARY KEY,
    name varchar NOT NULL,
    coordinates geometry(Point, 4326) NOT NULL,
    published boolean NOT NULL DEFAULT FALSE,
    description text,
    osm_node_id varchar,
    google_place_id varchar
)