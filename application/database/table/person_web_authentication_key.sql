CREATE TABLE
    "person_web_authentication_key" (
        "person_id" uuid NOT NULL REFERENCES "person"("id"),
        "web_authentication_key_id" uuid NOT NULl REFERENCES "web_authentication_key"("id"),
        PRIMARY KEY (
            "person_id",
            "web_authentication_key_id"
        )
    );
