-- Create location data tables
CREATE TABLE locations (
    id uuid NOT NULL,
    crowd_sourced BOOLEAN NOT NULL,
    position_type TEXT NOT NULL,
    vertical_accuracy INT NOT NULL,
    longitude DOUBLE PRECISION NOT NULL,
    floor_level INT NOT NULL,
    is_inaccurate BOOLEAN NOT NULL,
    is_old BOOLEAN NOT NULL,
    horizontal_accuracy DOUBLE PRECISION NOT NULL,
    latitude DOUBLE PRECISION NOT NULL,
    timestamp timestamptz NOT NULL,
    altitude INT NOT NULL,
    location_finished BOOLEAN NOT NULL,

    PRIMARY KEY(id)
);

CREATE TABLE addresses (
    id uuid NOT NULL,
    sub_administrative_area TEXT NOT NULL,
    label TEXT NOT NULL,
    street_address TEXT NOT NULL,
    country_code TEXT NOT NULL,
    state_code TEXT NOT NULL,
    administrative_area TEXT NOT NULL,
    street_name TEXT NOT NULL,
    formatted_address_lines TEXT[] NOT NULL,
    map_item_full_address TEXT NOT NULL,
    full_throroughfare TEXT NOT NULL,
    area_of_interest TEXT[] NOT NULL,
    locality TEXT NOT NULL,
    country TEXT NOT NULL,

    PRIMARY KEY(id)
);

CREATE TABLE location_snapshots (
    id uuid NOT NULL,
    location_id uuid NOT NULL REFERENCES locations(id),
    crowd_sourced_location_id uuid NOT NULL REFERENCES locations(id),
    address_id uuid NOT NULL REFERENCES addresses(id),
    server_timestamp timestamptz NOT NULL,

    PRIMARY KEY(id)
);
