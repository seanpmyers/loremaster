CREATE TABLE
    "completed_action" (
        "action_id" uuid NOT NULL REFERENCES "action" ("id"),
        "person_id" uuid NOT NULL REFERENCES "person" ("id"),
        "chronicle_id" uuid NOT NULL REFERENCES "chronicle" ("id"),
        "completion_time" timestamp(0)
        with
            time zone NOT NULL,
            "duration" interval NULL,
            "intention_id" uuid NULL,
            PRIMARY KEY ("action_id")
    );

CREATE INDEX
    "completed_action_person_id_index" ON "completed_action" ("person_id");

CREATE INDEX
    "completed_action_chronicle_id_index" ON "completed_action" ("chronicle_id");
