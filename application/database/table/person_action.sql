CREATE TABLE
    "person_action" (
        "action_id" uuid NOT NULL REFERENCES "action" ("id"),
        "person_id" uuid NOT NULL REFERENCES "person" ("id"),
        "average_duration" interval NULL,
        PRIMARY KEY ("action_id", "person_id")
    );
