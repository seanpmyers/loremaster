CREATE TABLE "chronicle_intention" (
    "chronicle_id" uuid NOT NULL,
    "intention_id" uuid NOT NULL,
    "person_id" uuid NOT NULL,
    "action_id" uuid NOT NULL
);

ALTER TABLE "chronicle_intention"
    ADD PRIMARY KEY ("chronicle_id", "intention_id", "person_id", "action_id");
