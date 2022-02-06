CREATE TABLE "person"(
    "id" UUID NOT NULL,
    "email_address" TEXT NOT NULL,
    "encrypted_password" TEXT NOT NULL,
    "registration_date" TIMESTAMP(0) WITH TIME zone NOT NULL,
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
    "creation_time" TIME(0) WITH TIME zone NULL
);
ALTER TABLE
    "chronicle" ADD PRIMARY KEY("id");
ALTER TABLE
    "chronicle" ADD PRIMARY KEY("person_id");
ALTER TABLE
    "chronicle" ADD PRIMARY KEY("date_recorded");
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
    "intended_time" TIME(0) WITH TIME zone NULL,
    "chronicle_id" UUID NULL
);
ALTER TABLE
    "intention" ADD PRIMARY KEY("id");
ALTER TABLE
    "intention" ADD PRIMARY KEY("action_id");
ALTER TABLE
    "intention" ADD PRIMARY KEY("person_id");
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
    "person_conviction" ADD PRIMARY KEY("person_id");
ALTER TABLE
    "person_conviction" ADD PRIMARY KEY("conviction_id");
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
    "person_action" ADD PRIMARY KEY("action_id");
ALTER TABLE
    "person_action" ADD PRIMARY KEY("person_id");
CREATE TABLE "achievement"(
    "id" UUID NOT NULL,
    "objective_id" UUID NOT NULL,
    "achieved_time" TIME(0) WITH TIME zone NULL,
    "chronicle_id" UUID NULL
);
ALTER TABLE
    "achievement" ADD PRIMARY KEY("id");
CREATE TABLE "conviction_objective"(
    "conviction_id" UUID NOT NULL,
    "objective_id" UUID NOT NULL
);
ALTER TABLE
    "conviction_objective" ADD PRIMARY KEY("conviction_id");
ALTER TABLE
    "conviction_objective" ADD PRIMARY KEY("objective_id");
CREATE TABLE "objective_action"(
    "objective_id" UUID NOT NULL,
    "action_id" UUID NOT NULL,
    "required" BOOLEAN NOT NULL,
    "minimum_count" INTEGER NOT NULL,
    "completion_time" TIME(0) WITH TIME zone NULL,
    "chronicle_id" UUID NULL
);
ALTER TABLE
    "objective_action" ADD PRIMARY KEY("objective_id");
ALTER TABLE
    "objective_action" ADD PRIMARY KEY("action_id");
CREATE TABLE "completed_action"(
    "action_id" UUID NOT NULL,
    "completion_time" TIME(0) WITH TIME zone NOT NULL,
    "person_id" UUID NOT NULL,
    "chronicle_id" UUID NOT NULL
);
ALTER TABLE
    "completed_action" ADD PRIMARY KEY("action_id");
ALTER TABLE
    "completed_action" ADD PRIMARY KEY("person_id");
ALTER TABLE
    "completed_action" ADD PRIMARY KEY("chronicle_id");
CREATE TABLE "completed_intention"(
    "intention_id" UUID NOT NULL,
    "completion_timestamp" TIME(0) WITH TIME zone NOT NULL,
    "chronicle_id" UUID NOT NULL
);
ALTER TABLE
    "completed_intention" ADD PRIMARY KEY("intention_id");
ALTER TABLE
    "completed_intention" ADD PRIMARY KEY("chronicle_id");
ALTER TABLE
    "intention" ADD CONSTRAINT "intention_chronicle_id_foreign" FOREIGN KEY("chronicle_id") REFERENCES "chronicle"("id");
ALTER TABLE
    "achievement" ADD CONSTRAINT "achievement_objective_id_foreign" FOREIGN KEY("objective_id") REFERENCES "objective"("id");
ALTER TABLE
    "achievement" ADD CONSTRAINT "achievement_chronicle_id_foreign" FOREIGN KEY("chronicle_id") REFERENCES "chronicle"("id");
ALTER TABLE
    "objective_action" ADD CONSTRAINT "objective_action_chronicle_id_foreign" FOREIGN KEY("chronicle_id") REFERENCES "chronicle"("id");