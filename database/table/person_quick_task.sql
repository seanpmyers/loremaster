CREATE TABLE
    "person_quick_task"(
        "person_id" uuid NOT NULL REFERENCES "person" ("id"),
        "quick_task_id" uuid NOT NULL REFERENCES "quick_task" ("id"),
        PRIMARY KEY ("person_id", "quick_task_id")
    );
