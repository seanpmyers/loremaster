CREATE TABLE
    "person_passkey" (
        "person_id" uuid NOT NULL REFERENCES "person"("id"),
        "passkey_id" uuid NOT NULl REFERENCES "passkey"("id"),
        PRIMARY KEY ("person_id", "passkey_id")
    );
