CREATE TABLE
    "frequency"(
        "id" uuid NOT NULL UNIQUE PRIMARY KEY,
        "hour" integer NOT NULL,
        "day" integer NOT NULL,
        "week" integer NOT NUll,
        "month" integer NOT NUll,
        "year" integer NOT NUll,
        "subsequent" boolean NOT NULL DEFAULT FALSE
    );
