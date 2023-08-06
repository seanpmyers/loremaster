CREATE TABLE
    "person_email_address" (
        "person_id" uuid NOT NULL REFERENCES "person" ("id"),
        "email_address_id" uuid NOT NULL REFERENCES "email_address" ("id"),
        PRIMARY KEY (
            "person_id",
            "email_address_id"
        )
    );
