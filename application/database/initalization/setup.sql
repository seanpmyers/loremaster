CREATE TABLE
    "person"(
        "id" UUID NOT NULL,
        "email_address_id" UUID NOT NULL,
        "encrypted_password" TEXT NOT NULL,
        "registration_date" TIMESTAMP(0)
        WITH
            TIME zone NOT NULL,
            "alias" TEXT NULL,
            "chronicle_id" UUID NULL
    );

CREATE INDEX "person_alias_index" ON "person"("alias");

CREATE INDEX
    "person_registration_date_index" ON "person"("registration_date");

ALTER TABLE "person" ADD PRIMARY KEY("id");

ALTER TABLE "person"
ADD
    CONSTRAINT "person_email_address_id_unique" UNIQUE("email_address_id");

CREATE TABLE
    "chronicle"(
        "id" UUID NOT NULL,
        "person_id" UUID NOT NULL,
        "date_recorded" DATE NOT NULL,
        "notes" TEXT NULL,
        "creation_time" TIMESTAMP(0)
        WITH TIME zone NULL
    );

ALTER TABLE "chronicle" ADD PRIMARY KEY("id", "date_recorded");

CREATE TABLE
    "action"(
        "id" UUID NOT NULL,
        "name" TEXT NOT NULL
    );

CREATE INDEX "action_name_index" ON "action"("name");

ALTER TABLE "action" ADD PRIMARY KEY("id");

ALTER TABLE "action"
ADD
    CONSTRAINT "action_name_unique" UNIQUE("name");

CREATE TABLE
    "intention"(
        "id" UUID NOT NULL,
        "action_id" UUID NOT NULL,
        "person_id" UUID NOT NULL,
        "intended_time" TIMESTAMP(0)
        WITH TIME zone NULL
    );

ALTER TABLE "intention" ADD PRIMARY KEY("id", "action_id");

CREATE TABLE
    "goal"(
        "id" UUID NOT NULL,
        "name" TEXT NOT NULL
    );

CREATE INDEX "goal_name_index" ON "goal"("name");

ALTER TABLE "goal" ADD PRIMARY KEY("id");

ALTER TABLE "goal"
ADD
    CONSTRAINT "goal_name_unique" UNIQUE("name");

CREATE TABLE
    "person_action"(
        "action_id" UUID NOT NULL,
        "person_id" UUID NOT NULL
    );

ALTER TABLE "person_action"
ADD
    PRIMARY KEY("action_id", "person_id");

CREATE TABLE
    "completed_action"(
        "action_id" UUID NOT NULL,
        "completion_time" TIMESTAMP(0)
        WITH
            TIME zone NOT NULL,
            "person_id" UUID NOT NULL,
            "chronicle_id" UUID NOT NULL
    );

CREATE INDEX
    "completed_action_person_id_index" ON "completed_action"("person_id");

CREATE INDEX
    "completed_action_chronicle_id_index" ON "completed_action"("chronicle_id");

ALTER TABLE "completed_action" ADD PRIMARY KEY("action_id");

CREATE TYPE frequency_unit AS ENUM (
    'Day',
    'Month',
    'Year',
    'Hour',
    'Minute'
);

CREATE TABLE
    "frequency"(
        "id" UUID NOT NULL,
        "unit" frequency_unit NOT NULL
    );

CREATE INDEX "frequency_unit_index" ON "frequency"("unit");

ALTER TABLE "frequency" ADD PRIMARY KEY("id");

ALTER TABLE "frequency"
ADD
    CONSTRAINT "frequency_unit_unique" UNIQUE("unit");

CREATE TABLE
    "intention_frequency"(
        "intention_id" UUID NOT NULL,
        "frequency_id" UUID NOT NULL
    );

ALTER TABLE
    "intention_frequency"
ADD
    PRIMARY KEY(
        "intention_id",
        "frequency_id"
    );

CREATE TABLE
    "person_goal"(
        "person_id" UUID NOT NULL,
        "goal_id" UUID NOT NULL
    );

ALTER TABLE "person_goal"
ADD
    PRIMARY KEY("person_id", "goal_id");

CREATE TABLE
    "email_address"(
        "id" UUID NOT NULL,
        "display" TEXT NOT NULL,
        "local_part" TEXT NOT NULL,
        "domain" TEXT NOT NULL,
        "validated" BOOLEAN NOT NULL DEFAULT '0',
        "validation_date" TIMESTAMP(0)
        WITH
            TIME zone NULL,
            "creation_date" TIMESTAMP(0)
        WITH TIME zone NOT NULL
    );

CREATE INDEX
    "email_address_domain_index" ON "email_address"("domain");

CREATE INDEX
    "email_address_display_index" ON "email_address"("display");

CREATE INDEX
    "email_address_validated_index" ON "email_address"("validated");

ALTER TABLE "email_address" ADD PRIMARY KEY("id");

CREATE TABLE
    "person_email_address"(
        "person_id" UUID NOT NULL,
        "email_address_id" UUID NOT NULL
    );

ALTER TABLE
    "person_email_address"
ADD
    PRIMARY KEY(
        "person_id",
        "email_address_id"
    );

CREATE TABLE
    "principle"(
        "id" UUID NOT NULL,
        "name" TEXT NOT NULL,
        "reason" TEXT
    );

CREATE INDEX "principle_name_index" ON "principle"("name");

ALTER TABLE "principle" ADD PRIMARY KEY("id");

ALTER TABLE "principle"
ADD
    CONSTRAINT "principle_name_unique" UNIQUE("name");

CREATE TABLE
    "person_principle"(
        "person_id" UUID NOT NULL,
        "principle_id" UUID NOT NULL
    );

ALTER TABLE "person_principle"
ADD
    PRIMARY KEY("person_id", "principle_id");

CREATE TABLE
    "chronicle_intention"(
        "chronicle_id" UUID NOT NULL,
        "intention_id" UUID NOT NULL,
        "person_id" UUID NOT NULL,
        "action_id" UUID NOT NULL
    );

ALTER TABLE
    "chronicle_intention"
ADD
    PRIMARY KEY(
        "chronicle_id",
        "intention_id",
        "person_id",
        "action_id"
    );

CREATE TABLE
    "sleep_schedule"(
        "id" UUID NOT NULl,
        "start_time" TIME(0) WITHOUT TIME ZONE NOT NULL,
        "end_time" TIME(0) WITHOUT TIME ZONE NOT NULL
    );

ALTER TABLE "sleep_schedule" ADD PRIMARY KEY("id");

CREATE TABLE
    "person_sleep_schedule"(
        "person_id" UUID NOT NULL,
        "sleep_schedule_id" UUID NOT NULL
    );

ALTER TABLE
    "person_sleep_schedule"
ADD
    PRIMARY KEY(
        "person_id",
        "sleep_schedule_id"
    );

CREATE TABLE
    "skill"(
        "id" UUID NOT NULL,
        "name" TEXT NOT NULL
    );

ALTER TABLE
    "skill"
ADD
    PRIMARY KEY("id");

ALTER TABLE
    "skill"
ADD 
    CONSTRAINT "skill_unique" UNIQUE("name");

CREATE TABLE
    "person_skill"(
        "person_id" UUID NOT NULL,
        "skill_id" UUID NOT NULL
    );

ALTER TABLE
    "person_skill"
ADD
    PRIMARY KEY("person_id", "skill_id");

ALTER TABLE
    "person_sleep_schedule"
ADD
    CONSTRAINT "person_id_unique" UNIQUE("person_id");

ALTER TABLE
    "person_skill"
ADD
    CONSTRAINT "person_foreign_key" FOREIGN KEY("person_id") REFERENCES "person"("id");

ALTER TABLE
    "person_skill"
ADD
    CONSTRAINT "skill_foreign_key" FOREIGN KEY("skill_id") REFERENCES "skill"("id");

ALTER TABLE
    "chronicle_intention"
ADD
    CONSTRAINT "chronicle_intention_action_id_foreign" FOREIGN KEY("action_id") REFERENCES "action"("id");

ALTER TABLE "chronicle"
ADD
    CONSTRAINT "chronicle_person_id_foreign" FOREIGN KEY("person_id") REFERENCES "person"("id");

ALTER TABLE "person_goal"
ADD
    CONSTRAINT "person_goal_person_id_foreign" FOREIGN KEY("person_id") REFERENCES "person"("id");

ALTER TABLE
    "person_email_address"
ADD
    CONSTRAINT "person_email_address_person_id_foreign" FOREIGN KEY("person_id") REFERENCES "person"("id");

ALTER TABLE "person_principle"
ADD
    CONSTRAINT "person_principle_person_id_foreign" FOREIGN KEY("person_id") REFERENCES "person"("id");

ALTER TABLE
    "chronicle_intention"
ADD
    CONSTRAINT "chronicle_intention_person_id_foreign" FOREIGN KEY("person_id") REFERENCES "person"("id");

ALTER TABLE "person"
ADD
    CONSTRAINT "person_email_address_id_foreign" FOREIGN KEY("email_address_id") REFERENCES "email_address"("id");

ALTER TABLE "intention"
ADD
    CONSTRAINT "intention_action_id_foreign" FOREIGN KEY("action_id") REFERENCES "action"("id");

ALTER TABLE "person_goal"
ADD
    CONSTRAINT "person_goal_goal_id_foreign" FOREIGN KEY("goal_id") REFERENCES "goal"("id");

ALTER TABLE "person_action"
ADD
    CONSTRAINT "person_action_action_id_foreign" FOREIGN KEY("action_id") REFERENCES "action"("id");

ALTER TABLE "person_action"
ADD
    CONSTRAINT "person_action_person_id_foreign" FOREIGN KEY("person_id") REFERENCES "person"("id");

ALTER TABLE "completed_action"
ADD
    CONSTRAINT "completed_action_person_id_foreign" FOREIGN KEY("person_id") REFERENCES "person"("id");

ALTER TABLE
    "intention_frequency"
ADD
    CONSTRAINT "intention_frequency_frequency_id_foreign" FOREIGN KEY("frequency_id") REFERENCES "frequency"("id");

ALTER TABLE
    "person_email_address"
ADD
    CONSTRAINT "person_email_address_email_address_id_foreign" FOREIGN KEY("email_address_id") REFERENCES "email_address"("id");

ALTER TABLE "person_principle"
ADD
    CONSTRAINT "person_principle_principle_id_foreign" FOREIGN KEY("principle_id") REFERENCES "principle"("id");

ALTER TABLE
    "person_sleep_schedule"
ADD
    CONSTRAINT "person_id_foreign_key" FOREIGN KEY("person_id") REFERENCES "person"("id");

ALTER TABLE
    "person_sleep_schedule"
ADD
    CONSTRAINT "sleep_schedule_id_foreign_key" FOREIGN KEY("sleep_schedule_id") REFERENCES "sleep_schedule"("id");

INSERT INTO
    frequency
VALUES
    ((select gen_random_uuid ()), 'Day'),
    ((select gen_random_uuid ()), 'Month'),
    ((select gen_random_uuid ()), 'Year'),
    ((select gen_random_uuid ()), 'Minute'),
    ((select gen_random_uuid ()), 'Hour')
;
