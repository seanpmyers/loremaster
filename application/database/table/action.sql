CREATE TABLE "action" (
    "id" uuid NOT NULL,
    "name" text NOT NULL
);

CREATE INDEX "action_name_index" ON "action" ("name");

ALTER TABLE "action"
    ADD PRIMARY KEY ("id");

ALTER TABLE "action"
    ADD CONSTRAINT "action_name_unique" UNIQUE ("name");
