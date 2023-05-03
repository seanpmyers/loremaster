CREATE TABLE
    "web_authentication_key" (
        "id" uuid NOT NULL PRIMARY KEY,
        "credential_id" bytea NOT NULL,
        "cose_algorithm" int NOT NULL,
        "passkey" jsonb NOT NULL
    );
