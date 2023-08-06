CREATE TABLE
    "person_skill" (
        "person_id" uuid NOT NULL REFERENCES "person" ("id"),
        "skill_id" uuid NOT NULL REFERENCES "skill" ("id"),
        PRIMARY KEY ("person_id", "skill_id")
    );
