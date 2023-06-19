CREATE TABLE
    "web_authentication_login" (
        "id" uuid NOT NULL PRIMARY KEY,
        "email_address" text NOT NULL,
        "passkey_authentication" jsonb NOT NULL
    );
