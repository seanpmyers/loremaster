CREATE TABLE "chronicle" (
    "id" uuid NOT NULL,
    "person_id" uuid NOT NULL,
    "date_recorded" date NOT NULL,
    "notes" text NULL,
    "creation_time" timestamp(0) with time zone NULL
);

ALTER TABLE "chronicle"
    ADD PRIMARY KEY ("id", "date_recorded");
