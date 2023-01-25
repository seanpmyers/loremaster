CREATE TABLE "person_action" (
    "action_id" uuid NOT NULL,
    "person_id" uuid NOT NULL,
    "average_duration" interval NULL
);

ALTER TABLE "person_action"
    ADD PRIMARY KEY ("action_id", "person_id");
