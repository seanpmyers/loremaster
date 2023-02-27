CREATE TABLE "skill" (
    "id" uuid NOT NULL,
    "name" text NOT NULL
);

ALTER TABLE "skill"
    ADD PRIMARY KEY ("id");

ALTER TABLE "skill"
    ADD CONSTRAINT "skill_unique" UNIQUE ("name");
