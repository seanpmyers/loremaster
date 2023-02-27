CREATE TABLE "goal" (
    "id" uuid NOT NULL,
    "name" text NOT NULL
);

CREATE INDEX "goal_name_index" ON "goal" ("name");

ALTER TABLE "goal"
    ADD PRIMARY KEY ("id");

ALTER TABLE "goal"
    ADD CONSTRAINT "goal_name_unique" UNIQUE ("name");
