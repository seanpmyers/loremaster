CREATE TABLE "principle" (
    "id" uuid NOT NULL,
    "name" text NOT NULL,
    "reason" text
);

CREATE INDEX "principle_name_index" ON "principle" ("name");

ALTER TABLE "principle"
    ADD PRIMARY KEY ("id");

ALTER TABLE "principle"
    ADD CONSTRAINT "principle_name_unique" UNIQUE ("name");
