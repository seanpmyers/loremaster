CREATE TABLE "intention" (
    "id" uuid NOT NULL,
    "action_id" uuid NOT NULL,
    "person_id" uuid NOT NULL,
    "intended_time" timestamp(0) with time zone NULL,
    "complete_by" timestamp(0) with time zone NULL,
    "predicted_duration" interval NULL
);

ALTER TABLE "intention"
    ADD PRIMARY KEY ("id", "action_id");
