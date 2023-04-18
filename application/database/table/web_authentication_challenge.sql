CREATE TABLE
    "web_authentication_challenge" (
        "id" uuid NOT NULL PRIMARY KEY,
        "user_name" text NOT NULL,
        "passkey_registration" json NOT NULL
    );
