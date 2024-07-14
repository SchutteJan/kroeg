-- buurt: neighbourhood, wijk: district, gebied: area, stadsdeel: borough
CREATE TYPE area_type AS ENUM('neighbourhood', 'district', 'area', 'borough');

CREATE TABLE areas (
    id SERIAL PRIMARY KEY,
    name varchar NOT NULL,
    area GEOMETRY NOT NULL,
    area_type area_type NOT NULL
);
