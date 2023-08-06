CREATE TABLE
    "schedule" (
        "id" uuid NOT NULL UNIQUE PRIMARY KEY,
        "frequency_interval" interval NOT NULL,
        "subsequent" boolean NOT NULL DEFAULT FALSE
    );
