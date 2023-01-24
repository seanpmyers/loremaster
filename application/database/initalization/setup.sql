CREATE TABLE "person" (
    "id" uuid NOT NULL,
    "email_address_id" uuid NOT NULL,
    "encrypted_password" text NOT NULL,
    "registration_date" timestamp(0) with time zone NOT NULL,
    "alias" text NULL,
    "chronicle_id" uuid NULL
);

CREATE INDEX "person_alias_index" ON "person" ("alias");

CREATE INDEX "person_registration_date_index" ON "person" ("registration_date");

ALTER TABLE "person"
    ADD PRIMARY KEY ("id");

ALTER TABLE "person"
    ADD CONSTRAINT "person_email_address_id_unique" UNIQUE ("email_address_id");

CREATE TABLE "chronicle" (
    "id" uuid NOT NULL,
    "person_id" uuid NOT NULL,
    "date_recorded" date NOT NULL,
    "notes" text NULL,
    "creation_time" timestamp(0) with time zone NULL
);

ALTER TABLE "chronicle"
    ADD PRIMARY KEY ("id", "date_recorded");

CREATE TABLE "action" (
    "id" uuid NOT NULL,
    "name" text NOT NULL
);

CREATE INDEX "action_name_index" ON "action" ("name");

ALTER TABLE "action"
    ADD PRIMARY KEY ("id");

ALTER TABLE "action"
    ADD CONSTRAINT "action_name_unique" UNIQUE ("name");

CREATE TABLE "intention" (
    "id" uuid NOT NULL,
    "action_id" uuid NOT NULL,
    "person_id" uuid NOT NULL,
    "intended_time" timestamp(0) with time zone NULL,
    "predicted_duration" interval NULL
);

ALTER TABLE "intention"
    ADD PRIMARY KEY ("id", "action_id");

CREATE TABLE "goal" (
    "id" uuid NOT NULL,
    "name" text NOT NULL
);

CREATE INDEX "goal_name_index" ON "goal" ("name");

ALTER TABLE "goal"
    ADD PRIMARY KEY ("id");

ALTER TABLE "goal"
    ADD CONSTRAINT "goal_name_unique" UNIQUE ("name");

CREATE TABLE "person_action" (
    "action_id" uuid NOT NULL,
    "person_id" uuid NOT NULL,
    "average_duration" interval NULL
);

ALTER TABLE "person_action"
    ADD PRIMARY KEY ("action_id", "person_id");

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

CREATE TYPE frequency AS ENUM (
    'Day',
    'Weekday',
    'Week',
    'Month',
    'Year',
    'Hour',
    'Minute'
);

CREATE TYPE weekday AS ENUM (
    'Sunday',
    'Monday',
    'Tuesday',
    'Wednesday',
    'Thursday',
    'Friday',
    'Saturday'
);

CREATE TABLE "intention_frequency" (
    "intention_id" uuid NOT NULL,
    "frequency" frequency NOT NULL
);

ALTER TABLE "intention_frequency"
    ADD PRIMARY KEY ("intention_id", "frequency");

CREATE TABLE "person_goal" (
    "person_id" uuid NOT NULL,
    "goal_id" uuid NOT NULL
);

ALTER TABLE "person_goal"
    ADD PRIMARY KEY ("person_id", "goal_id");

CREATE TABLE "email_address" (
    "id" uuid NOT NULL,
    "display" text NOT NULL,
    "local_part" text NOT NULL,
    "domain" text NOT NULL,
    "validated" boolean NOT NULL DEFAULT '0',
    "validation_date" timestamp(0) with time zone NULL,
    "creation_date" timestamp(0) with time zone NOT NULL
);

CREATE INDEX "email_address_domain_index" ON "email_address" ("domain");

CREATE INDEX "email_address_display_index" ON "email_address" ("display");

CREATE INDEX "email_address_validated_index" ON "email_address" ("validated");

ALTER TABLE "email_address"
    ADD PRIMARY KEY ("id");

CREATE TABLE "person_email_address" (
    "person_id" uuid NOT NULL,
    "email_address_id" uuid NOT NULL
);

ALTER TABLE "person_email_address"
    ADD PRIMARY KEY ("person_id", "email_address_id");

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

CREATE TABLE "person_principle" (
    "person_id" uuid NOT NULL,
    "principle_id" uuid NOT NULL
);

ALTER TABLE "person_principle"
    ADD PRIMARY KEY ("person_id", "principle_id");

CREATE TABLE "chronicle_intention" (
    "chronicle_id" uuid NOT NULL,
    "intention_id" uuid NOT NULL,
    "person_id" uuid NOT NULL,
    "action_id" uuid NOT NULL
);

ALTER TABLE "chronicle_intention"
    ADD PRIMARY KEY ("chronicle_id", "intention_id", "person_id", "action_id");

CREATE TABLE "sleep_schedule" (
    "id" uuid NOT NULL,
    "start_time" time(0) without time zone NOT NULL,
    "end_time" time(0) without time zone NOT NULL
);

ALTER TABLE "sleep_schedule"
    ADD PRIMARY KEY ("id");

CREATE TABLE "person_sleep_schedule" (
    "person_id" uuid NOT NULL,
    "sleep_schedule_id" uuid NOT NULL
);

ALTER TABLE "person_sleep_schedule"
    ADD PRIMARY KEY ("person_id", "sleep_schedule_id");

CREATE TABLE "skill" (
    "id" uuid NOT NULL,
    "name" text NOT NULL
);

ALTER TABLE "skill"
    ADD PRIMARY KEY ("id");

ALTER TABLE "skill"
    ADD CONSTRAINT "skill_unique" UNIQUE ("name");

CREATE TABLE "person_skill" (
    "person_id" uuid NOT NULL,
    "skill_id" uuid NOT NULL
);

ALTER TABLE "person_skill"
    ADD PRIMARY KEY ("person_id", "skill_id");

ALTER TABLE "person_sleep_schedule"
    ADD CONSTRAINT "person_id_unique" UNIQUE ("person_id");

ALTER TABLE "person_skill"
    ADD CONSTRAINT "person_foreign_key" FOREIGN KEY ("person_id") REFERENCES "person" ("id");

ALTER TABLE "person_skill"
    ADD CONSTRAINT "skill_foreign_key" FOREIGN KEY ("skill_id") REFERENCES "skill" ("id");

ALTER TABLE "chronicle_intention"
    ADD CONSTRAINT "chronicle_intention_action_id_foreign" FOREIGN KEY ("action_id") REFERENCES "action" ("id");

ALTER TABLE "chronicle"
    ADD CONSTRAINT "chronicle_person_id_foreign" FOREIGN KEY ("person_id") REFERENCES "person" ("id");

ALTER TABLE "person_goal"
    ADD CONSTRAINT "person_goal_person_id_foreign" FOREIGN KEY ("person_id") REFERENCES "person" ("id");

ALTER TABLE "person_email_address"
    ADD CONSTRAINT "person_email_address_person_id_foreign" FOREIGN KEY ("person_id") REFERENCES "person" ("id");

ALTER TABLE "person_principle"
    ADD CONSTRAINT "person_principle_person_id_foreign" FOREIGN KEY ("person_id") REFERENCES "person" ("id");

ALTER TABLE "chronicle_intention"
    ADD CONSTRAINT "chronicle_intention_person_id_foreign" FOREIGN KEY ("person_id") REFERENCES "person" ("id");

ALTER TABLE "person"
    ADD CONSTRAINT "person_email_address_id_foreign" FOREIGN KEY ("email_address_id") REFERENCES "email_address" ("id");

ALTER TABLE "intention"
    ADD CONSTRAINT "intention_action_id_foreign" FOREIGN KEY ("action_id") REFERENCES "action" ("id");

ALTER TABLE "person_goal"
    ADD CONSTRAINT "person_goal_goal_id_foreign" FOREIGN KEY ("goal_id") REFERENCES "goal" ("id");

ALTER TABLE "person_action"
    ADD CONSTRAINT "person_action_action_id_foreign" FOREIGN KEY ("action_id") REFERENCES "action" ("id");

ALTER TABLE "person_action"
    ADD CONSTRAINT "person_action_person_id_foreign" FOREIGN KEY ("person_id") REFERENCES "person" ("id");

ALTER TABLE "completed_action"
    ADD CONSTRAINT "completed_action_person_id_foreign" FOREIGN KEY ("person_id") REFERENCES "person" ("id");

ALTER TABLE "person_email_address"
    ADD CONSTRAINT "person_email_address_email_address_id_foreign" FOREIGN KEY ("email_address_id") REFERENCES "email_address" ("id");

ALTER TABLE "person_principle"
    ADD CONSTRAINT "person_principle_principle_id_foreign" FOREIGN KEY ("principle_id") REFERENCES "principle" ("id");

ALTER TABLE "person_sleep_schedule"
    ADD CONSTRAINT "person_id_foreign_key" FOREIGN KEY ("person_id") REFERENCES "person" ("id");

ALTER TABLE "person_sleep_schedule"
    ADD CONSTRAINT "sleep_schedule_id_foreign_key" FOREIGN KEY ("sleep_schedule_id") REFERENCES "sleep_schedule" ("id");

