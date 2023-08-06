CREATE TABLE
    "intention_instance" (
        "id" uuid NOT NULL PRIMARY KEY,
        "intention_id" uuid NOT NULL REFERENCES "intention" ("id"),
        "date_and_time" timestamp(0)
        WITH
            time zone NOT NULL,
            "complete" boolean NOT NULL DEFAULT FALSE,
            "duration" interval NULL
    );
