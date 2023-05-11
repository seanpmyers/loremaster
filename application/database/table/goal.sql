CREATE TABLE
    "goal" (
        "id" uuid NOT NULL PRIMARY KEY,
        "name" text UNIQUE NOT NULL
    );

CREATE INDEX "goal_name_index" ON "goal" ("name");
