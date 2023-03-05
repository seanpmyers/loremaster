CREATE TABLE
    "person_sleep_schedule" (
        "person_id" uuid NOT NULL UNIQUE REFERENCES "person" ("id"),
        "sleep_schedule_id" uuid NOT NULL REFERENCES "sleep_schedule" ("id"),
        PRIMARY KEY (
            "person_id",
            "sleep_schedule_id"
        )
    );
