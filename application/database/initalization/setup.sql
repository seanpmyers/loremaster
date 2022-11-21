CREATE TABLE "person"(
    "id" UUID NOT NULL,
    "email_address" TEXT NOT NULL,
    "encrypted_password" TEXT NOT NULL,
    "registration_date" TIMESTAMP(0) WITH
        TIME zone NOT NULL,
        "alias" TEXT NULL,
        "chronicle_id" UUID NULL
);
ALTER TABLE
    "person" ADD PRIMARY KEY("id");
ALTER TABLE
    "person" ADD CONSTRAINT "person_email_address_unique" UNIQUE("email_address");
CREATE TABLE "chronicle"(
    "id" UUID NOT NULL,
    "person_id" UUID NOT NULL,
    "date_recorded" DATE NOT NULL,
    "notes" TEXT NULL,
    "creation_time" TIMESTAMP(0) WITH
        TIME zone NULL
);
ALTER TABLE
    "chronicle" ADD PRIMARY KEY("id", "date_recorded");
CREATE TABLE "action"(
    "id" UUID NOT NULL,
    "name" TEXT NOT NULL
);
ALTER TABLE
    "action" ADD PRIMARY KEY("id");
ALTER TABLE
    "action" ADD CONSTRAINT "action_name_unique" UNIQUE("name");
CREATE TABLE "intention"(
    "id" UUID NOT NULL,
    "action_id" UUID NOT NULL,
    "person_id" UUID NOT NULL,
    "intended_time" TIMESTAMP(0) WITH
        TIME zone NULL,
        "chronicle_id" UUID NULL
);
ALTER TABLE
    "intention" ADD PRIMARY KEY("id", "action_id");
CREATE TABLE "conviction"(
    "id" UUID NOT NULL,
    "name" TEXT NOT NULL
);
ALTER TABLE
    "conviction" ADD PRIMARY KEY("id");
ALTER TABLE
    "conviction" ADD CONSTRAINT "conviction_name_unique" UNIQUE("name");
CREATE TABLE "person_conviction"(
    "person_id" UUID NOT NULL,
    "conviction_id" UUID NOT NULL
);
ALTER TABLE
    "person_conviction" ADD PRIMARY KEY("person_id", "conviction_id");
CREATE TABLE "objective"(
    "id" UUID NOT NULL,
    "name" TEXT NOT NULL
);
ALTER TABLE
    "objective" ADD PRIMARY KEY("id");
ALTER TABLE
    "objective" ADD CONSTRAINT "objective_name_unique" UNIQUE("name");
CREATE TABLE "person_action"(
    "action_id" UUID NOT NULL,
    "person_id" UUID NOT NULL
);
ALTER TABLE
    "person_action" ADD PRIMARY KEY("action_id", "person_id");
CREATE TABLE "achievement"(
    "id" UUID NOT NULL,
    "objective_id" UUID NOT NULL,
    "achieved_time" TIMESTAMP(0) WITH
        TIME zone NULL,
        "chronicle_id" UUID NULL
);
ALTER TABLE
    "achievement" ADD PRIMARY KEY("id");
CREATE TABLE "conviction_objective"(
    "conviction_id" UUID NOT NULL,
    "objective_id" UUID NOT NULL
);
ALTER TABLE
    "conviction_objective" ADD PRIMARY KEY("conviction_id", "objective_id");
CREATE TABLE "objective_action"(
    "objective_id" UUID NOT NULL,
    "action_id" UUID NOT NULL,
    "required" BOOLEAN NOT NULL,
    "minimum_count" INTEGER NOT NULL,
    "completion_time" TIMESTAMP(0) WITH
        TIME zone NULL,
        "chronicle_id" UUID NULL
);
ALTER TABLE
    "objective_action" ADD PRIMARY KEY("objective_id", "action_id");
CREATE TABLE "completed_action"(
    "action_id" UUID NOT NULL,
    "completion_time" TIMESTAMP(0) WITH
        TIME zone NOT NULL,
        "person_id" UUID NOT NULL,
        "chronicle_id" UUID NOT NULL
);
ALTER TABLE
    "completed_action" ADD PRIMARY KEY("action_id");
CREATE TABLE "completed_intention"(
    "intention_id" UUID NOT NULL,
    "completion_timestamp" TIMESTAMP(0) WITH
        TIME zone NOT NULL,
        "chronicle_id" UUID NOT NULL
);
ALTER TABLE
    "completed_intention" ADD PRIMARY KEY("intention_id");
CREATE TABLE "frequency"(
    "id" UUID NOT NULL,
    "unit" VARCHAR(255) CHECK
        ("unit" IN('')) NOT NULL
);
CREATE INDEX "frequency_unit_index" ON
    "frequency"("unit");
ALTER TABLE
    "frequency" ADD PRIMARY KEY("id");
ALTER TABLE
    "frequency" ADD CONSTRAINT "frequency_unit_unique" UNIQUE("unit");
CREATE TABLE "intention_frequency"(
    "intention_id" UUID NOT NULL,
    "frequency_id" UUID NOT NULL
);
ALTER TABLE
    "intention_frequency" ADD PRIMARY KEY("intention_id", "frequency_id");
ALTER TABLE
    "chronicle" ADD CONSTRAINT "chronicle_person_id_foreign" FOREIGN KEY("person_id") REFERENCES "person"("id");
ALTER TABLE
    "intention" ADD CONSTRAINT "intention_action_id_foreign" FOREIGN KEY("action_id") REFERENCES "action"("id");
ALTER TABLE
    "person_conviction" ADD CONSTRAINT "person_conviction_person_id_foreign" FOREIGN KEY("person_id") REFERENCES "person"("id");
ALTER TABLE
    "person_conviction" ADD CONSTRAINT "person_conviction_conviction_id_foreign" FOREIGN KEY("conviction_id") REFERENCES "conviction"("id");
ALTER TABLE
    "person_action" ADD CONSTRAINT "person_action_action_id_foreign" FOREIGN KEY("action_id") REFERENCES "action"("id");
ALTER TABLE
    "person_action" ADD CONSTRAINT "person_action_person_id_foreign" FOREIGN KEY("person_id") REFERENCES "person"("id");
ALTER TABLE
    "conviction_objective" ADD CONSTRAINT "conviction_objective_conviction_id_foreign" FOREIGN KEY("conviction_id") REFERENCES "conviction"("id");
ALTER TABLE
    "conviction_objective" ADD CONSTRAINT "conviction_objective_objective_id_foreign" FOREIGN KEY("objective_id") REFERENCES "objective"("id");
ALTER TABLE
    "achievement" ADD CONSTRAINT "achievement_objective_id_foreign" FOREIGN KEY("objective_id") REFERENCES "objective"("id");
ALTER TABLE
    "objective_action" ADD CONSTRAINT "objective_action_objective_id_foreign" FOREIGN KEY("objective_id") REFERENCES "objective"("id");
ALTER TABLE
    "objective_action" ADD CONSTRAINT "objective_action_action_id_foreign" FOREIGN KEY("action_id") REFERENCES "action"("id");
ALTER TABLE
    "completed_action" ADD CONSTRAINT "completed_action_person_id_foreign" FOREIGN KEY("person_id") REFERENCES "person"("id");
ALTER TABLE
    "intention" ADD CONSTRAINT "intention_id_foreign" FOREIGN KEY("id") REFERENCES "completed_intention"("intention_id");
ALTER TABLE
    "intention_frequency" ADD CONSTRAINT "intention_frequency_frequency_id_foreign" FOREIGN KEY("frequency_id") REFERENCES "frequency"("id");