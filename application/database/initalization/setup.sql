-- Enums
\i database/enum/frequency.sql
\i database/enum/weekday.sql

-- Entity Tables
\i database/table/person.sql
\i database/table/chronicle.sql
\i database/table/action.sql
\i database/table/intention.sql
\i database/table/goal.sql
\i database/table/email_address.sql
\i database/table/principle.sql
\i database/table/sleep_schedule.sql
\i database/table/skill.sql

-- Relation Tables
\i database/table/person_action.sql
\i database/table/person_goal.sql
\i database/table/person_email_address.sql
\i database/table/person_principle.sql
\i database/table/person_sleep_schedule.sql
\i database/table/person_skill.sql
\i database/table/completed_action.sql
\i database/table/intention_frequency.sql
\i database/table/chronicle_intention.sql

-- Relations

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

