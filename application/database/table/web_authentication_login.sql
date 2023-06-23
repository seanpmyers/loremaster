CREATE TABLE
    "web_authentication_login" (
        "id" uuid NOT NULL PRIMARY KEY,
        "user_name" text NOT NULL,
        "passkey" jsonb NOT NULL
    );
