CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE posts (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    title TEXT NOT NULL,
    content TEXT NOT NULL,
    latitude NUMERIC NOT NULL,
    longitude NUMERIC NOT NULL,
    created_at TIMESTAMP NOT NULL
);

create or replace function haversine(lat numeric, lng numeric, other_lat numeric, other_lng numeric)
    returns numeric 
    language plpgsql as $$
begin
    return 6371000 *
            acos(cos(radians(other_lat)) * 
            cos(radians(lat)) * 
            cos(radians(lng) - 
            radians(other_lng)) + 
            sin(radians(other_lat)) * 
            sin(radians(lat)));
end $$;
