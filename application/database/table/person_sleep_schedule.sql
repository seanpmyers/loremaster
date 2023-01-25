CREATE TABLE "person_sleep_schedule" (
    "person_id" uuid NOT NULL,
    "sleep_schedule_id" uuid NOT NULL
);

ALTER TABLE "person_sleep_schedule"
    ADD PRIMARY KEY ("person_id", "sleep_schedule_id");
