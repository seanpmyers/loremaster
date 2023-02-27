CREATE TABLE "sleep_schedule" (
    "id" uuid NOT NULL,
    "start_time" time(0) without time zone NOT NULL,
    "end_time" time(0) without time zone NOT NULL
);

ALTER TABLE "sleep_schedule"
    ADD PRIMARY KEY ("id");
