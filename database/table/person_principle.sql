CREATE TABLE
    "person_principle" (
        "person_id" uuid NOT NULL REFERENCES "person" ("id"),
        "principle_id" uuid NOT NULL REFERENCES "principle" ("id"),
        PRIMARY KEY ("person_id", "principle_id")
    );
