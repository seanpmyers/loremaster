CREATE TABLE
    "intention" (
        "id" uuid NOT NULL PRIMARY KEY,
        "action_id" uuid NOT NULL REFERENCES "action" ("id"),
        "person_id" uuid NOT NULL REFERENCES "person" ("id"),
        "created_on_date_and_time" timestamp(0)
        with
            time zone NOT NULl,
            "intended_date_and_time" timestamp(0)
        with
            time zone NULL,
            "complete_by_date_and_time" timestamp(0)
        with
            time zone NULL,
            "predicted_duration" interval NULL
    );
