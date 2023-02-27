CREATE TABLE "person_principle" (
    "person_id" uuid NOT NULL,
    "principle_id" uuid NOT NULL
);

ALTER TABLE "person_principle"
    ADD PRIMARY KEY ("person_id", "principle_id");
