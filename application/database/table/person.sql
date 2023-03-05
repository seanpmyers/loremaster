CREATE TABLE
    "person" (
        "id" uuid NOT NULL PRIMARY KEY,
        "email_address_id" uuid NOT NULL UNIQUE REFERENCES "email_address" ("id"),
        "encrypted_password" text NOT NULL,
        "registration_date" timestamp(0)
        with
            time zone NOT NULL,
            "alias" text NULL,
            "chronicle_id" uuid NULL UNIQUE
    );

CREATE INDEX "person_alias_index" ON "person" ("alias");

CREATE INDEX
    "person_registration_date_index" ON "person" ("registration_date");
