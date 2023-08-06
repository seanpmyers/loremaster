CREATE TABLE
    "person_goal" (
        "person_id" uuid NOT NULL REFERENCES "person" ("id"),
        "goal_id" uuid NOT NULL REFERENCES "goal" ("id"),
        PRIMARY KEY ("person_id", "goal_id")
    );
