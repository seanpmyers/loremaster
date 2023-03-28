CREATE TABLE
    "quick_task"(
        "id" uuid NOT NULL UNIQUE PRIMARY KEY,
        "title" text NOT NULL,
        "description" text NULL,
        "completed" boolean NOT NULL DEFAULT FALSE
    );
