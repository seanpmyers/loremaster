CREATE TABLE
    "person_intention_schedule" (
        "person_id" uuid NOT NULL REFERENCES "person" ("id"),
        "intention_id" uuid NOT NULL REFERENCES "intention" ("id"),
        "schedule_id" uuid NOT NULL REFERENCES "schedule" ("id"),
        "schedule_weekday_id" uuid NOT NULL REFERENCES "schedule_weekday" ("id"),
        PRIMARY KEY (
            "person_id",
            "intention_id",
            "schedule_id"
        )
    );
