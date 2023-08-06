CREATE TABLE
    "action" (
        "id" uuid NOT NULL PRIMARY KEY,
        "name" text NOT NULL UNIQUE
    );

CREATE INDEX "action_name_index" ON "action" ("name");
