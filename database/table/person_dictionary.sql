CREATE TABLE
    "person_dictionary" (
        "person_id" uuid NOT NULL REFERENCES "person" ("id"),
        "dictionary_id" uuid NOT NULL REFERENCES "dictionary" ("id"),
        PRIMARY KEY ("person_id", "dictionary_id")
    );
