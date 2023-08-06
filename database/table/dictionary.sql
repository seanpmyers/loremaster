CREATE TABLE
    "dictionary"(
        "id" uuid NOT NULL PRIMARY KEY,
        "name" text NOT NULL UNIQUE,
        "user_made" boolean NOT NULL DEFAULT FALSE
    );
