CREATE TABLE "completed_action" (
    "action_id" uuid NOT NULL,
    "completion_time" timestamp(0) with time zone NOT NULL,
    "person_id" uuid NOT NULL,
    "chronicle_id" uuid NOT NULL,
    "duration" interval NULL
);

CREATE INDEX "completed_action_person_id_index" ON "completed_action" ("person_id");

CREATE INDEX "completed_action_chronicle_id_index" ON "completed_action" ("chronicle_id");

ALTER TABLE "completed_action"
    ADD PRIMARY KEY ("action_id");
