CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE posts (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    title TEXT NOT NULL,
    content TEXT NOT NULL,
    latitude DOUBLE PRECISION NOT NULL,
    longitude DOUBLE PRECISION NOT NULL,
    created_at TIMESTAMP NOT NULL
);

create or replace function haversine(lat double precision, lng double precision, other_lat double precision, other_lng double precision)
    returns numeric 
    language plpgsql as $$
begin
    return 6371000 *
            acos(least(greatest(cos(radians(other_lat)) * 
            cos(radians(lat)) * 
            cos(radians(lng) - 
            radians(other_lng)) + 
            sin(radians(other_lat)) * 
            sin(radians(lat)), -1), 1));
end $$;
