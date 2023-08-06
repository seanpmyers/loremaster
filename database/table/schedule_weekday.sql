CREATE TABLE
    "schedule_weekday"(
        "id" uuid NOT NULL UNIQUE PRIMARY KEY,
        "sunday" boolean NOT NULL DEFAULT TRUE,
        "monday" boolean NOT NULL DEFAULT TRUE,
        "tuesday" boolean NOT NULL DEFAULT TRUE,
        "wednesday" boolean NOT NULL DEFAULT TRUE,
        "thursday" boolean NOT NULL DEFAULT TRUE,
        "friday" boolean NOT NULL DEFAULT TRUE,
        "saturday" boolean NOT NULL DEFAULT TRUE
    );
