CREATE TABLE
    "person_password" (
        "person_id" uuid NOT NULL REFERENCES "person" ("id"),
        "password_id" uuid NOT NULL REFERENCES "password" ("id"),
        PRIMARY KEY ("person_id", "password_id")
    );
