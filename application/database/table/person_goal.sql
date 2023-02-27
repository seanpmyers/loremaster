CREATE TABLE "person_goal" (
    "person_id" uuid NOT NULL,
    "goal_id" uuid NOT NULL
);

ALTER TABLE "person_goal"
    ADD PRIMARY KEY ("person_id", "goal_id");
