-- Enums

\i database/enum/weekday.sql

-- Entity Tables
\i database/table/email_address.sql
\i database/table/person.sql
\i database/table/chronicle.sql
\i database/table/action.sql
\i database/table/intention.sql
\i database/table/goal.sql
\i database/table/principle.sql
\i database/table/sleep_schedule.sql
\i database/table/skill.sql
\i database/table/dictionary.sql
\i database/table/term.sql
\i database/table/schedule.sql
\i database/table/web_authentication_challenge.sql
\i database/table/password.sql

-- Relation Tables
\i database/table/person_action.sql
\i database/table/person_goal.sql
\i database/table/person_email_address.sql
\i database/table/person_principle.sql
\i database/table/person_sleep_schedule.sql
\i database/table/person_skill.sql
\i database/table/dictionary_term.sql
\i database/table/schedule_weekday.sql
\i database/table/intention_instance.sql
\i database/table/person_intention_schedule.sql
\i database/table/person_dictionary.sql
\i database/table/person_password.sql


-- Relations

ALTER TABLE "person"
ADD
    CONSTRAINT "chronicle_foreign_key" FOREIGN KEY ("chronicle_id") REFERENCES "chronicle" ("id");
