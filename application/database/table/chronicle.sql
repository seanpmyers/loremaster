CREATE TABLE
    "chronicle" (
        "id" uuid NOT NULL UNIQUE,
        "person_id" uuid NOT NULL REFERENCES "person" ("id"),
        "date_recorded" date NOT NULL UNIQUE,
        "notes" text NULL,
        "creation_time" timestamp(0)
        with
            time zone NULL,
            PRIMARY KEY ("id", "date_recorded")
    );
